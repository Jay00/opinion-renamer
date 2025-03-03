use clap::builder::styling;
use clap::builder::OsStr;
use clap::builder::Styles;
use clap::Parser;
use dunce::canonicalize;

use regex::Regex;
use std::path::PathBuf;
use walkdir::WalkDir;

mod rename;

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

fn main() {
    let cli = Cli::parse();

    // You can check the value provided by positional arguments, or option arguments
    let path = cli.path;

    println!("Provided path: {:?}", path);

    process_directory(&path);
}

fn process_directory(path: &PathBuf) {
    let case_file_directory_absolute = canonicalize(path).unwrap();

    // let dir = "C:\\Users\\jason\\Downloads\\third";
    let dir = case_file_directory_absolute;

    // Date Regex YYYY-MM-DD
    let re = Regex::new(r"^(\d{4})-(\d{2})-(\d{2})").unwrap();

    let pdf_ext = OsStr::from("pdf");
    let docx_ext = OsStr::from("docx");

    println!("Renaming docx files in dir {:?}", dir);

    let walker = WalkDir::new(dir).into_iter();
    for entry in walker.into_iter() {
        if let Ok(e) = entry {
            let file_name_str = e.file_name().to_string_lossy();
            print!("{file_name_str}");

            // Check if date string already prefix's file
            let x = re.is_match(&file_name_str);
            if x {
                print!(" Skipping . . . \n");
                // Skip documents which already have the date
                continue;
            }

            let ext = e.path().extension();

            if let Some(ext) = ext {
                let ext_lower = ext.to_ascii_lowercase();
                // We are only looking in docx files
                if ext_lower == docx_ext {
                    let file_path = PathBuf::from(e.path());
                    rename::rename_docx(&file_path);
                }

                // We are only looking in pdf files
                if ext_lower == pdf_ext {
                    let file_path = PathBuf::from(e.path());
                    rename::rename_pdf(&file_path);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use clap::builder::OsStr;

    use crate::process_directory;

    #[test]
    fn gets_page_text() {
        // let path = std::path::Path::new("./tests");
        let path = std::path::Path::new(
            r"C:\Users\jason\C&S Dropbox\Active\Turner_Lewkus_1994-02-20\23-CF-784\research\Which Law Applies",
        );

        assert!(path.exists());
        process_directory(&path.to_path_buf());

        assert!(true);
    }

    #[test]
    fn comparison_of_osstr() {
        let pdf_ext = OsStr::from("pdf");
        let docx_ext = OsStr::from("docx");

        let test_ext = OsStr::from("pdf");
        assert_eq!(pdf_ext, test_ext);

        let test_ext = OsStr::from("PDF").to_ascii_lowercase();
        assert_eq!(pdf_ext, test_ext);

        let test_ext = OsStr::from("PDF").to_ascii_lowercase();
        assert_ne!(docx_ext, test_ext);
    }
}
