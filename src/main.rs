use chrono::NaiveDate;
use doe::*;
use regex::Regex;
use std::{fs, path::PathBuf, process::exit};
use walkdir::{DirEntry, WalkDir};

fn is_word_docx(entry: &DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|s| s.ends_with(".docx"))
        .unwrap_or(false)
}

fn main() {
    // Look for "September 20, 2011, Argued; September 6, 2012, Decided"
    let re = Regex::new(r"((January|February|March|April|May|June|July|August|September|October|November|December) \d{1,2}, \d{4}), Decided").unwrap();

    let dir = "C:\\Users\\jason\\Downloads\\more cases";

    println!("Renaming docx files in dir {}", dir);
    let walker = WalkDir::new(dir).into_iter();
    for entry in walker.into_iter() {
        if let Ok(e) = entry {
            println!("{}", e.path().display());

            if e.file_name().to_string_lossy().ends_with("docx") {
                let file_path = PathBuf::from(e.path());

                if !file_path.exists() {
                    eprintln!("File not found");
                    exit(0);
                }
                // docx::docx_replace("./name.docx", "name", "andrew").unwrap();
                let content = docx::docx_get_content(&file_path.to_string_lossy()).unwrap();

                for s in &content[0..30] {
                    if let Some(cap) = re.captures(s) {
                        let date_str = &cap[1];

                        println!("Found date of opinion: {} {:?}", date_str, file_path);

                        let date_of_opinion =
                            NaiveDate::parse_from_str(date_str, "%B %e, %Y").unwrap();

                        let mut prefix_str = date_of_opinion.format("%Y-%m-%d").to_string();

                        let n = &file_path.file_name().unwrap().to_string_lossy();
                        prefix_str.push_str(" ");
                        prefix_str.push_str(&n);

                        let new_path = file_path.with_file_name(prefix_str);

                        println!("Renaming {:?} to {:?}", &file_path, &new_path);
                        let res = fs::rename(&file_path, &new_path);
                        match res {
                            Ok(()) => {
                                println!("Renamed!");
                            }
                            Err(err) => {
                                println!("{err}");
                            }
                        } // Rename a.txt to b.txt
                    }
                }
            }
        }
    }

    // docx::docx_remove_read_only("./name.docx").unwrap();
}
