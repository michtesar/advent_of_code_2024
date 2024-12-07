use std::fs;

fn is_report_safe(report: &[i32]) -> bool {
    is_consistent(&*Vec::from(report)) && is_level_safe(&*Vec::from(report))
}

fn is_level_safe(report: &[i32]) -> bool {
    for i in 0..report.len() - 1 {
        let diff = (report[i] - report[i + 1]).abs();
        if diff > 3 || diff == 0 {
            return false;
        }
    }
    true
}

fn is_consistent(report: &[i32]) -> bool {
    let mut change = 0;
    let mut initial_direction = report[0] - report[1] > 0;
    for i in 0..report.len() - 1 {
        let level_status = report[i] - report[i + 1] > 0;
        if level_status != initial_direction {
            change += 1;
            initial_direction = !initial_direction;
        }
    }
    println!("Change: {}", change);
    change < 2
}
fn load_data() -> Vec<Vec<i32>> {
    let contents =
        fs::read_to_string("./resources/day_02.txt").expect("Cannot read the source file");
    let mut reports: Vec<Vec<i32>> = vec![];
    for line in contents.lines() {
        let report: Vec<i32> = line
            .split_whitespace()
            .map(|num| num.parse::<i32>().unwrap())
            .collect();
        reports.push(report);
    }
    reports
}

fn main() {
    let reports = load_data();
    let mut counter = 0;
    for report in reports.iter() {
        if is_report_safe(&report) {
            counter += 1;
        }
    }
    println!("Number of reports that are safe: {}", counter);
}
