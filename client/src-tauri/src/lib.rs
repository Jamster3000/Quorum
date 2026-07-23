mod commands;
pub mod common;
pub mod config;

use tauri_plugin_store::Builder as StoreBuilder;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            #[cfg(desktop)]
            app.handle()
            .plugin(
                tauri_plugin_single_instance::init(|_app, _args, _cwd| {})
            )?;

            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .plugin(StoreBuilder::default().build())
        .invoke_handler(tauri::generate_handler![
            commands::auth::signup,
            commands::auth::login,
            commands::auth::refresh_token,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
