use chrono::NaiveDate;
use pdfium_render::prelude::*;
use std::path::PathBuf;

use doe::*;
use extract::{extract_data_from_pdf, extract_decision_date_from_vec};
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

    let opinion_data = extract_data_from_pdf(&document);

    if let Some(opinion_data) = opinion_data {
        let new_path = generate_new_file_name(&file_path, &opinion_data.date);

        println!("Renaming {:?} to {:?}", &file_path, &new_path);

        let res = watermark::watermark_case(document, &new_path, opinion_data);

        match res {
            Ok(()) => {
                println!("Successfully Renamed PDF: {:?}", file_path);

                // Remove
                let _ = fs::remove_file(file_path);
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

#[cfg(test)]
mod tests {

    use super::*;

    // use std::path::Path;
    // use std::path::Pathbuf;
    #[test]
    fn pfium_binds_to_binary() {
        let path = std::env::current_dir().unwrap();
        println!("The current directory is {}", path.display());

        let pdfium = Pdfium::default();
        // BIND to PDFIUM
        // let pdfium = Pdfium::new(
        //     Pdfium::bind_to_library(Pdfium::pdfium_platform_library_name_at_path("./"))
        //         .or_else(|_| Pdfium::bind_to_system_library())
        //         .expect("Failed to Bind to pdfium"),
        // );

        assert!(true);
    }

    #[test]
    fn test_renames_pdf() {
        let path = std::env::current_dir().unwrap();
        println!("The current directory is {}", path.display());

        let file_path =
            std::path::Path::new("./tests/In re Prosecution of Crawley_978 A.2d 608.pdf");

        let res = rename_pdf(&file_path.to_path_buf());
    }
}
