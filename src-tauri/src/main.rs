// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use tauri_plugin_window_state::Builder as windowStatePlugin;
mod window;
use std::fs;
use std::fs::File;
use std::io::Write;
use rand::Rng;
use window::get_window;
use std::sync::OnceLock;
use tauri::Window;
static WINDOW: OnceLock<Window> = OnceLock::new();

#[derive(Clone, serde::Serialize)]
struct Payload {
    message: String,
}

#[tauri::command]
fn load_workflow() -> Vec<String> {
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

fn generate_random(oldname:&str)->String{
    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                            abcdefghijklmnopqrstuvwxyz\
                            0123456789";
    const PASSWORD_LEN: usize = 5;
    let mut rng = rand::thread_rng();

    let password: String = (0..PASSWORD_LEN)
        .map(|_| {
            let idx = rng.gen_range(0..CHARSET.len());
            CHARSET[idx] as char
        })
        .collect();
    format!("{}_{}.json",oldname,password)
}

#[tauri::command]
fn apply_workflow(data: &str) {
    let fpath = format!(".\\workflows\\{}", data);
    println!("{}", fpath);
    let contents = match fs::read_to_string(fpath){
        Ok(ff)=>ff,
        Err(error)=>String::from("Not Foundd"),
    };
    WINDOW
        .get()
        .expect("window avail")
        .emit("msg", Payload { message: contents })
        .expect("failed");
}

#[tauri::command]
fn save_workflow(data: &str,filename:&str) ->String{
    let _filename =  generate_random(&filename);
    let fpath = format!(".\\workflows\\{}",&_filename);    
    let mut file = File::create(fpath).unwrap();
    file.write(data.as_bytes()).expect("error writing file");      
    String::from(_filename)
    
}
#[tauri::command]
fn delete_workflow(filename:&str) ->String{
    let filepath = format!(".\\workflows\\{}", filename);
    let result = match fs::remove_file(filepath){
        Ok(())=>"ok",
        Err(e)=>"error"
    };
    String::from(result)
    
}

fn launch_ui() {
    let ui = tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![load_workflow, apply_workflow,save_workflow,delete_workflow])
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
