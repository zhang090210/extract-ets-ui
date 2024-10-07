// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

extern crate extract_ets_lib;
use anyhow::Result;
use extract_ets_lib::{Paper, ETS};
use wkhtmltopdf::PdfApplication;

fn main() -> Result<()> {
    // extract_ets_ui_lib::run();
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        // This is where you pass in your commands
        .invoke_handler(tauri::generate_handler![get_papers, export_pdf])
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
fn export_pdf(source_path: &str, pdf_path: &str) -> Result<String, String> {
    let pdf_app = match PdfApplication::new() {
        Ok(app) => app,
        Err(e) => return Err(e.to_string())
    };

    let paper = match Paper::read_from_path(source_path) {
        Ok(paper) => paper,
        Err(e) => return Err(e.to_string())
    };
    let answers = match paper.read_answers() {
        Ok(answers) => answers,
        Err(e) => return Err(e.to_string())
    };

    answers.export_pdf(&pdf_app, pdf_path).expect("failed to export pdf");

    Ok("success to export pdf".into())
}
