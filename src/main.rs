use chrono::NaiveDate;
use doe::*;
use regex::Regex;
use std::{fs, path::PathBuf};
use walkdir::WalkDir;

fn extract_decision_date(content: &Vec<String>) -> Option<NaiveDate> {
    // Look for "September 20, 2011, Argued; September 6, 2012, Decided"
    let re = Regex::new(r"((January|February|March|April|May|June|July|August|September|October|November|December) \d{1,2}, \d{4}), Decided").unwrap();

    for s in &content[0..30] {
        if let Some(cap) = re.captures(s) {
            let date_str = &cap[1];

            println!("Found date of opinion: {:?}", date_str);

            let date_of_opinion = NaiveDate::parse_from_str(date_str, "%B %e, %Y").unwrap();

            return Some(date_of_opinion);
        }
    }
    // No decision date found.
    None
}

fn generate_new_file_name(file_path: &PathBuf, opinion_date: &NaiveDate) -> PathBuf {
    // Generate a name for the file
    let n = &file_path.file_name().unwrap().to_string_lossy();

    let mut prefix_str = opinion_date.format("%Y-%m-%d").to_string();
    prefix_str.push_str(" ");
    prefix_str.push_str(&n);

    let new_path = file_path.with_file_name(prefix_str);
    return new_path;
}

fn main() {
    let dir = "C:\\Users\\jason\\Downloads\\more cases";

    println!("Renaming docx files in dir {}", dir);
    let walker = WalkDir::new(dir).into_iter();
    for entry in walker.into_iter() {
        if let Ok(e) = entry {
            println!("{}", e.path().display());

            // We are only looking in docx files
            if e.file_name().to_string_lossy().ends_with("docx") {
                let file_path = PathBuf::from(e.path());

                // docx::docx_replace("./name.docx", "name", "andrew").unwrap();
                let content = docx::docx_get_content(&file_path.to_string_lossy()).unwrap();

                let opinion_date_option = extract_decision_date(&content);

                if let Some(opinion_date) = opinion_date_option {
                    let new_path = generate_new_file_name(&file_path, &opinion_date);

                    println!("Renaming {:?} to {:?}", &file_path, &new_path);
                    let res = fs::rename(&file_path, &new_path);
                    match res {
                        Ok(()) => {
                            println!("Renamed!");
                            // Exit the loop. We don't need to keep looking through the remaining strings.
                            break;
                        }
                        Err(err) => {
                            eprintln!("{err}");
                        }
                    } // Rename a.txt to b.txt
                }
            }
        }
    }

    // docx::docx_remove_read_only("./name.docx").unwrap();
}
