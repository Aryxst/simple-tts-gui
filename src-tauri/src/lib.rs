// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
mod commands;
use commands::fs_extras::list_files;
use commands::tts;
use tauri::Manager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_single_instance::init(|app, _args, _cwd| {
            let _ = app
                .get_webview_window("main")
                .expect("No main window")
                .set_focus();
        }))
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![tts::synthesize, list_files])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
