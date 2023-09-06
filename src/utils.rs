use std::fs;

pub fn load_workflow() -> Vec<String> {
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

pub fn save_workflow(data: &str, filename: &str) -> String {
    // let _filename = generate_random(&filename, 5);
    // let fpath = format!(".\\workflows\\{}", &_filename);
    // let mut file = File::create(fpath).unwrap();
    // file.write(data.as_bytes()).expect("error writing file");
    // String::from(_filename)
    String::from("yeah")
}

pub fn delete_workflow(filename: &str) -> String {
    let filepath = format!(".\\workflows\\{}", filename);
    let result = match fs::remove_file(filepath) {
        Ok(()) => "ok",
        Err(_e) => "error",
    };
    String::from(result)
}
