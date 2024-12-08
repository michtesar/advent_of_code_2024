use std::collections::HashMap;
use std::fs;

#[derive(Eq, Hash, PartialEq, Debug)]
enum Direction {
    East,
    West,
    South,
    North,
    NorthEast,
    NorthWest,
    SouthEast,
    SouthWest,
}

#[derive(Eq, PartialEq)]
struct Dimension {
    width: i32,
    height: i32,
}

#[derive(Debug)]
struct Coordinate {
    x: i32,
    y: i32,
}

fn get_indexes(
    col: i32,
    row: i32,
    word_length: i32,
    dimension: Dimension,
) -> HashMap<Direction, Vec<Coordinate>> {
    let mut indexes: HashMap<Direction, Vec<Coordinate>> = HashMap::new();

    // East
    if col + word_length <= dimension.width {
        let mut coordinates = Vec::new();
        for i in 0..word_length {
            coordinates.push(Coordinate { x: col + i, y: row });
        }
        indexes.insert(Direction::East, coordinates);
    }

    // West
    if col - word_length >= -1 {
        let mut coordinates = Vec::new();
        for i in 0..word_length {
            coordinates.push(Coordinate { x: col - i, y: row });
        }
        indexes.insert(Direction::West, coordinates);
    }

    // South
    if row + word_length <= dimension.height {
        let mut coordinates = Vec::new();
        for i in 0..word_length {
            coordinates.push(Coordinate { x: col, y: row + i });
        }
        indexes.insert(Direction::South, coordinates);
    }

    // North
    if row - word_length >= -1 {
        let mut coordinates = Vec::new();
        for i in 0..word_length {
            coordinates.push(Coordinate { x: col, y: row - i });
        }
        indexes.insert(Direction::North, coordinates);
    }

    // NorthEast
    if col + word_length <= dimension.width && row - word_length >= -1 {
        let mut coordinates = Vec::new();
        for i in 0..word_length {
            coordinates.push(Coordinate {
                x: col + i,
                y: row - i,
            });
        }
        indexes.insert(Direction::NorthEast, coordinates);
    }

    // NorthWest
    if col - word_length >= -1 && row - word_length >= -1 {
        let mut coordinates = Vec::new();
        for i in 0..word_length {
            coordinates.push(Coordinate {
                x: col - i,
                y: row - i,
            });
        }
        indexes.insert(Direction::NorthWest, coordinates);
    }

    // SouthEast
    if col + word_length <= dimension.width && row + word_length <= dimension.height {
        let mut coordinates = Vec::new();
        for i in 0..word_length {
            coordinates.push(Coordinate {
                x: col + i,
                y: row + i,
            });
        }
        indexes.insert(Direction::SouthEast, coordinates);
    }

    // SouthWest
    if col - word_length >= -1 && row + word_length <= dimension.height {
        let mut coordinates = Vec::new();
        for i in 0..word_length {
            coordinates.push(Coordinate {
                x: col - i,
                y: row + i,
            });
        }
        indexes.insert(Direction::SouthWest, coordinates);
    }
    indexes
}

fn get_word_from_indexes(grid: &Vec<Vec<char>>, coordinates: &Vec<Coordinate>) -> String {
    let mut word: String = String::new();
    for coordinate in coordinates {
        word.push(grid[coordinate.y as usize][coordinate.x as usize]);
    }
    word.to_lowercase()
}

fn load_puzzle_grid() -> Vec<Vec<char>> {
    let content =
        fs::read_to_string("./resources/day_04.txt").expect("Cannot read the source file");
    let mut rows: Vec<Vec<char>> = vec![];
    for line in content.lines() {
        let row: Vec<char> = line.chars().collect();
        rows.push(row);
    }
    rows
}

fn main() {
    let puzzle_grid = load_puzzle_grid();
    // let puzzle_grid = [
    //     ["M", "M", "M", "S", "X", "X", "M", "A", "S", "M"],
    //     ["M", "S", "A", "M", "X", "M", "S", "M", "S", "A"],
    //     ["A", "M", "X", "S", "X", "M", "A", "A", "M", "M"],
    //     ["M", "S", "A", "M", "A", "S", "M", "S", "M", "X"],
    //     ["X", "M", "A", "S", "A", "M", "X", "A", "M", "M"],
    //     ["X", "X", "A", "M", "M", "X", "X", "A", "M", "A"],
    //     ["S", "M", "S", "M", "S", "A", "S", "X", "S", "S"],
    //     ["S", "A", "X", "A", "M", "A", "S", "A", "A", "A"],
    //     ["M", "A", "M", "M", "M", "X", "M", "M", "M", "M"],
    //     ["M", "X", "M", "X", "A", "X", "M", "A", "S", "X"],
    // ];
    let search_word = "XMAS".to_lowercase();
    let search_word_length = search_word.len();

    let mut count = 0;
    for row_index in 0..puzzle_grid.len() {
        for col_index in 0..puzzle_grid[0].len() {
            let indexes = get_indexes(
                col_index as i32,
                row_index as i32,
                search_word_length as i32,
                Dimension {
                    width: puzzle_grid[0].len() as i32,
                    height: puzzle_grid.len() as i32,
                },
            );

            for (_, coords) in &indexes {
                let word = get_word_from_indexes(&puzzle_grid, coords);
                if word == search_word {
                    count += 1;
                }
            }
        }
    }
    println!("Found {} words", count);
}
