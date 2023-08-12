// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use reqwest::get;
use serde_json::Value;
use std::{thread, time};
use tauri::Window;
use std::env::current_dir;
use std::fs::{File, write, read_dir, read_to_string};
use std::env;
use std::fs;

const id: &str = "a62b666d37039f2888f2e7c540f3afad";

use tauri::{Manager};

// the payload type must implement `Serialize` and `Clone`.
#[derive(Clone, serde::Serialize)]
struct Payload {
}

// init a background process on the command, and emit periodic events only to the window that used the command
#[tauri::command]
fn init_process(window: Window) {
  std::thread::spawn(move || {
    loop {
      window.emit("event-name", Payload { }).unwrap();
      thread::sleep(time::Duration::from_millis(60000));
    }
  });
}

#[tauri::command]
async fn get_temperature(latitude: f64, longitude: f64) -> String{
    //https://api.openweathermap.org/data/2.5/weather?lat={lat}&lon={lon}&appid={API key}
    let body = reqwest::get(
        format!("{}{}{}{}{}{}{}", 
        "https://api.openweathermap.org/data/2.5/weather?lat=",
        latitude,
        "&lon=",
        longitude,
        "&appid=",
        id,
        "&units=metric"
    ))
    .await.expect("REASON")
    .text()
    .await.unwrap();

    
    let v: Value = serde_json::from_str(&body).unwrap();
    //println!("{}", v);
    let temp: String = v["main"]["temp"].to_string();

    return temp;
}

#[tauri::command]
async fn get_coord_by_city(city: String)->(f64, f64){
    //http://api.openweathermap.org/geo/1.0/direct?q={city name},{state code},{country code}&limit={limit}&appid={API key}
    let body = reqwest::get(
        format!("{}{}{}{}", 
        "http://api.openweathermap.org/geo/1.0/direct?q=",
        city,
        ",642&limit=5&appid=",
        id
    ))
    .await.expect("REASON")
    .text()
    .await.unwrap();
    
    let v: Value = serde_json::from_str(&body).unwrap();

    //println!("{}", v[0]["lat"]);
    //println!("{}", v[0]["lon"]);

    let x = v[0]["lat"].to_string().parse::<f64>().unwrap();
    let y = v[0]["lon"].to_string().parse::<f64>().unwrap();

    let t : (f64, f64)= (x, y);

    return t;
}

#[tauri::command]
async fn get_city_by_coord(latitude: f64, longitude: f64)->String{
    //https://api.openweathermap.org/data/2.5/weather?lat={lat}&lon={lon}&appid={API key}
    let body = reqwest::get(
        format!("{}{}{}{}{}{}{}", 
        "https://api.openweathermap.org/data/2.5/weather?lat=",
        latitude,
        "&lon=",
        longitude,
        "&appid=",
        id,
        "&units=metric"
    ))
    .await.expect("REASON")
    .text()
    .await.unwrap();

    
    let v: Value = serde_json::from_str(&body).unwrap();
   
    let temp: String = v["name"].to_string().replace("\"", "");
    //println!("{}", temp);
    return temp;
}

#[tauri::command]
async fn get_icon(latitude: f64, longitude: f64)->String{
   //https://api.openweathermap.org/data/2.5/weather?lat={lat}&lon={lon}&appid={API key}
   let body = reqwest::get(
    format!("{}{}{}{}{}{}{}", 
    "https://api.openweathermap.org/data/2.5/weather?lat=",
    latitude,
    "&lon=",
    longitude,
    "&appid=",
    id,
    "&units=metric"
))
    .await.expect("REASON")
    .text()
    .await.unwrap();
    
    let v: Value = serde_json::from_str(&body).unwrap();

    let x = v["weather"][0]["icon"].to_string().replace("\"", "");
    //println!("{}", x);
    let y = format!("{}{}{}", "https://openweathermap.org/img/wn/", x, "@2x.png");
    //println!("{}", y);
    return y;
}

#[tauri::command]
fn writetofile(text: String) ->Result<(), ()> {
    let _ = fs::write("favoriteCities.txt", text);
    println!("Succesfully written");
    return Ok(())
}

#[tauri::command]
fn getcontent() ->String
{
    //println!("{}", fs::read_to_string(name.clone()).expect("REASON"));
    return fs::read_to_string("favoriteCities.txt").expect("REASON")
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            get_temperature,
            get_coord_by_city,
            get_icon,
            get_city_by_coord,
            writetofile,
            getcontent,
            init_process
            ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
