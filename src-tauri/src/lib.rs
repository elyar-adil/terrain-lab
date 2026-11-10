mod dev_automation;

use app_backend::ProjectDocument;
pub use app_backend::{GenerationResult, build_generation_result, build_payload};
use serde::Serialize;
use std::path::PathBuf;
use tauri::{AppHandle, Emitter};
use terrain_core::{SimulationConfig, save_png};

// The backend itself (terrain, infrastructure, the city cache, traffic) is the
// `app-backend` crate, shared with the browser build. This file is only the
// desktop transport: Tauri commands, the progress event and the file paths.

#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
struct ProgressEvent {
    stage: String,
    progress: f32,
}

fn emit_progress(app: &AppHandle, progress: f32, stage: &str) {
    let _ = app.emit(
        "terrain-progress",
        ProgressEvent {
            stage: stage.to_owned(),
            progress,
        },
    );
}

#[tauri::command]
async fn generate_terrain(
    app: AppHandle,
    config: SimulationConfig,
) -> Result<GenerationResult, String> {
    tauri::async_runtime::spawn_blocking(move || {
        // Only the city *plans* are kept. A scene is tens of megabytes of vertex
        // buffers, textures and a traffic fleet, so it is built the first time
        // `city_scene` is asked for that city, and not returned here: a generate
        // response carrying every scene was hundreds of megabytes of JSON that
        // killed the webview on arrival.
        app_backend::generate_world(config, &|progress, stage| {
            emit_progress(&app, progress, stage)
        })
    })
    .await
    .map_err(|error| format!("native generation task failed: {error}"))?
}

/// One city's finished scene (JSON form; the app uses `city_scene_bin`).
#[tauri::command]
async fn city_scene(index: usize) -> Result<city_scene::CityScene, String> {
    tauri::async_runtime::spawn_blocking(move || {
        app_backend::ensure_city_scene(index).map(|scene| (*scene).clone())
    })
    .await
    .map_err(|error| format!("building the city scene failed: {error}"))?
}

/// One square tile of a city's detail, in the binary container.
#[tauri::command]
async fn city_tile_bin(
    index: usize,
    column: i32,
    row: i32,
    with_textures: bool,
) -> Result<tauri::ipc::Response, String> {
    let bytes = tauri::async_runtime::spawn_blocking(move || {
        app_backend::city_tile_bytes(index, column, row, with_textures)
    })
    .await
    .map_err(|error| format!("building the city tile failed: {error}"))??;
    Ok(tauri::ipc::Response::new(bytes))
}

/// The same scene as `city_scene`, as the binary container sent as a raw
/// `ArrayBuffer`: no base64, no JSON of a 100 MB string, and the webview can view
/// the buffers in place.
#[tauri::command]
async fn city_scene_bin(index: usize) -> Result<tauri::ipc::Response, String> {
    let bytes = tauri::async_runtime::spawn_blocking(move || app_backend::city_scene_bytes(index))
        .await
        .map_err(|error| format!("building the city scene failed: {error}"))??;
    Ok(tauri::ipc::Response::new(bytes))
}

#[tauri::command]
async fn export_terrain(
    app: AppHandle,
    config: SimulationConfig,
    output_path: String,
    output_size: usize,
) -> Result<String, String> {
    tauri::async_runtime::spawn_blocking(move || {
        let image = app_backend::render_export(&config, output_size, &|progress, stage| {
            emit_progress(&app, progress, stage)
        })?;
        let path = PathBuf::from(&output_path);
        save_png(&image, &path).map_err(|error| error.to_string())?;
        Ok(path.to_string_lossy().into_owned())
    })
    .await
    .map_err(|error| format!("native export task failed: {error}"))?
}

#[tauri::command]
async fn save_project(project: ProjectDocument, output_path: String) -> Result<String, String> {
    let serialized = app_backend::project_to_bytes(&project)?;
    let path = PathBuf::from(&output_path);
    std::fs::write(&path, serialized).map_err(|error| error.to_string())?;
    Ok(path.to_string_lossy().into_owned())
}

#[tauri::command]
async fn load_project(input_path: String) -> Result<ProjectDocument, String> {
    let bytes = std::fs::read(input_path).map_err(|error| error.to_string())?;
    app_backend::project_from_bytes(&bytes)
}

/// Advance one city's traffic by `dt` seconds and return the new poses and
/// signal aspects. The renderer calls this a few times a second rather than every
/// frame; 20 Hz with client-side interpolation is indistinguishable at 60 fps.
#[tauri::command]
fn traffic_step(city: usize, dt: f32) -> Result<serde_json::Value, String> {
    app_backend::traffic_step(city, dt)
}

/// Total vehicle count per city, for the renderer's capacity check.
#[tauri::command]
fn traffic_capacities() -> Vec<usize> {
    app_backend::traffic_capacities()
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .setup(|app| {
            dev_automation::start_watchdog(app.handle().clone());
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            dev_automation::automation_spec,
            dev_automation::automation_event,
            generate_terrain,
            city_scene,
            city_scene_bin,
            city_tile_bin,
            export_terrain,
            save_project,
            load_project,
            traffic_step,
            traffic_capacities
        ])
        .run(tauri::generate_context!())
        .expect("error while running wind & water terrain lab");
}
