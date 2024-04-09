use tauri::{CustomMenuItem, Menu, Submenu, WindowMenuEvent};

pub fn create_menu() -> Menu {
    let options_menu = 
        Menu::new().add_item(CustomMenuItem::new("zoom_in".to_string(), "Zoom In").accelerator("CommandOrControl+-"))
                   .add_item(CustomMenuItem::new("zoom_out".to_string(), "Zoom Out").accelerator("CommandOrControl+Plus"));

    let menu = Menu::new().add_submenu(Submenu::new("Options", options_menu));
    
    return menu;
}

pub fn handle_menu_event(event: WindowMenuEvent) {
    match event.menu_item_id() {
        "zoom_in" => {
            event.window().emit("zoom_in", {}).unwrap();
        }
        "zoom_out" => {
            event.window().emit("zoom_out", {}).unwrap();
        }
        _ => {}
      }
}