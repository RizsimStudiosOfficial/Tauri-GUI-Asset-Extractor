use std::fs;
use std::path::PathBuf;
use std::process::Command;

use raw_window_handle::HasRawWindowHandle;
use tauri::Manager;

const DUMPER_BYTES: &[u8] = include_bytes!("../binaries/tauri-dumper-x86_64-pc-windows-msvc.exe");

fn prepare_dumper_exe() -> Result<PathBuf, String> {
    let temp_exe = std::env::temp_dir().join("tauri-dumper-embedded.exe");
    let needs_write = match fs::metadata(&temp_exe) {
        Ok(meta) => meta.len() != DUMPER_BYTES.len() as u64,
        Err(_) => true,
    };

    if needs_write {
        fs::write(&temp_exe, DUMPER_BYTES).map_err(|e| e.to_string())?;
    }

    Ok(temp_exe)
}

#[cfg(target_os = "windows")]
fn apply_win32_icon(window: &tauri::WebviewWindow) {
    use raw_window_handle::{HasWindowHandle, RawWindowHandle};
    use windows_sys::Win32::Foundation::HWND;
    use windows_sys::Win32::UI::WindowsAndMessaging::{SendMessageW, WM_SETICON};

    if let Ok(handle) = window.window_handle() {
        if let RawWindowHandle::Win32(win32) = handle.as_raw() {
            let hwnd: HWND = win32.hwnd.get() as HWND;
            let _ = (hwnd, SendMessageW as usize, WM_SETICON);
        }
    }
}

#[tauri::command]
async fn extract_assets(file_path: String, out_dir: String) -> Result<String, String> {
    let temp_exe = prepare_dumper_exe()?;

    let mut cmd = Command::new(&temp_exe);
    cmd.args(["extract", &file_path, "-o", &out_dir]);

    #[cfg(windows)]
    {
        use std::os::windows::process::CommandExt;
        const CREATE_NO_WINDOW: u32 = 0x08000000;
        cmd.creation_flags(CREATE_NO_WINDOW);
    }

    let output = cmd.output().map_err(|e| e.to_string())?;

    if output.status.success() {
        Ok(format!("Done! Extracted to {}", out_dir))
    } else {
        Err(String::from_utf8_lossy(&output.stderr).to_string())
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            #[cfg(target_os = "windows")]
            if let Some(window) = app.get_webview_window("main") {
                apply_win32_icon(&window);
            }

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![extract_assets])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
