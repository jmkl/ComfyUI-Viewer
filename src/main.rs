#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use serde::{Deserialize, Serialize};
use serde_json::json;
use serde_json::Result;
use wry::application::window::Window;
mod utils;

#[derive(Serialize, Deserialize)]
struct IPCData {
    key: String,
    data: String,
}

fn main() -> wry::Result<()> {
    use wry::{
        application::{
            event::{Event, StartCause, WindowEvent},
            event_loop::{ControlFlow, EventLoop},
            window::WindowBuilder,
        },
        webview::WebViewBuilder,
    };

    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("ComfyUI")
        .build(&event_loop)?;
    let _webview = WebViewBuilder::new(window)?
        .with_url("http://127.0.0.1:8188")?
        .with_initialization_script(include_str!("js/panel.js"))
        .with_ipc_handler(move |window: &Window, req: String| match req.as_str() {
            "load-workflow" => {
                println!("load me")
            }
            "apply-workflow" => {
                let i: IPCData = serde_json::from_str(req.as_str()).unwrap();
                println!("hello : {} \n{}!!!", i.key, i.data);
            }
            "save-workflow" => {}
            "delete-workflow" => {
                println!("delete me");
            }
            _ => {
                if !req.is_empty() {
                    let i: IPCData = serde_json::from_str(req.as_str()).unwrap();
                    println!("hello : {} \n{}!!!", i.key, i.data);
                    match i.key.as_str() {
                        "save" => {
                            println!("save : {}", i.data)
                        }
                        "apply" => {
                            println!("apply : {}", i.data)
                        }
                        "delete" => {
                            println!("delete : {}", i.data)
                        }
                        "load" => {
                            println!("load : {}", i.data)
                        }
                        _ => println!("nothing"),
                    }
                }
            }
        })
        .build()?;

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::NewEvents(StartCause::Init) => println!("Wry has started!"),
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => *control_flow = ControlFlow::Exit,
            _ => (),
        }
    });
}
