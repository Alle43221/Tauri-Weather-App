// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use reqwest::get;
use serde_json::Value;
use std::{thread, time};
use tauri::Window;

#[tauri::command]
async fn get_temperature(latitude: f64, longitude: f64) {
    
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_temperature])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
