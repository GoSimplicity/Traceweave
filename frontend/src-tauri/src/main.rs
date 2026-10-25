#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands;
mod state;

fn main() {
    tauri::Builder::default()
        .run(tauri::generate_context!())
        .expect("failed to run Traceweave desktop");
}
