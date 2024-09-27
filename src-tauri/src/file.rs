use std::{fs, io::{Read, Write}};

#[tauri::command]
pub fn read_file_string(name: &str) -> String {
    let mut file = fs::File::open(name).unwrap();
    let mut contents = String::new();

    file.read_to_string(&mut contents).unwrap();

    return contents;
}

#[tauri::command]
pub fn read_file_bytes(name: &str) -> Vec<u8> {
    let mut file = fs::File::open(name).unwrap();
    let mut contents: Vec<u8> = Vec::new(); 
    
    file.read_to_end(&mut contents).unwrap();

    println!("{}", contents.len());

    return contents;
}

#[tauri::command]
pub fn append_to_file(name: &str, content: &str) {
    let mut file = fs::File::open(name).unwrap();
    file.write_all(content.as_bytes()).unwrap();
}

#[tauri::command]
pub fn create_file(path: &str) {
    let mut file = fs::File::create(path).unwrap();
}
