// Prevents additional console window on Windows in release, DO NOT REMOVE!!

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use log_analysis::{
    zeek::zeek_search_params::ZeekSearchParams,
    zeek::zeek_search_params::ZeekSearchParamsBuilder,     
    zeek::zeek_log::ZeekLog,
    types::types::LogTree,
    types::helpers::print_type_of,
};

use serde::{Deserialize, Serialize};

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}
#[derive(Serialize, Deserialize, Debug, Clone)]
struct SearchQuery<'q> {
    ip: Option<&'q str>,
    date: Option<&'q str>,
}
impl<'q> SearchQuery<'q>
{
    fn get_ip(&self) -> Option<&'q str>
    {
        self.ip
    }

    fn get_date(&self) -> Option<&'q str>
    {
        self.date
    }
}

#[tauri::command]
fn zeek_search(query: String) -> String 
{
    let search : Result<SearchQuery, serde_json::Error> = serde_json::from_str(&query);
    if let Ok(result) = search 
    {
        let params = ZeekSearchParamsBuilder::default()
            .path_prefix("~/dev/log-analysis/zeek-test-logs")
            .start_date(result.get_date())
            .src_ip(result.get_ip()) // "43.134.231.178" exists
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
    }
    String::from("")
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet,zeek_search])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
