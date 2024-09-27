use std::{fs, io::{Read, Write}};

mod file;

// Prevents additional console window on Windows in release, DO NOT REMOVE!#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![file::read_file_string, 
                                                 file::read_file_bytes, 
                                                 file::append_to_file])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
