// Prevents additional console window on Windows in release, DO NOT REMOVE!!

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use log_analysis::{
    zeek::zeek_search_params::ZeekSearchParamsBuilder,     
    zeek::zeek_log::ZeekLog,
    types::types::LogTree,
    types::helpers::print_type_of,
};

//use serde::{Deserialize, Serialize};
// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn zeek_search() -> String 
{
    let params = ZeekSearchParamsBuilder::default()
        .path_prefix("~/dev/log-analysis/zeek-test-logs")
        .start_date("2024-07-02")
        .src_ip("43.134.231.178")
        .build()
        .unwrap();
    dbg!(&params);

    let mut log = ZeekLog::new();
    let res = log.search(&params);
    println!("{:?}", log.data.keys());
    if let Ok(res) = serde_json::to_string(&log.data)
    {
        return res
    }
    String::from("")
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet,zeek_search])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
