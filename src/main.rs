#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use serde::{Deserialize, Serialize};
use wry::application::window::Window;

use crate::utils::{load_workflow, save_workflow, delete_workflow,load_config,Config, apply_workflow};
mod utils;

#[derive(Serialize, Deserialize, Debug)]
struct Data {
    filename: String,
    data: String,
}

#[derive(Serialize, Deserialize)]
struct IPCData {
    key: String,
    data: Data,
}

fn main() -> wry::Result<()> {
    use rfd::FileDialog;
    use std::fs;
    use std::{env::temp_dir, path::PathBuf};
    use wry::{
        application::{
            event::{Event, StartCause, WindowEvent},
            event_loop::{ControlFlow, EventLoopBuilder},
            window::WindowBuilder,
        },
        webview::WebViewBuilder,
    };

    //init
    let conf:Config = load_config().unwrap();
    Config{port:conf.port,workflow_dir:String::from(conf.workflow_dir)}.make_current();

    let wf_dir = String::from(&Config::current().workflow_dir);
  
    fs::create_dir_all(wf_dir).expect("failed to create");

    let event_loop = EventLoopBuilder::<UserEvent>::with_user_event().build();
    let window = WindowBuilder::new()
        .with_decorations(false)
        .build(&event_loop)?;
    let proxy = event_loop.create_proxy();
    enum UserEvent {
        DownloadStarted(String, String),
        DownloadComplete(Option<PathBuf>, bool),
        Rejected(String),
        CloseWindow,
        LoadWorkflow,
        SaveWorkflow(String),
        DeleteWorkflow(String),
        ApplyWorkflow(String),
        AppendWorkflow(String),
    }

    let mut _webview = Some(
        WebViewBuilder::new(window)?
            .with_url(format!("http://127.0.0.1:{}",Config::current().port).as_str())?
            .with_accept_first_mouse(true)
            .with_initialization_script(include_str!("js/panel.js"))
            .with_ipc_handler(move |window: &Window, req: String| match req.as_str() {
                "minimize" => {
                    window.set_minimized(true);
                }
                "maximize" => {
                    window.set_maximized(!window.is_maximized());
                }
                "close" => {
                    let _ = proxy.send_event(UserEvent::CloseWindow);
                }
                "drag_window" => {
                    let _ = window.drag_window();
                }
                _ => {
                    if !req.is_empty() {
                        let i: IPCData = serde_json::from_str(req.as_str()).unwrap();
                      
                        match i.key.as_str() {
                            "save" => {
                                let _ = proxy.send_event(UserEvent::SaveWorkflow(req));                                
                                
                            }
                            "apply" => {
                                let _ = proxy.send_event(UserEvent::ApplyWorkflow(req));       
                            }
                            "append" => {
                                let _ = proxy.send_event(UserEvent::AppendWorkflow(req));       
                            }
                            "delete" => {
                                let _ = proxy.send_event(UserEvent::DeleteWorkflow(req));                                

                            }
                            "load" => {
                                let _ = proxy.send_event(UserEvent::LoadWorkflow);                                

                            }
                            _ => println!("nothing"),
                        }
                    }
                }
            })
            .with_download_started_handler({
                let proxy = event_loop.create_proxy();
                move |uri: String, default_path: &mut PathBuf| {
                    let path = temp_dir().join("workflow.json").as_path().to_path_buf();
                    println!("anama:x{:?}", default_path);
                    *default_path = FileDialog::new()
                        .set_title("Save into ")
                        .set_file_name("workflow.json")
                        .add_filter("JSON file (*.json)", &["*.json", "*.*"])
                        .set_directory(&default_path)
                        .save_file()
                        .unwrap();

                    println!("The user choose: {:#?}", path);
                    let submitted = proxy
                        .send_event(UserEvent::DownloadStarted(
                            uri,
                            default_path.as_path().display().to_string(),
                        ))
                        .is_ok();

                    return submitted;

                    //let _ = proxy.send_event(UserEvent::Rejected(uri));
                }
            })
            .build()?,
    );

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::NewEvents(StartCause::Init) => println!("ComfyUIViewer - 0.0.1"),
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => *control_flow = ControlFlow::Exit,

            Event::UserEvent(UserEvent::DownloadStarted(uri, temp_dir)) => {
                println!("Download: {}", uri);
                println!("Will write to: {:?}", temp_dir);
            }
            Event::UserEvent(UserEvent::Rejected(uri)) => {
                println!("Rejected download from: {}", uri)
            }
            Event::UserEvent(UserEvent::CloseWindow) => {
                let _ = _webview.take();
                *control_flow = ControlFlow::Exit
            }
            Event::UserEvent(UserEvent::LoadWorkflow) => {
                let vec_result = load_workflow(&Config::current().workflow_dir);
                let result = serde_json::to_string(&vec_result);
                let _send_me = r###".forEach((element)=>{console.log(element);var opt = document.createElement("option");opt.value = element;opt.textContent = element;document.getElementById("wf").appendChild(opt);})"###;
                let send_me = format!("try{{{}{}}}catch(e){{console.log(e)}}",&result.unwrap().as_str(),_send_me);
               
                let _ = _webview
                    .as_mut()
                    .unwrap()
                    .evaluate_script(send_me.as_str())
                    .unwrap();
            }
            Event::UserEvent(UserEvent::DeleteWorkflow(req)) => {
                
                let i: IPCData = serde_json::from_str(req.as_str()).unwrap();

                let _ = delete_workflow(&Config::current().workflow_dir,&i.data.filename);
                let send_me = format!(r#"
                    wf.remove(Array.from(wf).findIndex(e=>e.value=="{}"));
                "#,i.data.filename);
                let _ = _webview
                .as_mut()
                .unwrap()
                .evaluate_script(send_me.as_str())
                .unwrap();
            }
            Event::UserEvent(UserEvent::ApplyWorkflow(req)) => {
                let i: IPCData = serde_json::from_str(req.as_str()).unwrap();
                let content =  apply_workflow(&Config::current().workflow_dir, &i.data.filename);
                let send_me = format!("try{{window.app.loadGraphData({});}}catch(e){{console.log(e);}}",content.as_str());

                let _ = _webview
                .as_mut()
                .unwrap()
                .evaluate_script(send_me.as_str())
                .unwrap();
              
            }
            Event::UserEvent(UserEvent::AppendWorkflow(req)) => {
                let i: IPCData = serde_json::from_str(req.as_str()).unwrap();
                let content =  apply_workflow(&Config::current().workflow_dir, &i.data.filename);
                let send_me = format!(r#"
                try{{
                    const old = localStorage.getItem("litegrapheditor_clipboard");
                    localStorage.setItem("litegrapheditor_clipboard", JSON.stringify({}));
                    app.canvas.pasteFromClipboard();
                    localStorage.setItem("litegrapheditor_clipboard", old);
					
                }}catch(e){{
                    console.log(e);
                }}"#,content.as_str());

                let _ = _webview
                .as_mut()
                .unwrap()
                .evaluate_script(send_me.as_str())
                .unwrap();
            }
            Event::UserEvent(UserEvent::SaveWorkflow(req)) => {
                let i: IPCData = serde_json::from_str(req.as_str()).unwrap();
                let new_file =  save_workflow(&Config::current().workflow_dir,&i.data.data, &i.data.filename);     
                println!("new filename :{}",&new_file);
                let send_me = format!(r#"      
                try{{ 
                    var element = "{}";
                    console.log(element);                 
                    var opt = document.createElement("option");
                    opt.value = element;
                    opt.textContent = element;
                    document.getElementById("wf").appendChild(opt);
                }}catch(e){{console.log(e)}}        "#,new_file.as_str());
         
                let _ = _webview
                    .as_mut()
                    .unwrap()
                    .evaluate_script(send_me.as_str())
                    .unwrap();
            }
            _ => (),
        }
    });
}
