use tauri::{App, Window, WindowBuilder, WindowUrl};
use std::include_str;

pub fn get_window(app: &mut App) -> Window {
    let user_agent =  "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/108.0.0.0 Safari/537.36";
     let url = WindowUrl::App("http://localhost:8188/".parse().unwrap());
    
    let window_builder = WindowBuilder::new(app, "CompfyUI", url)
        .title("COMFYUI Viewer")
        .user_agent(user_agent)
        .visible(false) // Prevent initial shaking
        .resizable(true)
        .fullscreen(false)
        .inner_size(800_f64, 600_f64)
        .disable_file_drop_handler()
        .initialization_script(include_str!("./inject/panel.js"));

    window_builder.build().unwrap()
}
