use std::fs;

fn load_initial_arrangement() -> Vec<i64> {
    let content = fs::read_to_string("./resources/day_11.txt").expect("Failed to read the file");
    content.split(" ").map(|c| c.parse().unwrap()).collect()
}

fn count_digits(mut num: i64) -> usize {
    if num == 0 {
        return 1;
    }
    num = num.abs();
    let mut count = 0;

    while num > 0 {
        num /= 10;
        count += 1;
    }

    count
}

fn split_number(num: i64) -> (i64, i64) {
    let chars = num.to_string().chars().collect::<Vec<char>>();
    let (first_half, second_half) = chars.split_at(chars.len() / 2);
    (
        first_half
            .iter()
            .collect::<String>()
            .parse::<i64>()
            .unwrap(),
        second_half
            .iter()
            .collect::<String>()
            .parse::<i64>()
            .unwrap(),
    )
}
fn blink(arrangement: &mut Vec<i64>, n: i32) -> Vec<i64> {
    for _ in 0..n {
        let mut new_arrangement = Vec::new();
        for stone in &mut *arrangement {
            // If the stone is 0, set it to 1
            if *stone == 0 {
                new_arrangement.push(1);
            }
            // If the stone has even digits, split it in half
            else if count_digits(*stone) % 2 == 0 {
                let (first_digit, rest) = split_number(*stone);
                new_arrangement.push(first_digit);
                new_arrangement.push(rest);
            }
            // If none of the rules applies multiply the stone by 2048
            else {
                new_arrangement.push(*stone * 2024);
            }
        }
        *arrangement = new_arrangement;
    }
    arrangement.clone()
}

fn main() {
    let mut initial_arrangement = load_initial_arrangement();
    // println!("Initial arrangement: {:?}", initial_arrangement);
    let state = blink(&mut initial_arrangement, 25);
    // println!("State after 1 blinks: {:?}", state);
    println!("Number of stones: {}", state.len());
}
