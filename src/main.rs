use rayon::prelude::*;
use std::io::{BufRead, BufReader};

use clap::Parser;
use pdf;

/// Simple PDF cracker
#[derive(Parser, Debug)]
#[command(author="Warren", version, about, long_about = None)]
struct Args {
    /// Path to a password protected PDF file
    #[arg(short, long)]
    pdf: String,

    /// Path to a wordlist. Each word should be on a new line
    #[arg(short, long)]
    wordlist: Option<String>,

    /// Whether or not the password is entirely numeric
    #[arg(short, long, default_value_t = false)]
    is_numeric: bool,

    /// Minimum length of numeric password
    #[arg(short, long, default_value_t = 1)]
    smallest_numeric_length: usize,

    /// Maximum length of numeric password
    #[arg(short, long, default_value_t = 8)]
    largest_numeric_length: usize,
}

fn try_password(pdf_contents: &[u8], password: &str) -> bool {
    pdf::file::File::from_data_password(pdf_contents, password.as_bytes()).is_ok()
}

fn main() {
    let args = Args::parse();

    let pdf_bytes = std::fs::read(&args.pdf).expect("Unable to read PDF");

    let mut password_list: Vec<String> = Vec::new();
    if let Some(wordlist) = args.wordlist {
        println!(
            "Attempting to crack {} using wordlist {}",
            &args.pdf, &wordlist
        );
        let passwords_file = std::fs::File::open(&wordlist).expect("Unable to open wordlist file");
        let mut reader = BufReader::new(passwords_file);

        let mut password = String::new();

        loop {
            let len = reader.read_line(&mut password);
            if len.is_err() {
                continue;
            }
            if len.unwrap() == 0 {
                break;
            }
            password_list.push(String::from(&password));
            password.clear();
        }
    } else if args.is_numeric {
        println!("Generating numeric password list...");
        (args.smallest_numeric_length..=args.largest_numeric_length)
            .into_iter()
            .for_each(|length| {
                println!("Generating length {} numeric passwords...", length);
                password_list.append(
                    &mut (0..usize::pow(10 as usize, length as u32))
                        .into_par_iter()
                        .map(|password| format!("{:0width$}", password, width = length))
                        .collect(),
                );
            });
    } else {
        println!("No cracking method specified. Did you mean to use numeric cracking?");
    }

    println!("Starting password cracking...");
    password_list.into_par_iter().for_each(|password| {
        if try_password(&pdf_bytes, &password) {
            println!("Found password: {}", &password.trim());
        }
    });

    println!("Done")
}
