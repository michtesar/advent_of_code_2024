use std::collections::{HashMap, HashSet};
use std::fs;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string("./resources/day_05.txt")?;
    let lines: Vec<&str> = content.lines().collect();

    let mut separator_index = 0;
    for i in 0..lines.len() {
        if lines[i].contains(',') {
            separator_index = i;
            break;
        }
    }

    let rules_input = &lines[..separator_index];
    let updates_input = &lines[separator_index..];

    let rules = parse_rules(rules_input)?;
    let updates = parse_updates(updates_input)?;

    let valid_middle_sum = process_updates(&updates, &rules);
    println!("Sum of middle pages of correctly ordered updates: {}", valid_middle_sum);

    Ok(())
}

fn parse_rules(rules_input: &[&str]) -> Result<HashMap<i32, HashSet<i32>>, Box<dyn Error>> {
    let mut rules = HashMap::new();

    for rule in rules_input {
        let parts: Vec<&str> = rule.split('|').collect();
        if parts.len() == 2 {
            let before = parts[0].parse::<i32>()?;
            let after = parts[1].parse::<i32>()?;
            rules.entry(before).or_insert_with(HashSet::new).insert(after);
        } else {
            println!("Warning: skipping malformed rule: {}", rule);
        }
    }

    Ok(rules)
}

fn parse_updates(updates_input: &[&str]) -> Result<Vec<Vec<i32>>, Box<dyn Error>> {
    let updates: Vec<Vec<i32>> = updates_input
        .iter()
        .map(|line| {
            line.split(',')
                .filter_map(|s| s.trim().parse::<i32>().ok())
                .collect()
        })
        .collect();

    Ok(updates)
}

fn process_updates(updates: &[Vec<i32>], rules: &HashMap<i32, HashSet<i32>>) -> i32 {
    updates.iter().filter_map(|update| {
        if is_valid_order(update, rules) {
            Some(update[update.len() / 2])
        } else {
            None
        }
    }).sum()
}

fn is_valid_order(update: &[i32], rules: &HashMap<i32, HashSet<i32>>) -> bool {
    let mut index_map = HashMap::new();

    for (i, &page) in update.iter().enumerate() {
        index_map.insert(page, i);
    }

    for (&rule_before, must_be_after) in rules {
        if let Some(&index_before) = index_map.get(&rule_before) {
            for &rule_after in must_be_after {
                if let Some(&index_after) = index_map.get(&rule_after) {
                    if index_before >= index_after {
                        return false;
                    }
                }
            }
        }
    }

    true
}