use clap::Parser;
use std::fs;
use std::process;

use cv_tailor::tailor_cv;

#[derive(Parser)]
#[command(author, version, about)]
struct Args {
    /// Path to CV file
    #[arg(long)]
    cv: String,

    /// Path to job profile file
    #[arg(long)]
    profile: String,

    /// Output file path
    #[arg(long)]
    out: String,
}

fn main() {
    let args = Args::parse();

    let cv_content = read_file(&args.cv);
    let profile_content = read_file(&args.profile);

    let tailored = tailor_cv(&cv_content, &profile_content);

    fs::write(&args.out, tailored).unwrap_or_else(|_| {
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
