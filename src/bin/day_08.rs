use std::collections::{HashMap, HashSet};
use std::fs;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Position {
    row: usize,
    col: usize,
}

fn main() {
    // Read input from file
    let input = fs::read_to_string("./resources/day_08.txt").expect("Failed to read input file").trim().to_string();

    let mut map = Vec::new();
    let mut all_antennas: HashMap<char, Vec<Position>> = HashMap::new();
    let mut all_antinodes: HashSet<String> = HashSet::new();

    // Process input
    let lines = input.lines();
    for (row, line) in lines.enumerate() {
        let line = line.trim().to_string();
        map.push(line.clone());

        for (col, symbol) in line.chars().enumerate() {
            if symbol == '.' {
                continue;
            }

            all_antennas
                .entry(symbol)
                .or_insert(Vec::new())
                .push(Position { row, col });
        }
    }

    // Find antinodes
    for (_symbol, antennas) in &all_antennas {
        find_antinodes(antennas, &mut all_antinodes, map[0].len(), map.len());
    }

    // Print the result
    println!("The answer is {}", all_antinodes.len());
}

fn find_antinodes(
    antennas: &[Position],
    all_antinodes: &mut HashSet<String>,
    width: usize,
    height: usize,
) {
    let len = antennas.len();
    for n in 0..len - 1 {
        for p in n + 1..len {
            let a = &antennas[n];
            let b = &antennas[p];
            find_antinodes_for(a, b, all_antinodes, width, height);
        }
    }
}

fn find_antinodes_for(
    a: &Position,
    b: &Position,
    all_antinodes: &mut HashSet<String>,
    width: usize,
    height: usize,
) {
    let delta_row = b.row as isize - a.row as isize;
    let delta_col = b.col as isize - a.col as isize;

    let positions = vec![
        Position {
            row: (a.row as isize - delta_row) as usize,
            col: (a.col as isize - delta_col) as usize,
        },
        Position {
            row: (b.row as isize + delta_row) as usize,
            col: (b.col as isize + delta_col) as usize,
        },
    ];

    for pos in positions {
        register_antinode(pos, all_antinodes, width, height);
    }
}

fn register_antinode(
    pos: Position,
    all_antinodes: &mut HashSet<String>,
    width: usize,
    height: usize,
) {
    if pos.row >= height || pos.col >= width {
        return;
    }

    let key = format!("{}:{}", pos.row, pos.col);
    all_antinodes.insert(key);
}