// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use tauri_plugin_window_state::Builder as windowStatePlugin;
mod window;
use window::get_window;
use std::thread;
use std::process::{Command, Stdio};
use std::path::Path;
use std::io::{BufReader, BufRead};
use tauri::{Window};
fn exec_stream<P: AsRef<Path>>(binary: P, args: Vec<&'static str>,window:&Window) {
    let mut isrun = false;
    let mut cmd = Command::new(binary.as_ref())
        .args(&args)
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();

    {
        let stdout = cmd.stdout.as_mut().unwrap();
        let stdout_reader = BufReader::new(stdout);
        let stdout_lines = stdout_reader.lines();

        for line in stdout_lines {
            window.emit("msg", Payload { message: line.unwrap() }).unwrap();
            if !isrun{
               
                
            }
            isrun = true;
           
        }
    }

    cmd.wait().unwrap();
}

#[derive(Clone, serde::Serialize)]
struct Payload {
  message: String,
}


fn launch_ui(){
   
    let ui = tauri::Builder::default()
    .plugin(windowStatePlugin::default().build())
    .setup(|app| {

        let mainwindow = get_window(app);
        // _window.open_devtools();
        mainwindow.show().unwrap();
        // thread::spawn(move ||{
        //     exec_stream("G:\\STABLEDIFF\\COMFYUI\\python_embeded\\python.exe", vec!("-s", "G:\\STABLEDIFF\\COMFYUI\\ComfyUI\\main.py","--windows-standalone-build","--lowvram"),&mainwindow);
        // });
        
        
        Ok(())
    });

    ui.run(tauri::generate_context!())
    .expect("error while running tauri application");
   

}

fn main() {
     
    
  

    launch_ui();

    

   
}
