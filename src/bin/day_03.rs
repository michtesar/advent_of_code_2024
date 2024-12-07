use regex::Regex;
use std::fs;

fn read_data() -> String {
    let content =
        fs::read_to_string("./resources/day_03.txt").expect("Cannot read the source file");
    content
}
fn main() {
    let instructions = read_data();
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    let mut sum = 0;
    for cap in re.captures_iter(&instructions) {
        println!("Matched: mul({}, {})", &cap[1], &cap[2]);
        let a = &cap[1].parse::<i32>().unwrap();
        let b = &cap[2].parse::<i32>().unwrap();
        let mul = a * b;
        sum += mul;
    }
    println!("Sum: {}", sum);
}
