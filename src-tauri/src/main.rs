// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]


use tauri_plugin_window_state::Builder as windowStatePlugin;
mod window;
use window::get_window;
use std::fs;
use tauri::{Manager,State,Window};

use std::sync::OnceLock;
static WINDOW: OnceLock<Window> = OnceLock::new();

#[derive(Clone, serde::Serialize)]
struct Payload {
  message: String
}

#[tauri::command]
fn load_workflow()->Vec<String> {
    let wf_dir = ".\\workflows";
    fs::create_dir_all(wf_dir).expect("failed to create");
    let paths = fs::read_dir(wf_dir)
        .unwrap()
        .map(|entry| {
            let entry = entry.unwrap();          
            let entry_path = entry.path();          
            let file_name = entry_path.file_name().unwrap();          
            let file_name_as_str = file_name.to_str().unwrap();          
            let file_name_as_string = String::from(file_name_as_str);          
            file_name_as_string
          })
        .collect::<Vec<_>>();
    paths

}

#[tauri::command]
fn apply_workflow(data:&str) {
    let fpath = format!(".\\workflows\\{}",data);
    println!("{}",fpath);
    let contents = fs::read_to_string(fpath).expect("Should have been able to read the file");

        WINDOW.get().expect("window avail").emit("msg", Payload{message:contents}).expect("failed");

    
    

}

fn launch_ui(){
   
    let ui = tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![load_workflow,apply_workflow])
    .plugin(windowStatePlugin::default().build())
    .setup(|app| {
        
        let mwindow = get_window(app);
        mwindow.show().unwrap();
        _ = WINDOW.set(mwindow);
      
      
        

       
        Ok(())
    });

    ui.run(tauri::generate_context!())
    .expect("error while running tauri application");
   

}

fn main() {


    launch_ui();

    

   
}
