use regex::Regex;
use std::collections::{BTreeMap, HashMap, HashSet};
use std::fs::read_to_string;

fn main() {
    let data = read("input.txt");
    let alergens = find_ingredients_with_alergen(&data);
    println!(
        "part1 solution: {:?}",
        find_no_alergen_ingredients_appearance(&data, &alergens)
    );
    println!(
        "part2 solution: {}",
        find_canonical_dangerous_list(&alergens)
    );
}

fn find_no_alergen_ingredients_appearance(
    data: &[Food],
    alergens: &BTreeMap<String, String>,
) -> usize {
    let alergen_ingredients = alergens.values().cloned().collect::<HashSet<String>>();
    data.iter()
        .map(|f| {
            f.ingredients
                .iter()
                .filter(|&i| !alergen_ingredients.contains(i))
                .count()
        })
        .sum()
}

fn find_canonical_dangerous_list(alergens: &BTreeMap<String, String>) -> String {
    alergens
        .values()
        .cloned()
        .collect::<Vec<String>>()
        .join(",")
}

fn find_ingredients_with_alergen(data: &[Food]) -> BTreeMap<String, String> {
    let mut alergens = HashMap::new();
    for food in data {
        for al in food.alergens.iter() {
            let potential_ingredient = alergens.entry(al.to_string()).or_insert(HashSet::new());
            if potential_ingredient.is_empty() {
                potential_ingredient.extend(food.ingredients.iter().cloned());
            } else {
                *potential_ingredient = potential_ingredient
                    .intersection(&food.ingredients)
                    .cloned()
                    .collect();
            }
        }
    }

    let mut processed = BTreeMap::new();
    while !alergens.is_empty() {
        let (curr_key, curr_vals) = alergens.iter().find(|(_k, v)| v.len() == 1).unwrap();
        let curr_val = curr_vals.iter().next().unwrap().clone();
        processed.insert(curr_key.clone(), curr_val.clone());
        for (_target_key, target_vals) in alergens.iter_mut() {
            target_vals.remove(&curr_val);
        }
        alergens.retain(|_k, v| !v.is_empty());
    }
    processed
}

fn read(filename: &str) -> Vec<Food> {
    let re = Regex::new(r"(.+)\s\(contains\s(.+,?\s?)+\)").unwrap();
    read_to_string(filename)
        .expect("Failed to read file")
        .lines()
        .map(|l| {
            let caps = re.captures(l).unwrap();
            let ingredients = caps[1]
                .split_whitespace()
                .map(|s| s.to_string())
                .collect::<HashSet<String>>();
            let alergens = caps[2]
                .split(", ")
                .map(|s| s.to_string())
                .collect::<HashSet<String>>();
            Food {
                ingredients,
                alergens,
            }
        })
        .collect()
}

struct Food {
    ingredients: HashSet<String>,
    alergens: HashSet<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let data = read("test-input.txt");
        let alergens = find_ingredients_with_alergen(&data);
        assert_eq!(find_no_alergen_ingredients_appearance(&data, &alergens), 5);
        assert_eq!(
            find_canonical_dangerous_list(&alergens),
            String::from("mxmxvkd,sqjhc,fvjkl")
        );
    }
}
