use std::collections::HashMap;

/// Tailor a CV by selecting and reordering skill blocks
pub fn tailor_cv(cv: &str, profile: &str) -> String {
    let cv_blocks = parse_cv(cv);
    let priorities: Vec<&str> = profile.lines().map(|l| l.trim()).collect();

    let mut output = String::new();

    for skill in priorities {
        if let Some(blocks) = cv_blocks.get(skill) {
            for block in blocks {
                output.push_str(block);
                output.push_str("\n\n");
            }
        }
    }

    output
}

fn parse_cv(input: &str) -> HashMap<String, Vec<String>> {
    let mut map = HashMap::new();
    let mut current_skill: Option<String> = None;
    let mut buffer = Vec::new();

    for line in input.lines() {
        if let Some(skill) = parse_skill(line) {
            if let Some(s) = current_skill.take() {
                map.entry(s).or_insert(Vec::new()).push(buffer.join("\n"));
                buffer.clear();
            }
            current_skill = Some(skill);
        } else {
            buffer.push(line.to_string());
        }
    }

    if let Some(s) = current_skill {
        map.entry(s).or_insert(Vec::new()).push(buffer.join("\n"));
    }

    map
}

fn parse_skill(line: &str) -> Option<String> {
    if line.starts_with("[skill:") && line.ends_with(']') {
        Some(line[7..line.len() - 1].to_lowercase())
    } else {
        None
    }
}
