use anyhow::{Context, Result};
use std::collections::HashMap;
use std::env;
use std::fs;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        eprintln!("Usage: cv-t <cv-file> <profile-file>");
        // Return Ok(()) to exit gracefully, or an error if you prefer strict failure.
        // For CLI tools, printing usage and exiting non-zero is common, but let's be clean.
        std::process::exit(1);
    }

    let cv_path = &args[1];
    let profile_path = &args[2];

    let cv_content = fs::read_to_string(cv_path)
        .with_context(|| format!("Failed to read CV file: {}", cv_path))?;
    
    let profile_content = fs::read_to_string(profile_path)
        .with_context(|| format!("Failed to read profile file: {}", profile_path))?;

    let sections = parse_cv(&cv_content);
    let profile = parse_profile(&profile_content);

    let output = build_output(&sections, &profile);
    print!("{}", output);

    Ok(())
}

fn parse_cv(input: &str) -> HashMap<String, Vec<String>> {
    let mut sections: HashMap<String, Vec<String>> = HashMap::new();
    let mut current_skill: Option<String> = None;

    for line in input.lines() {
        if let Some(skill) = parse_skill_tag(line) {
            current_skill = Some(skill.clone());
            sections.entry(skill).or_insert_with(Vec::new);
        } else if let Some(skill) = &current_skill {
            // Only add lines if we are currently inside a skill section
            sections.get_mut(skill).expect("Skill key must exist").push(line.to_string());
        }
    }

    sections
}

fn parse_skill_tag(line: &str) -> Option<String> {
    let line = line.trim();
    if line.starts_with("[skill:") && line.ends_with(']') {
        // Safer slicing: skip "[skill:" (7 chars) and truncate last char "]"
        if line.len() > 8 {
             let skill = &line[7..line.len() - 1];
             return Some(skill.to_string());
        }
    }
    None
}

fn parse_profile(input: &str) -> Vec<String> {
    input
        .lines()
        .map(|l| l.trim())
        .filter(|l| !l.is_empty())
        .map(String::from)
        .collect()
}

fn build_output(
    sections: &HashMap<String, Vec<String>>,
    profile: &[String],
) -> String {
    let mut output = String::new();

    for skill in profile {
        if let Some(lines) = sections.get(skill) {
            // Add the header tag back to preserve structure if needed, 
            // or just the content? The user said "Outputs a job-oriented CV version without modifying the original data"
            // The original had [skill:Name]. Let's keep it to maintain structure.
            output.push_str(&format!("[skill:{}]\n", skill));
            for line in lines {
                output.push_str(line);
                output.push('\n');
            }
            // Add a newline separators between sections for readability
            output.push('\n'); 
        }
    }

    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_skill_tag() {
        assert_eq!(parse_skill_tag("[skill:Rust]"), Some("Rust".to_string()));
        assert_eq!(parse_skill_tag(" [skill:Python] "), Some("Python".to_string()));
        assert_eq!(parse_skill_tag("NoTag"), None);
        assert_eq!(parse_skill_tag("[skill:]"), None); // Empty skill name
    }
}
