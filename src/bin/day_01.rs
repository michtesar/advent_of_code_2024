use std::fs;

fn main() {
    let contents =
        fs::read_to_string("./resources/day_01.txt").expect("Cannot read the source file");
    let mut coordinates_a: Vec<i32> = vec![];
    let mut coordinates_b: Vec<i32> = vec![];
    for line in contents.lines() {
        let split = line.split("   ").collect::<Vec<&str>>();
        if !split.is_empty() {
            coordinates_a.push(split[0].parse::<i32>().unwrap());
            coordinates_b.push(split[1].parse::<i32>().unwrap());
        }
    }
    coordinates_a.sort();
    coordinates_b.sort();

    let mut diffs: Vec<i32> = vec![];
    for i in 0..coordinates_a.len() {
        diffs.push((coordinates_a[i] - coordinates_b[i]).abs());
    }
    println!("Sum of differences: {}", diffs.iter().sum::<i32>());

    let mut similarity_searches: Vec<i32> = vec![];
    for i in 0..coordinates_a.len() {
        let coordinate = coordinates_a[i];
        let factor = coordinates_b.iter().filter(|&&x| x == coordinate).count() as i32;
        similarity_searches.push(coordinate * factor);
    }
    println!("Similarity searches: {}", similarity_searches.iter().sum::<i32>());
}
