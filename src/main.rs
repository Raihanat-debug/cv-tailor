use std::env;
use std::fs;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    let mut cv_path = String::new();
    let mut profile_path = String::new();
    let mut out_path = String::new();
    
    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--cv" => {
                i += 1;
                if i < args.len() {
                    cv_path = args[i].clone();
                }
            }
            "--profile" => {
                i += 1;
                if i < args.len() {
                    profile_path = args[i].clone();
                }
            }
            "--out" => {
                i += 1;
                if i < args.len() {
                    out_path = args[i].clone();
                }
            }
            _ => {}
        }
        i += 1;
    }
    
    if cv_path.is_empty() || profile_path.is_empty() || out_path.is_empty() {
        eprintln!("Usage: cv-t --cv <cv_file> --profile <profile_file> --out <output_file>");
        process::exit(1);
    }

    let cv_content = read_file(&cv_path);
    let profile_content = read_file(&profile_path);

    let tailored = tailor_cv(&cv_content, &profile_content);

    fs::write(&out_path, tailored).unwrap_or_else(|_| {
        eprintln!("Failed to write output file");
        process::exit(1);
    });
}

fn read_file(path: &str) -> String {
    fs::read_to_string(path).unwrap_or_else(|_| {
        eprintln!("Failed to read file: {}", path);
        process::exit(1);
    })
}

fn tailor_cv(cv: &str, profile: &str) -> String {
    let priorities: Vec<&str> = profile.lines().collect();
    let mut output = String::new();

    for p in priorities {
        for block in cv.split("\n\n") {
            if block.to_lowercase().contains(&p.to_lowercase()) {
                output.push_str(block);
                output.push_str("\n\n");
            }
        }
    }

    output
}
