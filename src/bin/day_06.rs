use std::fs;

fn read_map() -> Vec<Vec<char>> {
    let file_content = fs::read_to_string("./resources/day_06.txt").unwrap();
    let lines = file_content.split("\n").collect::<Vec<&str>>();
    let mut map: Vec<Vec<char>> = vec![];
    for line in lines {
        map.push(line.chars().collect::<Vec<char>>());
    }
    map
}

fn print_map(map: &Vec<Vec<char>>) {
    for line in map {
        println!("{}", line.iter().collect::<String>());
    }
}

#[derive(Clone)]
struct Position {
    x: i32,
    y: i32,
}

fn find_guard(map: &Vec<Vec<char>>, directions: &Vec<char>) -> Option<Position> {
    for i in 0..map.len() {
        for j in 0..map[i].len() {
            let char = map[i][j];
            if directions.contains(&char) {
                return Some(Position {
                    x: i as i32,
                    y: j as i32,
                });
            }
        }
    }
    None
}

fn get_char(map: &Vec<Vec<char>>, position: &Position) -> char {
    map[position.x as usize][position.y as usize]
}

fn is_outside(map: &Vec<Vec<char>>, position: &Position) -> bool {
    position.x < 0
        || position.x >= map.len() as i32
        || position.y < 0
        || position.y >= map[0].len() as i32
}

fn move_guard(
    map: &mut Vec<Vec<char>>,
    guard: &Position,
    new_position: &Position,
    guard_direction: char,
) {
    // Check if the new position is not colliding somewhere
    if map[new_position.x as usize][new_position.y as usize] == '#' {
        if guard_direction == '^' {
            map[guard.x as usize][guard.y as usize] = '>';
        } else if guard_direction == '>' {
            map[guard.x as usize][guard.y as usize] = 'v';
        } else if guard_direction == 'v' {
            map[guard.x as usize][guard.y as usize] = '<';
        } else if guard_direction == '<' {
            map[guard.x as usize][guard.y as usize] = '^';
        }
        return;
    }

    // Mark visited place
    map[guard.x as usize][guard.y as usize] = 'X';
    // Move to the new location
    map[new_position.x as usize][new_position.y as usize] = guard_direction;
}

fn get_visited_places(map: &Vec<Vec<char>>) -> i32 {
    let mut visited_places: i32 = 0;
    for i in 0..map.len() {
        for j in 0..map[i].len() {
            if map[i][j] == 'X' {
                visited_places += 1;
            }
        }
    }
    visited_places
}

fn main() {
    let mut map = read_map();
    let directions = vec!['^', '>', 'v', '<'];
    loop {
        let guard = find_guard(&map, &directions).expect("No guard found");
        let guard_direction = get_char(&map, &guard);
        let mut new_position = guard.clone();

        if guard_direction == '^' {
            new_position.x -= 1;
        } else if guard_direction == 'v' {
            new_position.x += 1;
        } else if guard_direction == '>' {
            new_position.y += 1;
        } else if guard_direction == '<' {
            new_position.y -= 1;
        }

        if is_outside(&map, &new_position) {
            println!("Guard is outside of the map");
            map[guard.x as usize][guard.y as usize] = 'X';
            print_map(&map);
            println!(
                "Guard visited {} specific positions",
                get_visited_places(&map)
            );
            break;
        }
        move_guard(&mut map, &guard, &new_position, guard_direction);
    }
}
