// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use log_analysis::{
    zeek::zeek_search_params::ZeekSearchParamsBuilder,     
    zeek::zeek_log::ZeekLog,
};
// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    let params = ZeekSearchParamsBuilder::default()
        .path_prefix("~/dev/log-analysis/zeek-test-logs")
        .start_date("2024-07-02")
        .src_ip("43.134.231.178")
        .build()
        .unwrap();
    dbg!(&params);
    let mut log = ZeekLog::new();
    let res = log.search(&params);
    let res = res.unwrap();
    format!("Hello, {}! You've been greeted from Rust! {:?}", name, res)
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
