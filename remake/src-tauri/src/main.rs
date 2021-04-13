#![cfg_attr(all(not(debug_assertions), target_os = "windows"), windows_subsystem = "windows")]

mod cmd;

fn main() {
    tauri::AppBuilder::new()
        .invoke_handler(|_webview, arg| {
            use cmd::Cmd::*;
            serde_json::from_str(arg)
                .map(|command| match command {
                    MyCustomCommand { argument } => println!("{}", argument),
                })
                .map_err(|e| e.to_string())
        })
        .build()
        .run();
}
