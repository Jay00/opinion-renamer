use chrono::NaiveDate;
use regex::Regex;

use pdfium_render::prelude::*;

/// Convert Single PDF's to RAW txt lines
pub fn get_first_page_text(document: &PdfDocument) -> String {
    // let text = document.pages().first().unwrap().text().unwrap().all();

    let first_page = document.pages().first().unwrap();

    let text = first_page
        .objects()
        .iter()
        .filter_map(|object| object.as_text_object().map(|object| object.text()))
        .collect::<Vec<_>>()
        .join("\n");

    print!("{text}");

    return text;
}

pub fn extract_decision_date_from_string(content: &String) -> Option<NaiveDate> {
    // Look for "September 20, 2011, Argued; September 6, 2012, Decided"
    let re = Regex::new(r"((January|February|March|April|May|June|July|August|September|October|November|December) \d{1,2}, \d{4}), (Decided|Filed|Rendered)").unwrap();

    println!("{content}");
    if let Some(cap) = re.captures(&content) {
        let date_str = &cap[1];

        println!("Found date of opinion: {:?}", date_str);

        let date_of_opinion = NaiveDate::parse_from_str(date_str, "%B %e, %Y").unwrap();

        return Some(date_of_opinion);
    }

    eprintln!("No date found in opinion!");
    // No decision date found.
    None
}

pub fn extract_caption_and_court(content: &String) -> Option<(String, String)> {
    // Look for "September 20, 2011, Argued; September 6, 2012, Decided"
    let re = Regex::new(r"(?m)(^.+ v. .+$)\n(^.+$)").unwrap();

    println!("{content}");
    if let Some(cap) = re.captures(&content) {
        let caption = cap[1].to_string();
        let court = cap[2].to_string();

        println!("Found caption and court: {:?}, {:?}", caption, court);

        return Some((caption, court));
    }

    eprintln!("No date found in opinion!");
    // No decision date found.
    None
}

pub fn extract_reporter(content: &String) -> Option<String> {
    // Look for "September 20, 2011, Argued; September 6, 2012, Decided"
    let re = Regex::new(r"(?m)(Reporter\n)([^\*]+)").unwrap();

    println!("{content}");
    if let Some(cap) = re.captures(&content) {
        let reporter_cite = cap[2].to_string();

        println!("Found reporter citaton of opinion: {:?}", reporter_cite);

        return Some(reporter_cite);
    }

    eprintln!("No date found in opinion!");
    // No decision date found.
    None
}

pub fn extract_decision_date_from_vec(content: &Vec<String>) -> Option<NaiveDate> {
    // Look for "September 20, 2011, Argued; September 6, 2012, Decided"
    let re = Regex::new(r"((January|February|March|April|May|June|July|August|September|October|November|December) \d{1,2}, \d{4}), (Decided|Filed|Rendered)").unwrap();

    for s in &content[0..30] {
        println!("{s}");
        if let Some(cap) = re.captures(s) {
            let date_str = &cap[1];

            println!("Found date of opinion: {:?}", date_str);

            let date_of_opinion = NaiveDate::parse_from_str(date_str, "%B %e, %Y").unwrap();

            return Some(date_of_opinion);
        }
    }
    eprintln!("No date found in opinion!");
    // No decision date found.
    None
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
    fn gets_page_text() {
        let path = std::env::current_dir().unwrap();
        println!("The current directory is {}", path.display());

        let file_path = std::path::Path::new("./Curry v. United States.pdf");
        let pdfium = Pdfium::default();
        let document = pdfium.load_pdf_from_file(&file_path, None).unwrap();

        let text = get_first_page_text(&document);

        extract_reporter(&text);
        extract_caption_and_court(&text);

        assert!(true);
    }
}
