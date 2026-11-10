//! The application backend as a WebAssembly module, for the browser build.
//!
//! The desktop shell exposes `app-backend` through Tauri commands; this module
//! exposes the same functions through one plain C-style entry point, so the
//! module needs no JavaScript glue generator and imports only two host functions.
//!
//! ABI. The host copies a UTF-8 JSON request into memory obtained from `alloc`,
//! calls `call(command, ptr, len)`, then reads `result_len()` bytes at
//! `result_ptr()`. A return value of 0 means the bytes are the command's result
//! (JSON, or raw bytes for the binary commands); 1 means they are an error message.
//! The result stays valid until the next call. Everything runs on the calling
//! thread (a Web Worker): rayon finds no threads to spawn and runs inline.

// Only meaningful as a WebAssembly module (it imports its host's functions); a
// host build of the workspace gets an empty crate instead of a link error.
#![cfg(target_arch = "wasm32")]

use app_backend::ProjectDocument;
use serde::Deserialize;
use std::cell::RefCell;
use terrain_core::SimulationConfig;

#[link(wasm_import_module = "env")]
unsafe extern "C" {
    /// Progress of the running command: fraction 0..1 and a stage label (UTF-8).
    fn host_progress(fraction: f32, stage_ptr: *const u8, stage_len: u32);
    /// A diagnostic line (panic messages).
    fn host_log(ptr: *const u8, len: u32);
}

pub const GENERATE: u32 = 0;
pub const CITY_SCENE_BIN: u32 = 1;
pub const CITY_TILE_BIN: u32 = 2;
pub const TRAFFIC_STEP: u32 = 3;
pub const TRAFFIC_CAPACITIES: u32 = 4;
pub const EXPORT_PNG: u32 = 5;
pub const PROJECT_TO_BYTES: u32 = 6;
pub const PROJECT_FROM_BYTES: u32 = 7;
pub const CITY_SCENE_JSON: u32 = 8;

thread_local! {
    static RESULT: RefCell<Vec<u8>> = const { RefCell::new(Vec::new()) };
}

#[unsafe(no_mangle)]
pub extern "C" fn alloc(len: usize) -> *mut u8 {
    let mut buffer = Vec::<u8>::with_capacity(len.max(1));
    let ptr = buffer.as_mut_ptr();
    std::mem::forget(buffer);
    ptr
}

/// # Safety
/// `ptr` and `len` must be a block returned by `alloc` with the same `len`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn dealloc(ptr: *mut u8, len: usize) {
    drop(unsafe { Vec::from_raw_parts(ptr, 0, len.max(1)) });
}

#[unsafe(no_mangle)]
pub extern "C" fn result_ptr() -> *const u8 {
    RESULT.with(|slot| slot.borrow().as_ptr())
}

#[unsafe(no_mangle)]
pub extern "C" fn result_len() -> usize {
    RESULT.with(|slot| slot.borrow().len())
}

/// Release the last result's memory.
#[unsafe(no_mangle)]
pub extern "C" fn result_free() {
    RESULT.with(|slot| *slot.borrow_mut() = Vec::new());
}

/// Install the panic hook that reports through `host_log`. Call once after instantiation.
#[unsafe(no_mangle)]
pub extern "C" fn init() {
    std::panic::set_hook(Box::new(|info| {
        let message = info.to_string();
        unsafe { host_log(message.as_ptr(), message.len() as u32) };
    }));
}

fn progress(fraction: f32, stage: &str) {
    unsafe { host_progress(fraction, stage.as_ptr(), stage.len() as u32) };
}

#[derive(Deserialize)]
struct CityRequest {
    index: usize,
}

#[derive(Deserialize)]
struct TileRequest {
    index: usize,
    column: i32,
    row: i32,
    #[serde(default, rename = "withTextures")]
    with_textures: bool,
}

#[derive(Deserialize)]
struct StepRequest {
    city: usize,
    dt: f32,
}

#[derive(Deserialize)]
struct ExportRequest {
    config: SimulationConfig,
    size: usize,
}

fn parse<'a, T: Deserialize<'a>>(input: &'a [u8]) -> Result<T, String> {
    serde_json::from_slice(input).map_err(|error| format!("bad request: {error}"))
}

fn json<T: serde::Serialize>(value: &T) -> Result<Vec<u8>, String> {
    serde_json::to_vec(value).map_err(|error| error.to_string())
}

fn dispatch(command: u32, input: &[u8]) -> Result<Vec<u8>, String> {
    match command {
        GENERATE => {
            let config: SimulationConfig = parse(input)?;
            let result = app_backend::generate_world(config, &progress)?;
            json(&result)
        }
        CITY_SCENE_BIN => app_backend::city_scene_bytes(parse::<CityRequest>(input)?.index),
        CITY_SCENE_JSON => {
            let scene = app_backend::ensure_city_scene(parse::<CityRequest>(input)?.index)?;
            json(&*scene)
        }
        CITY_TILE_BIN => {
            let tile: TileRequest = parse(input)?;
            app_backend::city_tile_bytes(tile.index, tile.column, tile.row, tile.with_textures)
        }
        TRAFFIC_STEP => {
            let step: StepRequest = parse(input)?;
            json(&app_backend::traffic_step(step.city, step.dt)?)
        }
        TRAFFIC_CAPACITIES => json(&app_backend::traffic_capacities()),
        EXPORT_PNG => {
            let request: ExportRequest = parse(input)?;
            let image = app_backend::render_export(&request.config, request.size, &progress)?;
            let mut png = std::io::Cursor::new(Vec::new());
            image
                .write_to(&mut png, image::ImageFormat::Png)
                .map_err(|error| error.to_string())?;
            Ok(png.into_inner())
        }
        PROJECT_TO_BYTES => app_backend::project_to_bytes(&parse::<ProjectDocument>(input)?),
        PROJECT_FROM_BYTES => json(&app_backend::project_from_bytes(input)?),
        other => Err(format!("unknown command {other}")),
    }
}

/// Run one command. See the module comment for the ABI.
///
/// # Safety
/// `ptr` and `len` must describe a readable block (from `alloc`).
#[unsafe(no_mangle)]
pub unsafe extern "C" fn call(command: u32, ptr: *const u8, len: usize) -> u32 {
    let input = unsafe { std::slice::from_raw_parts(ptr, len) };
    let (status, bytes) = match dispatch(command, input) {
        Ok(bytes) => (0, bytes),
        Err(message) => (1, message.into_bytes()),
    };
    RESULT.with(|slot| *slot.borrow_mut() = bytes);
    status
}
