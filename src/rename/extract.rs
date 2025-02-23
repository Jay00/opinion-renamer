use chrono::NaiveDate;
use regex::Regex;

use pdfium_render::prelude::*;

/// Convert Single PDF's to RAW txt lines
pub fn get_first_page_text(document: &PdfDocument) -> String {
    let text = document.pages().first().unwrap().text().unwrap().all();
    return text;
}

pub fn extract_decision_date_from_string(content: String) -> Option<NaiveDate> {
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
