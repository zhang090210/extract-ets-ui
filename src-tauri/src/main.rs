// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

extern crate extract_ets_lib;

use std::env;
use std::fs::remove_file;
use std::path::Path;
use anyhow::Result;
use extract_ets_lib::{Paper, PaperType, ETS, Answers};
use tempfile::Builder;
use webbrowser;

fn main() -> Result<()> {
    // extract_ets_ui_lib::run();
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        // This is where you pass in your commands
        .invoke_handler(tauri::generate_handler![get_papers, view_result])
        .run(tauri::generate_context!())
        .expect("failed to run app");

    Ok(())
}

#[tauri::command]
fn get_papers() -> Result<Vec<Paper>, String> {
    let ets = match ETS::default() {
        Ok(ets) => ets,
        Err(e) => return Err(e.to_string())
    };
    let papers = match ets.get_papers() {
        Ok(papers) => papers,
        Err(e) => return Err(e.to_string())
    };

    Ok(papers)
}

#[tauri::command]
fn view_result(paper_path: &str, paper_type: PaperType) -> Result<String, String> {
    let paper = Paper::read_from_path(paper_path).expect("Failed to read paper");
    let answers = paper.read_answers(paper_type).expect("Failed to read answers");

    // 创建一个临时文件构建器
    let mut builder = Builder::new();

    // 设置临时文件的目录，这里使用系统的临时目录
    builder.tempdir().expect("Failed to create temp dir");
    if Path::exists(
        &env::temp_dir().join(format!("{}.html", paper.paper_id))
    ) {
        remove_file(&env::temp_dir().join(format!("{}.html", paper.paper_id))).expect("Failed to remove file");
    }
    // 创建一个没有前缀的临时文件
    let file = builder
        .prefix(&paper.paper_id) // 设置前缀为空字符串
        .suffix(".html") // 设置后缀为空字符串
        .rand_bytes(0) // 不使用随机字节
        .tempfile()
        .expect("Failed to create temp file");
    let _ = answers.export_to_html(&file.path().to_str().unwrap()).expect("Failed to export to html");
    webbrowser::open(file.path().to_str().unwrap()).expect("Failed to open browser");
    let (_file, _path) = &file.keep().expect("TODO: panic message");
    Ok("success".to_string())
}