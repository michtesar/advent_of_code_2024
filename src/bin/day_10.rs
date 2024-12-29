use std::collections::HashSet;
use std::fs;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: usize,
    y: usize,
}

struct Points {
    items: HashSet<Point>,
}

impl Points {
    fn new() -> Self {
        Points {
            items: HashSet::new(),
        }
    }

    fn has_point(&self, point: Point) -> bool {
        self.items.contains(&point)
    }

    fn add_point(&mut self, point: Point) {
        self.items.insert(point);
    }

    fn length(&self) -> usize {
        self.items.len()
    }
}

fn load_map(file_path: &str) -> Vec<Vec<char>> {
    let content = fs::read_to_string(file_path).expect("Failed to read the file");
    content.lines().map(|line| line.chars().collect()).collect()
}

fn do_get_trailhead_score(map: &Vec<Vec<char>>, x: usize, y: usize, visited: &mut Points) {
    let current_char = map[y][x];
    if current_char == '9' {
        let point = Point { x, y };
        if !visited.has_point(point) {
            visited.add_point(point);
        }
        return;
    }

    // Check upwards
    if y > 0 && map[y - 1][x] == (current_char as u8 + 1) as char {
        do_get_trailhead_score(map, x, y - 1, visited);
    }

    // Check left
    if x > 0 && map[y][x - 1] == (current_char as u8 + 1) as char {
        do_get_trailhead_score(map, x - 1, y, visited);
    }

    // Check downwards
    if y + 1 < map.len() && map[y + 1][x] == (current_char as u8 + 1) as char {
        do_get_trailhead_score(map, x, y + 1, visited);
    }

    // Check right
    if x + 1 < map[y].len() && map[y][x + 1] == (current_char as u8 + 1) as char {
        do_get_trailhead_score(map, x + 1, y, visited);
    }
}

fn get_trailhead_score(map: &Vec<Vec<char>>, x: usize, y: usize) -> usize {
    let mut visited = Points::new();
    do_get_trailhead_score(map, x, y, &mut visited);
    visited.length()
}

fn main() {
    // Replace with the actual map file path
    let file_path = "./resources/day_10.txt";
    let map = load_map(file_path);

    let mut total_trailhead_score = 0;

    for y in 0..map.len() {
        for x in 0..map[y].len() {
            if map[y][x] == '0' {
                total_trailhead_score += get_trailhead_score(&map, x, y);
            }
        }
    }

    println!("Sum of scores of all trailheads: {}", total_trailhead_score);
}
