use std::{io::{BufReader, BufRead}, thread};

use pdf;
use clap::Parser;

#[derive(Clone, Debug)]
struct Password {
    slots: Vec<u8>
}

impl Password {
    fn new(length: usize) -> Self {
        let mut result = Password {slots: vec![]};
        for _ in 0..length {
            result.slots.push(0);
        }
        result
    }

    fn next(&self) -> Option<Self> {
        let mut result = self.clone();
        for i in 0..result.slots.len() {
            result.slots[i] += 1;
            if result.slots[i] >= 10 {
                result.slots[i] = 0;
                if i == result.slots.len() - 1{
                    return None;
                }
            }
            else {
                break;
            }
        }
        Some(result)
    }

    fn to_string(&self) -> String {
        let mut result = String::new();
        for digit in &self.slots {
            result += format!("{}", digit).as_str();
        }
        result
    }
}

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

    /// Number of threads to use
    #[arg(short, long, default_value_t = 16)]
    threads: usize,
}


fn try_password(pdf_path: &str, password: &str) -> bool {
    let result = pdf::file::FileOptions::cached().password(password.as_bytes()).open(pdf_path);
    result.is_ok()
}

fn main() {
    let args = Args::parse();
    let mut password_list: Vec<String> = Vec::new();
    if let Some(wordlist) = args.wordlist {
        println!("Attempting to crack {} using wordlist {}", &args.pdf, &wordlist);
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
    }
    else if args.is_numeric {
        println!("Generating numeric password list...");
        for length in args.smallest_numeric_length..=args.largest_numeric_length {
            println!("Generating length {} numeric passwords...", length);
            let mut password: Password = Password::new(length);
            password_list.push(password.to_string());
            loop {
                let new_password = password.next();
                if let Some(pass) = new_password {
                    password_list.push(pass.to_string());
                    password = pass;
                }
                else {
                    break;
                }
            }
            
        }
    }
    else {
        println!("No cracking method specified. Did you mean to use numeric cracking?");
    }
    
    println!("Starting password cracking threads...");
    let passwords_per_thread = ((password_list.len() as f64) / (args.threads as f64)).ceil() as usize;
    let mut threads = Vec::new();

    for i in 0..args.threads {
        let mut start = passwords_per_thread*i;
        let mut end = passwords_per_thread*(i+1);
        if start > password_list.len() {
            start = password_list.len()
        }
        if end > password_list.len() {
            end = password_list.len()
        }
        let passwords: Vec<String> = password_list[start..end].into();
        let pdf = args.pdf.clone();
        let handle = thread::spawn(move || {
            for password in passwords {
                if try_password(&pdf, &password.trim()) {
                    println!("Found password: {}", &password.trim());
                    break;
                }
            }
        });
        threads.push(handle);
    }
    for handle in threads {
        handle.join();
    }
    println!("Done")
}
