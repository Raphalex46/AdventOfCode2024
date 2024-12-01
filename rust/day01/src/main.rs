use std::io::{self, BufRead};

fn solve_p1(mut v1: Vec<i32>, mut v2: Vec<i32>) -> i32 {
    // Sort lists
    v1.sort();
    v2.sort();
    let mut diff_v = Vec::new();
    for i in 0..v1.len() {
        // Compute the differences between each sorted element
        diff_v.push((v1[i] - v2[i]).abs());
    }
    // Finally, return the sum
    diff_v.iter().sum()
}

fn solve_p2(v1: Vec<i32>, v2: Vec<i32>) -> i32 {
    let mut similarity_score = 0;
    // For each number in l1...
    for num_v1 in v1 {
        let mut occurences = 0;
        // Compute the number of occurences of the number in l2
        for num_v2 in &v2 {
            if *num_v2 == num_v1 {
                occurences += 1;
            }
        }
        // Add occurences times the number to the similarity score
        similarity_score += num_v1 * occurences;
    }
    similarity_score
}

fn parse_input() -> (Vec<i32>, Vec<i32>) {
    let mut v1: Vec<i32> = Vec::new();
    let mut v2: Vec<i32> = Vec::new();
    // Parse the input
    for line_res in io::stdin().lock().lines() {
        let line = line_res.expect("Failed to read input");
        // Split each line...
        let splitted_input = line.split_whitespace();
        let mut v_ref = &mut v1;
        for word in splitted_input {
            // And fill the lists by adding the first number to the first list and the second
            // number to the second list.
            v_ref.push(
                word.parse::<i32>()
                    .expect("Failed to parse integer from input"),
            );
            v_ref = &mut v2;
        }
    }
    (v1, v2)
}

fn main() {
    let (v1, v2) = parse_input();
    println!("Result for problem 1: {}", solve_p1(v1.clone(), v2.clone()));
    println!("Result for problem 2: {}", solve_p2(v1, v2));
}
