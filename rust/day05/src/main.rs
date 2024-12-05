use std::{
    cmp::Ordering,
    io::{self, BufRead},
};

type Update = Vec<i32>;

#[derive(Copy, Clone, Debug, PartialEq)]
struct Rule(i32, i32);

fn solve_p1(rules: Vec<Rule>, updates: Vec<Update>) -> i32 {
    let compare = |&a, &b| {
        if rules.contains(&Rule(b, a)) {
            false
        } else {
            true
        }
    };
    let mut sum = 0;
    for update in &updates {
        if update.is_sorted_by(compare) {
            sum += update[update.len() / 2];
        }
    }
    sum
}

fn solve_p2(rules: Vec<Rule>, updates: Vec<Update>) -> i32 {
    let compare_bool = |&a, &b| {
        if rules.contains(&Rule(b, a)) {
            false
        } else {
            true
        }
    };
    let mut sum = 0;
    for update in &updates {
        if !update.is_sorted_by(compare_bool) {
            let mut clone = update.clone();
            clone.sort_by(|&a, &b| {
                if rules.contains(&Rule(b, a)) {
                    Ordering::Greater
                } else {
                    Ordering::Less
                }
            });
            sum += clone[clone.len() / 2];
        }
    }
    sum
}

fn parse_input() -> (Vec<Rule>, Vec<Update>) {
    // Parse rules
    let mut rules = Vec::new();
    let mut updates = Vec::new();
    let lines: Vec<String> = io::stdin()
        .lock()
        .lines()
        .map(|l| l.expect("Failed to fetch line"))
        .collect();
    let mut i = 0;
    while !lines[i].is_empty() {
        let r: Vec<i32> = lines[i]
            .split("|")
            .map(|e| e.parse::<i32>().expect("Failed to parse integer"))
            .collect();
        rules.push(Rule(r[0], r[1]));
        i += 1;
    }
    i += 1;
    while i < lines.len() {
        updates.push(
            lines[i]
                .split(",")
                .map(|e| e.parse::<i32>().expect("Failed to parse integer"))
                .collect(),
        );
        i += 1;
    }
    (rules, updates)
}

fn main() {
    let (rules, updates) = parse_input();
    println!(
        "Result for problem 1: {}",
        solve_p1(rules.clone(), updates.clone())
    );
    println!("Result for problem 2: {}", solve_p2(rules, updates));
}
