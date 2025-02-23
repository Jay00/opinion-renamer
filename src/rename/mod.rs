use chrono::NaiveDate;
use pdfium_render::prelude::*;
use std::path::PathBuf;

use doe::*;
use extract::{
    extract_decision_date_from_string, extract_decision_date_from_vec, get_first_page_text,
};
use std::fs;

mod extract;
mod watermark;

fn generate_new_file_name(file_path: &PathBuf, opinion_date: &NaiveDate) -> PathBuf {
    // Generate a name for the file
    let n = &file_path.file_name().unwrap().to_string_lossy();

    let mut prefix_str = opinion_date.format("%Y-%m-%d").to_string();
    prefix_str.push_str(" ");
    prefix_str.push_str(&n);

    let new_path = file_path.with_file_name(prefix_str);
    return new_path;
}

pub fn rename_pdf(file_path: &PathBuf) {
    let pdfium = Pdfium::default();
    let document = pdfium.load_pdf_from_file(&file_path, None).unwrap();

    let first_page = get_first_page_text(&document);

    let opinion_date_option = extract_decision_date_from_string(first_page);

    if let Some(opinion_date) = opinion_date_option {
        let new_path = generate_new_file_name(&file_path, &opinion_date);

        println!("Renaming {:?} to {:?}", &file_path, &new_path);

        let res = watermark::watermark_case(document, &new_path, opinion_date);

        match res {
            Ok(()) => {
                println!("Renamed!");
            }
            Err(err) => {
                eprintln!("{err}");
            }
        } // Rename a.txt to b.txt
    }
}

pub fn rename_docx(file_path: &PathBuf) {
    // docx::docx_replace("./name.docx", "name", "andrew").unwrap();
    let content = docx::docx_get_content(&file_path.to_string_lossy()).unwrap();

    let opinion_date_option = extract_decision_date_from_vec(&content);

    if let Some(opinion_date) = opinion_date_option {
        let new_path = generate_new_file_name(&file_path, &opinion_date);

        println!("Renaming {:?} to {:?}", &file_path, &new_path);
        let res = fs::rename(&file_path, &new_path);
        match res {
            Ok(()) => {
                println!("Renamed!");
            }
            Err(err) => {
                eprintln!("{err}");
            }
        } // Rename a.txt to b.txt
    }
}
