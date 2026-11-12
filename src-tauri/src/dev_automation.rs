//! Dev-only automation hook for the real desktop app.
//!
//! Off by default: with none of the environment variables below set, the spec
//! command answers `null`, the event command does nothing and no watchdog runs, so
//! normal use (and any release build) behaves as if this module did not exist.
//!
//! * `TERRAIN_LAB_AUTOMATION` - a JSON document, or the path of a file holding one,
//!   describing what the frontend should do on start (seed, landform, view, camera,
//!   roam mode, time of day). The Rust side only transports it; the frontend owns
//!   the schema (`src/devAutomation.ts`).
//! * `TERRAIN_LAB_LOG` - file that receives one JSON line per event the frontend
//!   reports (`generated`, `ready`, errors, console output).
//! * `TERRAIN_LAB_READY_FILE` - file written (with the `ready` event's detail) when
//!   the requested view has drawn its first settled frames.
//! * `TERRAIN_LAB_EXIT_AFTER_SECS` - hard watchdog: the process exits after this long.
//!
//! See `docs/real-app-verification.md`.

use std::{io::Write, path::PathBuf, time::Duration};

use tauri::AppHandle;

const SPEC: &str = "TERRAIN_LAB_AUTOMATION";
const LOG: &str = "TERRAIN_LAB_LOG";
const READY: &str = "TERRAIN_LAB_READY_FILE";
const EXIT_AFTER: &str = "TERRAIN_LAB_EXIT_AFTER_SECS";

/// The spec document, from inline JSON or from a file. `None` when automation is off.
pub fn read_spec(value: Option<&str>) -> Result<Option<serde_json::Value>, String> {
    let Some(raw) = value.map(str::trim).filter(|raw| !raw.is_empty()) else {
        return Ok(None);
    };
    let text = if raw.starts_with('{') {
        raw.to_owned()
    } else {
        std::fs::read_to_string(raw).map_err(|error| format!("{SPEC} file {raw}: {error}"))?
    };
    serde_json::from_str(&text)
        .map(Some)
        .map_err(|error| format!("{SPEC} is not valid JSON: {error}"))
}

/// One log line for an event, newline-terminated.
pub fn event_line(event: &str, detail: &serde_json::Value) -> String {
    let at_ms = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|elapsed| elapsed.as_millis())
        .unwrap_or(0);
    format!(
        "{}\n",
        serde_json::json!({ "atMs": at_ms, "event": event, "detail": detail })
    )
}

fn env_path(name: &str) -> Option<PathBuf> {
    std::env::var_os(name)
        .filter(|value| !value.is_empty())
        .map(PathBuf::from)
}

#[tauri::command]
pub fn automation_spec() -> Result<Option<serde_json::Value>, String> {
    read_spec(std::env::var(SPEC).ok().as_deref())
}

/// Frontend report. Appends to the log; `ready` also writes the ready file; `exit`
/// closes the app. A no-op unless the matching variable is set.
#[tauri::command]
pub fn automation_event(app: AppHandle, event: String, detail: serde_json::Value) {
    if let Some(path) = env_path(LOG) {
        if let Ok(mut file) = std::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(path)
        {
            let _ = file.write_all(event_line(&event, &detail).as_bytes());
        }
    }
    if event == "ready" {
        if let Some(path) = env_path(READY) {
            let _ = std::fs::write(path, event_line(&event, &detail));
        }
    }
    if event == "exit" && std::env::var_os(SPEC).is_some() {
        app.exit(0);
    }
}

/// Start the watchdog thread when `TERRAIN_LAB_EXIT_AFTER_SECS` is set.
pub fn start_watchdog(app: AppHandle) {
    let Some(seconds) = std::env::var(EXIT_AFTER)
        .ok()
        .and_then(|value| value.trim().parse::<u64>().ok())
    else {
        return;
    };
    std::thread::spawn(move || {
        std::thread::sleep(Duration::from_secs(seconds));
        app.exit(2);
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn off_when_unset_or_blank() {
        assert_eq!(read_spec(None).unwrap(), None);
        assert_eq!(read_spec(Some("  ")).unwrap(), None);
    }

    #[test]
    fn inline_json_is_parsed() {
        let spec = read_spec(Some(r#"{"view":"city","city":0}"#))
            .unwrap()
            .unwrap();
        assert_eq!(spec["view"], "city");
    }

    #[test]
    fn file_is_read_and_bad_input_is_an_error() {
        let path = std::env::temp_dir().join("terrain-lab-automation-test.json");
        std::fs::write(&path, r#"{"seed":7}"#).unwrap();
        let spec = read_spec(path.to_str()).unwrap().unwrap();
        assert_eq!(spec["seed"], 7);
        let _ = std::fs::remove_file(&path);
        assert!(read_spec(Some("/definitely/not/here.json")).is_err());
        assert!(read_spec(Some("{not json")).is_err());
    }

    #[test]
    fn event_lines_are_json() {
        let line = event_line("ready", &serde_json::json!({"frames": 3}));
        assert!(line.ends_with('\n'));
        let parsed: serde_json::Value = serde_json::from_str(line.trim()).unwrap();
        assert_eq!(parsed["event"], "ready");
        assert_eq!(parsed["detail"]["frames"], 3);
    }
}
