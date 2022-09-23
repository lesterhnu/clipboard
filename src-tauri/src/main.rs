#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]
use std::env;
use tauri::{
    App,AppHandle, SystemTray, SystemTrayEvent,
    SystemTrayEvent::{LeftClick, MenuItemClick, RightClick},
    Wry, Manager
};
use tauri::{CustomMenuItem, SystemTrayMenu};
use std::fs::File;
use std::path::Path;
mod command;
mod db;
mod utils;

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            set_up(app);
            Ok(())
        })
        .system_tray(system_tray())
        .on_system_tray_event(listen_event)
        .invoke_handler(tauri::generate_handler![
            command::clear_data,
            command::insert_record,
            command::insert_if_not_exist,
            command::batch_get_record,
            command::delete_record,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[allow(unused)]
fn listen_event(app: &AppHandle<Wry>, event: SystemTrayEvent) {
    match event {
        LeftClick { position, size, .. } => {
            if let Some(window)  = app.get_window("main"){
                window.set_position(position);
                window.show();
                window.set_always_on_top(true);
            }
            println!("x:{},y:{}", position.x, position.y);
            println!("w:{},h:{}", size.width, size.height);
        }
        RightClick {
            position: _,
            size: _,
            ..
        } => {
            println!("right click")
        }
        MenuItemClick { id, .. } => match id.as_str() {
            "quit" => {
                std::process::exit(0);
            }
            _ => {}
        },
        _ => todo!(),
    }
}

fn set_up(app: &mut App){
    let path = "../data.sqlite";
    if !Path::new(path).exists(){
        File::create(path).unwrap();
    }
    db::SqliteDB::init();
    if let Some(window) = app.get_window("main"){
        window.hide().unwrap();
    }
}

fn system_tray()->tauri::SystemTray{
    let quit = CustomMenuItem::new("quit".to_string(), "退出");
    let hide = CustomMenuItem::new("hide".to_string(), "隐藏");
    let tray_menu = SystemTrayMenu::new().add_item(quit).add_item(hide);
    SystemTray::new().with_menu(tray_menu)
}