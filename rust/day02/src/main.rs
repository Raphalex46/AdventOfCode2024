use std::io::{self, BufRead};

fn solve_p1(reports: Vec<Vec<i32>>) -> i32 {
    let mut safe_reports = 0;
    for report in reports {
        let sign = (report[0] - report[1]).signum();
        let mut is_safe = true;
        for i in 0..(report.len() - 1) {
            let diff = report[i] - report[i + 1];
            if diff.abs() > 3 || diff.abs() < 1 || diff.signum() != sign {
                is_safe = false;
            }
        }
        if is_safe {
            safe_reports += 1;
        }
    }
    safe_reports
}

fn solve_p2(reports: Vec<Vec<i32>>) -> i32 {
    let mut safe_reports = 0;
    for report in reports {
        let sign = (report[0] - report[1]).signum();
        let mut is_safe = true;
        for i in 0..(report.len() - 1) {
            let diff = report[i] - report[i + 1];
            if diff.abs() > 3 || diff.abs() < 1 || diff.signum() != sign {
                is_safe = false;
            }
        }
        if !is_safe {
            for bad_level in 0..report.len() {
                let mut alt_report = report.clone();
                let mut alt_is_safe = true;
                alt_report.remove(bad_level);
                let sign = (alt_report[0] - alt_report[1]).signum();
                for i in 0..(alt_report.len() - 1) {
                    let diff = alt_report[i] - alt_report[i + 1];
                    if diff.abs() > 3 || diff.abs() < 1 || diff.signum() != sign {
                        alt_is_safe = false;
                    }
                }
                if alt_is_safe {
                    is_safe = true;
                }
            }
        }
        if is_safe {
            safe_reports += 1;
        }
    }
    safe_reports
}

fn parse_input() -> Vec<Vec<i32>> {
    let mut reports = Vec::new();
    for line_res in io::stdin().lock().lines() {
        let mut report = Vec::new();
        let line = line_res.expect("Failed to read input");
        for word in line.split_whitespace() {
            report.push(
                word.parse::<i32>()
                    .expect("Failed to parse integer from input"),
            );
        }
        reports.push(report);
    }
    reports
}

fn main() {
    let reports = parse_input();
    println!("Result for problem 1: {}", solve_p1(reports.clone()));
    println!("Result for problem 2: {}", solve_p2(reports));
}
