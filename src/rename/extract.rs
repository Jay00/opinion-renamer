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

        log::debug!("Found date of opinion: {:?}", date_str);

        let date_of_opinion = NaiveDate::parse_from_str(date_str, "%B %e, %Y").unwrap();

        return Some(date_of_opinion);
    }

    // "Argued August 24, 1959 ; December 15, 1959"
    if let Some(cap) = re_alt.captures(&content) {
        let date_str = &cap[3];

        log::debug!("Found date of opinion: {:?}", date_str);

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

    log::error!("\n PDF. No date found in opinion!");
    // No decision date found.
    None
}

fn extract_caption_and_court(content: &String) -> (String, String) {
    // Look for e.g., "Curry v. United States"
    // Next line is usually court name
    let re = Regex::new(r"(?m)(^.+ v. .+$)\n(^.+$)").unwrap();

    // Also need to look for "In re "
    // Second Line will be court name
    let re_in_re_cases = Regex::new(r"(?m)(^In re .+$)\n(^.+$)").unwrap();

    // println!("{content}");
    if let Some(cap) = re.captures(&content) {
        let caption = cap[1].to_string();
        let court = cap[2].to_string();

        println!("Found caption and court: {:?}, {:?}", caption, court);

        return (caption, court);
    } else if let Some(cap) = re_in_re_cases.captures(&content) {
        let caption = cap[1].to_string();
        let court = cap[2].to_string();

        println!("Found caption and court: {:?}, {:?}", caption, court);

        return (caption, court);
    } else {
        // No decision date found.
        eprintln!("No Case Name / Caption found in opinion!");
        return ("Not found".to_string(), "Not found".to_string());
    }
}

fn extract_reporter(content: &String) -> String {
    // Look for "September 20, 2011, Argued; September 6, 2012, Decided"
    let re = Regex::new(r"(?m)(Reporter\n)([^\*]+)").unwrap();

    // println!("{content}");
    if let Some(cap) = re.captures(&content) {
        let reporter_cite = cap[2].trim().to_string();

        log::debug!("Found reporter citaton of opinion: {:?}", reporter_cite);

        return reporter_cite;
    } else {
        log::warn!("No reporter found in opinion!");
        return "Not Found".to_string();
    }
}

pub fn extract_decision_date_from_vec(content: &Vec<String>) -> Option<NaiveDate> {
    // Look for "September 20, 2011, Argued; September 6, 2012, Decided"
    let re = Regex::new(r"((January|February|March|April|May|June|July|August|September|October|November|December) \d{1,2}, \d{4}), (Decided|Filed|Rendered)").unwrap();

    let re_alt = Regex::new(r"(Argued (January|February|March|April|May|June|July|August|September|October|November|December) \d{1,2}, \d{4}) ; ((January|February|March|April|May|June|July|August|September|October|November|December) \d{1,2}, \d{4})").unwrap();

    for s in &content[0..30] {
        log::debug!("{s}");
        if let Some(cap) = re.captures(s) {
            let date_str = &cap[1];

            log::debug!("DOCX: Found date of opinion: {:?}", date_str);

            let date_of_opinion = NaiveDate::parse_from_str(date_str, "%B %e, %Y").unwrap();

            return Some(date_of_opinion);
        }
    }

    for s in &content[0..30] {
        log::debug!("{s}");
        if let Some(cap) = re_alt.captures(s) {
            let date_str = &cap[3];

            log::debug!("DOCX: Found date of opinion: {:?}", date_str);

            let date_of_opinion = NaiveDate::parse_from_str(date_str, "%B %e, %Y").unwrap();

            return Some(date_of_opinion);
        }
    }

    log::warn!("DOCX: No date found in opinion!");
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

        let _pdfium = Pdfium::default();

        assert!(true);
    }

    #[test]
    fn gets_page_text() {
        let path = std::env::current_dir().unwrap();
        println!("The current directory is {}", path.display());

        let pdfium = Pdfium::default();

        let file_path = std::path::Path::new("./tests/Curry v. United States.pdf");
        let document = pdfium.load_pdf_from_file(&file_path, None).unwrap();

        let text = get_first_page_text(&document);

        let reporter = extract_reporter(&text);
        assert_eq!(reporter, "520 A.2d 255");

        let (caption, court) = extract_caption_and_court(&text);
        assert_eq!(caption, "Curry v. United States");
        assert_eq!(court, "District of Columbia Court of Appeals");

        let date = extract_decision_date_from_string(&text);
        assert_eq!(date, Some(NaiveDate::from_ymd_opt(1987, 1, 14).unwrap()));

        let file_path =
            std::path::Path::new("./tests/In re Prosecution of Crawley_978 A.2d 608.pdf");
        let document = pdfium.load_pdf_from_file(&file_path, None).unwrap();

        let text = get_first_page_text(&document);

        let reporter = extract_reporter(&text);
        assert_eq!(reporter, "978 A.2d 608");

        let (caption, court) = extract_caption_and_court(&text);
        assert_eq!(caption, "In re Prosecution of Crawley");
        assert_eq!(court, "District of Columbia Court of Appeals");

        let date = extract_decision_date_from_string(&text);
        assert_eq!(date, Some(NaiveDate::from_ymd_opt(2009, 8, 20).unwrap()));
    }

    #[test]
    fn gets_pdf_data() {
        let path = std::env::current_dir().unwrap();
        println!("The current directory is {}", path.display());

        let file_path =
            std::path::Path::new("./tests/In re Prosecution of Crawley_978 A.2d 608.pdf");
        let pdfium = Pdfium::default();
        let document = pdfium.load_pdf_from_file(&file_path, None).unwrap();

        let op = extract_data_from_pdf(&document).unwrap();

        let expected_opinion = Opinion {
            date: NaiveDate::from_ymd_opt(2009, 8, 20).unwrap(),
            caption: "In re Prosecution of Crawley".to_string(),
            court: "District of Columbia Court of Appeals".to_string(),
            reporter: "978 A.2d 608".to_string(),
        };

        assert_eq!(op.date, expected_opinion.date);
        assert_eq!(op.caption, expected_opinion.caption);
        assert_eq!(op.court, expected_opinion.court);
        assert_eq!(op.reporter, expected_opinion.reporter);
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
