use clap::builder::styling;
use clap::builder::Styles;
use clap::Parser;
use dunce::canonicalize;

use chrono::NaiveDate;
use doe::*;
use regex::Regex;
use std::{fs, path::PathBuf};
use walkdir::WalkDir;

fn my_styles() -> Styles {
    styling::Styles::styled()
        .header(styling::AnsiColor::Yellow.on_default() | styling::Effects::BOLD)
        .usage(styling::AnsiColor::Yellow.on_default() | styling::Effects::BOLD)
        .literal(styling::AnsiColor::Green.on_default() | styling::Effects::BOLD)
        .placeholder(styling::AnsiColor::Green.on_default())
}

#[derive(Parser)]
#[command(styles=my_styles())]
#[command(arg_required_else_help = true)]
#[command(author = "Jason K. Clark <jasonclarklaw.com>")]
#[command(version)]
#[command(name = "Opinion Renamer")]
#[command(bin_name = "renameops")]
#[command(about = "\n\n
** OPINION RENAMER **
A utility to rename docx court opinions by date.
Created by Jason K. Clark", long_about = None)]
struct Cli {
    /// Optional path to the directory containing your case file
    #[arg(short, long, value_name = "FILE", default_value = "./")]
    path: PathBuf,
}

fn extract_decision_date(content: &Vec<String>) -> Option<NaiveDate> {
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
    let cli = Cli::parse();

    // You can check the value provided by positional arguments, or option arguments
    let path = cli.path;

    println!("Provided path: {:?}", path);

    let case_file_directory_absolute = canonicalize(path).unwrap();

    // let dir = "C:\\Users\\jason\\Downloads\\third";
    let dir = case_file_directory_absolute;

    // Date Regex YYYY-MM-DD
    let re = Regex::new(r"^(\d{4})-(\d{2})-(\d{2})").unwrap();

    println!("Renaming docx files in dir {:?}", dir);
    let walker = WalkDir::new(dir).into_iter();
    for entry in walker.into_iter() {
        if let Ok(e) = entry {
            let file_name_str = e.file_name().to_string_lossy();
            // We are only looking in docx files
            if file_name_str.ends_with("docx") {
                // Check if date string is already appended
                let x = re.is_match(&file_name_str);
                if x {
                    // Skip documents which already have the date
                    continue;
                }

                println!("Checking File: {}", e.path().display());

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
