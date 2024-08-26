// Prevents additional console window on Windows in release, DO NOT REMOVE!!

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use log_analysis::{
    zeek::zeek_search_params::ZeekSearchParams,
    zeek::zeek_search_params::ZeekSearchParamsBuilder,     
    zeek::zeek_log_proto::ZeekProtocol,
    zeek::zeek_log::ZeekLog,
    types::types::LogTree,
    types::helpers::print_type_of,
};
mod ip2location;

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
    proto: Option<&'q str>,
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

    fn get_proto(&self) -> Option<&'q str>
    {
        self.proto
    }
}

#[tauri::command]
fn zeek_search(query: String) -> String 
{
    // date = 2024-07-02, ip = 43.134.231.178, proto = conn
    let search : Result<SearchQuery, serde_json::Error> = serde_json::from_str(&query);
    if let Ok(result) = search 
    {
        let params = ZeekSearchParamsBuilder::default()
            .path_prefix("~/dev/log-analysis/zeek-test-logs")
            .start_date(result.get_date())
            .src_ip(result.get_ip())
            .proto_type(result.get_proto())
            .build()
            .unwrap();
        dbg!(&params);

        let mut log = ZeekLog::new();
        let res = log.search(&params);
        println!("{:?}", log.data);
        return String::from("gathering data...")
    }
    String::from("")
}

fn main() {
    println!("{}",ip2location::get_info());
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet,zeek_search])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
