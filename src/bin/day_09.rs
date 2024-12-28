use std::fs;

const BLANK: i32 = -1;

fn load_puzzle() -> Vec<i32> {
    let content = fs::read_to_string("./resources/day_09.txt").expect("Failed to read the file");

    content
        .chars()
        .filter_map(|c| c.to_digit(10).map(|d| d as i32))
        .collect()
}

fn main() {
    let puzzle = load_puzzle();

    let mut disk: Vec<i32> = Vec::new();
    let mut file_id: i32 = 0;

    // Build the "disk" based on the input puzzle.
    for (i, &value) in puzzle.iter().enumerate() {
        if i % 2 == 0 {
            disk.extend(vec![file_id; value as usize].iter());
            file_id += 1;
        } else {
            disk.extend(vec![BLANK; value as usize].iter());
        }
    }

    // Find all indices where the value is -1
    let blanks: Vec<usize> = disk
        .iter()
        .enumerate()
        .filter_map(|(i, &x)| if x == BLANK { Some(i) } else { None })
        .collect();

    // Process blanks and re-arrange the "disk".
    for &i in &blanks {
        while disk.last() == Some(&BLANK) {
            disk.pop();
        }
        if disk.len() <= i {
            break;
        }
        // Replace the blank spot with the last value from the disk (popped value).
        if let Some(value) = disk.pop() {
            disk[i] = value;
        }
    }

    let mut checksum: i128 = 0;
    for (i, &x) in disk.iter().enumerate() {
        checksum += x as i128 * i as i128;
    }
    println!("Checksum: {}", checksum);
}
