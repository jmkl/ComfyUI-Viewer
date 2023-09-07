use rand::Rng;
use serde_json::json;
use std::fs;
use std::io::{BufWriter, Write};
use std::path::Path;
use std::sync::{Arc, RwLock};
#[derive(Default, serde::Deserialize)]
pub struct Config {
    pub port: u32,
    pub workflow_dir: String,
}
impl Config {
    pub fn current() -> Arc<Config> {
        CURRENT_CONFIG.with(|c| c.read().unwrap().clone())
    }
    pub fn make_current(self) {
        CURRENT_CONFIG.with(|c| *c.write().unwrap() = Arc::new(self))
    }
}
thread_local! {
    static CURRENT_CONFIG: RwLock<Arc<Config>> = RwLock::new(Default::default());
}

pub fn load_config() -> Result<Config, Box<dyn std::error::Error>> {
    let config = Path::new(".\\.comfyuiviewer.json");
    if !config.exists() {
        let file = fs::File::create(&config).unwrap();
        let data = json!({
            "port": 8188,
            "workflow_dir": ".\\workflows",
        });
        let mut writer = BufWriter::new(file);
        serde_json::to_writer_pretty(&mut writer, &data).unwrap();
        writer.flush().unwrap();
    }
    let contents = match fs::read_to_string(config) {
        Ok(ff) => ff,
        Err(_e) => String::from("Not Foundd"),
    };
    let conf: Config = serde_json::from_str(&String::from(contents))?;
    Ok(conf)
}

pub fn load_workflow(wfdir: &str) -> Vec<String> {
    fs::create_dir_all(wfdir).expect("failed to create");
    let paths = fs::read_dir(wfdir)
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

fn generate_random(oldname: &str, str_len: usize) -> String {
    const CHARSET: &[u8] = b"abcdefghijklmnopqrstuvwxyz0123456789";

    let mut rng = rand::thread_rng();
    let password: String = (0..str_len)
        .map(|_| {
            let idx = rng.gen_range(0..CHARSET.len());
            CHARSET[idx] as char
        })
        .collect();
    format!("{} [{}].json", oldname, password)
}

pub fn save_workflow(wfdir: &str, data: &str, filename: &str) -> String {
    let _filename = generate_random(&filename, 5);
    let fpath = format!("{}\\{}", wfdir, &_filename);
    let mut file = fs::File::create(fpath).unwrap();
    file.write(data.as_bytes()).expect("error writing file");
    String::from(_filename)
}

pub fn delete_workflow(wfdir: &str, filename: &str) -> String {
    let filepath = format!("{}\\{}", wfdir, filename);
    let result = match fs::remove_file(filepath) {
        Ok(()) => "ok",
        Err(_e) => "error",
    };
    String::from(result)
}
pub fn apply_workflow(wfdir: &str, data: &str) -> String {
    let fpath = format!("{}\\{}", wfdir, data);
    let contents = match fs::read_to_string(fpath) {
        Ok(ff) => ff,
        Err(_e) => String::from("{}"),
    };
    contents
    // WINDOW
    //     .get()
    //     .expect("window avail")
    //     .emit("msg", strct::Payload { message: contents })
    //     .expect("failed");
}
