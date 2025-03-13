use chrono::NaiveDate;
use regex::Regex;

use pdfium_render::prelude::*;

pub struct Opinion {
    pub date: NaiveDate,
    pub caption: String,
    pub court: String,
    pub reporter: String,
}

impl Opinion {
    pub fn to_date_y_m_d(&self) -> String {
        self.date.format("%Y-%m-%d").to_string()
    }

    pub fn to_date_medium(&self) -> String {
        self.date.format("%b. %-e, %Y").to_string()
    }
}

pub fn extract_data_from_pdf(document: &PdfDocument) -> Option<Opinion> {
    let first_page = get_first_page_text(document);
    let date_option = extract_decision_date_from_string(&first_page);
    let cc = extract_caption_and_court(&first_page);
    let reporter = extract_reporter(&first_page);

    if let Some(date) = date_option {
        return Some(Opinion {
            date,
            caption: cc.0,
            court: cc.1,
            reporter: reporter,
        });
    } else {
        return None;
    }
}

/// Convert Single PDF's to RAW txt lines
fn get_first_page_text(document: &PdfDocument) -> String {
    // let text = document.pages().first().unwrap().text().unwrap().all();

    let first_page = document.pages().first().unwrap();

    let text = first_page
        .objects()
        .iter()
        .filter_map(|object| object.as_text_object().map(|object| object.text()))
        .collect::<Vec<_>>()
        .join("\n");

    // print!("{text}");

    return text;
}

fn extract_decision_date_from_string(content: &String) -> Option<NaiveDate> {
    // Look for "September 20, 2011, Argued; September 6, 2012, Decided"
    let re = Regex::new(r"((January|February|March|April|May|June|July|August|September|October|November|December) \d{1,2}, \d{4}), (Decided|Filed|Rendered)").unwrap();

    let re_alt = Regex::new(r"(Argued (January|February|March|April|May|June|July|August|September|October|November|December) \d{1,2}, \d{4}) ; ((January|February|March|April|May|June|July|August|September|October|November|December) \d{1,2}, \d{4})").unwrap();

    let re_alt_2 = Regex::new(r"(January|February|March|April|May|June|July|August|September|October|November|December) \d{1,2}, \d{4}").unwrap();

    // println!("{content}");
    if let Some(cap) = re.captures(&content) {
        let date_str = &cap[1];

        println!("Found date of opinion: {:?}", date_str);

        let date_of_opinion = NaiveDate::parse_from_str(date_str, "%B %e, %Y").unwrap();

        return Some(date_of_opinion);
    }

    // "Argued August 24, 1959 ; December 15, 1959"
    if let Some(cap) = re_alt.captures(&content) {
        let date_str = &cap[3];

        println!("Found date of opinion: {:?}", date_str);

        let date_of_opinion = NaiveDate::parse_from_str(date_str, "%B %e, %Y").unwrap();

        return Some(date_of_opinion);
    }

    // "December 15, 1959"
    // if let Some(cap) = re_alt_2.captures(&content) {
    //     let date_str = &cap[0];

    //     println!("Found date of opinion: {:?}", date_str);

    //     let date_of_opinion = NaiveDate::parse_from_str(date_str, "%B %e, %Y").unwrap();

    //     return Some(date_of_opinion);
    // }

    eprintln!("\n PDF. No date found in opinion!");
    // No decision date found.
    None
}

fn extract_caption_and_court(content: &String) -> (String, String) {
    // Look for "September 20, 2011, Argued; September 6, 2012, Decided"
    let re = Regex::new(r"(?m)(^.+ v. .+$)\n(^.+$)").unwrap();

    // println!("{content}");
    if let Some(cap) = re.captures(&content) {
        let caption = cap[1].to_string();
        let court = cap[2].to_string();

        println!("Found caption and court: {:?}, {:?}", caption, court);

        return (caption, court);
    } else {
        // No decision date found.
        eprintln!("No date found in opinion!");
        return ("Not found".to_string(), "Not found".to_string());
    }
}

fn extract_reporter(content: &String) -> String {
    // Look for "September 20, 2011, Argued; September 6, 2012, Decided"
    let re = Regex::new(r"(?m)(Reporter\n)([^\*]+)").unwrap();

    // println!("{content}");
    if let Some(cap) = re.captures(&content) {
        let reporter_cite = cap[2].trim().to_string();

        println!("Found reporter citaton of opinion: {:?}", reporter_cite);

        return reporter_cite;
    } else {
        eprintln!("No reporter found in opinion!");
        return "Not Found".to_string();
    }
}

pub fn extract_decision_date_from_vec(content: &Vec<String>) -> Option<NaiveDate> {
    // Look for "September 20, 2011, Argued; September 6, 2012, Decided"
    let re = Regex::new(r"((January|February|March|April|May|June|July|August|September|October|November|December) \d{1,2}, \d{4}), (Decided|Filed|Rendered)").unwrap();

    let re_alt = Regex::new(r"(Argued (January|February|March|April|May|June|July|August|September|October|November|December) \d{1,2}, \d{4}) ; ((January|February|March|April|May|June|July|August|September|October|November|December) \d{1,2}, \d{4})").unwrap();

    for s in &content[0..30] {
        println!("{s}");
        if let Some(cap) = re.captures(s) {
            let date_str = &cap[1];

            println!("DOCX: Found date of opinion: {:?}", date_str);

            let date_of_opinion = NaiveDate::parse_from_str(date_str, "%B %e, %Y").unwrap();

            return Some(date_of_opinion);
        }
    }

    for s in &content[0..30] {
        println!("{s}");
        if let Some(cap) = re_alt.captures(s) {
            let date_str = &cap[3];

            println!("DOCX: Found date of opinion: {:?}", date_str);

            let date_of_opinion = NaiveDate::parse_from_str(date_str, "%B %e, %Y").unwrap();

            return Some(date_of_opinion);
        }
    }

    eprintln!("DOCX: No date found in opinion!");
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
        extract_decision_date_from_string(&text);

        assert!(true);
    }

    #[test]
    fn gets_pdf_data() {
        let path = std::env::current_dir().unwrap();
        println!("The current directory is {}", path.display());

        let file_path = std::path::Path::new("./Curry v. United States.pdf");
        let pdfium = Pdfium::default();
        let document = pdfium.load_pdf_from_file(&file_path, None).unwrap();

        let op = extract_data_from_pdf(&document);

        assert!(true);
    }

    #[test]
    fn gets_pdf_data_cross() {
        let path = std::env::current_dir().unwrap();
        println!("The current directory is {}", path.display());

        let file_path = std::path::Path::new("./tests/Cross v. United States_145 U.S. 571.Pdf");
        let pdfium = Pdfium::default();
        let document = pdfium.load_pdf_from_file(&file_path, None).unwrap();

        let op = extract_data_from_pdf(&document);

        assert!(true);
    }
}
