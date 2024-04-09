use tauri::{App, Manager};
use webkit2gtk::glib::{gobject_ffi, ObjectExt};

// https://github.com/tauri-apps/tauri/discussions/3843
// https://docs.rs/tauri/latest/tauri/window/struct.Window.html

pub fn fix_pinch_zoom(app: &mut App) -> Result<(), Box<dyn std::error::Error>> {
    let main_window = app.get_window("main").unwrap();
    main_window.with_webview(|webview| {
        #[cfg(target_os = "linux")]
        {
            unsafe {
                if let Some(data) = webview.inner().data::<>("wk-view-zoom-gesture") {
                    gobject_ffi::g_signal_handlers_destroy(data.as_ptr());
                }
            }
        }
    }).expect("Something went wrong disabling pinch zoom.");

    Ok(())
}