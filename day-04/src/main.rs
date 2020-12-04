use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::prelude::*;
use std::ops::RangeInclusive;

fn main() {
    let fields: HashSet<String> = required_keys();
    let docs = read("input.txt");

    println!(
        "part1 solution: {}",
        validate_passports(&docs, &fields, &has_required_fields)
    );
    println!(
        "part2 solution: {}",
        validate_passports(&docs, &fields, &validate_strict)
    );
}

fn validate_passports(
    passports: &[HashMap<String, String>],
    required: &HashSet<String>,
    validation_fn: &dyn Fn(&HashMap<String, String>, &HashSet<String>) -> bool,
) -> usize {
    passports
        .iter()
        .filter(|p| validation_fn(p, required))
        .count()
}

fn required_keys() -> HashSet<String> {
    ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid", "cid"]
        .iter()
        .filter_map(|&id| {
            if id != "cid" {
                Some(id.to_string())
            } else {
                None
            }
        })
        .collect()
}

fn has_required_fields(passport: &HashMap<String, String>, required: &HashSet<String>) -> bool {
    required.iter().all(|f| passport.contains_key(f))
}

fn validate_strict(passport: &HashMap<String, String>, required: &HashSet<String>) -> bool {
    has_required_fields(passport, required)
        && passport.iter().all(|(key, val)| match key.as_str() {
            "byr" => validate_year(val, 1920..=2002),
            "iyr" => validate_year(val, 2010..=2020),
            "eyr" => validate_year(val, 2020..=2030),
            "hgt" => validate_height(val),
            "hcl" => validate_color_hex(val),
            "ecl" => validate_eye_color(val),
            "pid" => validate_number(val, 9),
            "cid" => true,
            _ => false,
        })
}

fn validate_year(input_str: &str, range: RangeInclusive<usize>) -> bool {
    validate_number(input_str, 4)
        && range.contains(&input_str.parse::<usize>().expect("Couldn't parse"))
}

fn validate_height(input_str: &str) -> bool {
    let valid_measurements = ["cm", "in"];
    let m_type = valid_measurements.iter().find(|&m| input_str.ends_with(m));
    if let Some(&measurement_type) = m_type {
        let range = match measurement_type {
            "cm" => Some(150..=193),
            "in" => Some(59..=76),
            _ => None,
        };
        if let Some(length_range) = range {
            let (len_str, _) = input_str.split_at(input_str.len() - measurement_type.len());
            if let Ok(val) = len_str.parse::<usize>() {
                return length_range.contains(&val);
            }
        }
    }
    false
}

fn validate_color_hex(input_str: &str) -> bool {
    input_str.len() == 7
        && input_str.starts_with('#')
        && input_str
            .chars()
            .skip(1)
            .all(|ch| matches!(ch, '0'..='9' | 'a'..='f'))
}

fn validate_eye_color(input_str: &str) -> bool {
    matches!(
        input_str,
        "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth"
    )
}

fn validate_number(input_str: &str, length: usize) -> bool {
    input_str.len() == length && input_str.chars().all(|ch| ch.is_ascii_digit())
}

fn read(filename: &str) -> Vec<HashMap<String, String>> {
    let mut file = File::open(filename).expect("File not found");
    let mut content = String::new();
    file.read_to_string(&mut content)
        .expect("Failed to read file content");

    let mut res = Vec::new();
    let mut map = HashMap::new();
    for s in content.split_terminator('\n') {
        if s.is_empty() {
            res.push(map.clone());
            map.clear();
            continue;
        }
        for chunk in s.split_whitespace() {
            let values = chunk.split(':').collect::<Vec<&str>>();
            map.insert(values[0].to_string(), values[1].to_string());
        }
    }
    res.push(map);
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test() {
        let docs = read("test-input.txt");
        let fields = required_keys();
        assert_eq!(validate_passports(&docs, &fields, &has_required_fields), 2);
    }

    #[test]
    fn part2_invalid_test() {
        let docs = read("test-input-invalid.txt");
        let fields = required_keys();
        assert_eq!(validate_passports(&docs, &fields, &validate_strict), 0);
    }

    #[test]
    fn part2_valid_test() {
        let docs = read("test-input-valid.txt");
        let fields = required_keys();
        assert_eq!(validate_passports(&docs, &fields, &validate_strict), 4);
    }
}
