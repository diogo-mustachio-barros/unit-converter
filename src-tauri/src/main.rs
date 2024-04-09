// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod units;
mod converter;
mod util;

mod commands;
mod menu;
mod zoom_fix;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![ commands::get_unit_types
                                                , commands::get_units
                                                , commands::convert
                                                ])
        .menu(menu::create_menu())
        .on_menu_event(menu::handle_menu_event)
        .setup(zoom_fix::fix_pinch_zoom)
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}