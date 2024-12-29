use std::collections::{HashSet, VecDeque};
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    // Read the input into a 2D vector
    let mut inp: Vec<Vec<char>> = Vec::new();
    if let Ok(lines) = read_lines("./resources/day_12.txt") {
        for line in lines {
            if let Ok(row) = line {
                inp.push(row.chars().collect());
            }
        }
    }

    let num_rows = inp.len();
    let num_cols = if num_rows > 0 { inp[0].len() } else { 0 };

    // Closure to check bounds
    let in_bounds = |(r, c): (usize, usize)| -> bool { r < num_rows && c < num_cols };

    // Closure to get the plant type at a given cell
    let get_plant = |(r, c): (usize, usize)| -> char { inp[r][c] };

    // Closure to get neighbors of a cell
    let get_neighbors = |(r, c): (usize, usize)| -> Vec<(usize, usize)> {
        let deltas = [(-1_isize, 0), (0, 1), (1, 0), (0, -1)]; // NESW
        deltas
            .iter()
            .filter_map(|&(dr, dc)| {
                let new_r = r as isize + dr;
                let new_c = c as isize + dc;
                if new_r >= 0 && new_c >= 0 {
                    let new_rc = (new_r as usize, new_c as usize);
                    if in_bounds(new_rc) {
                        Some(new_rc)
                    } else {
                        None
                    }
                } else {
                    None
                }
            })
            .collect()
    };

    // Closure to get neighboring cells with the same plant type
    let get_plant_neighbors = |rc: (usize, usize)| -> Vec<(usize, usize)> {
        get_neighbors(rc)
            .into_iter()
            .filter(|&n| get_plant(n) == get_plant(rc))
            .collect()
    };

    // BFS to determine a connected region
    let get_region = |start: (usize, usize)| -> HashSet<(usize, usize)> {
        let mut visited = HashSet::new();
        let mut region = HashSet::new();
        let mut queue = VecDeque::new();
        queue.push_back(start);

        while let Some(node) = queue.pop_front() {
            if !visited.contains(&node) {
                visited.insert(node);
                region.insert(node);

                let neighbors = get_plant_neighbors(node);
                for neighbor in neighbors {
                    if !visited.contains(&neighbor) {
                        queue.push_back(neighbor);
                    }
                }
            }
        }

        region
    };

    // Calculate the perimeter of a region
    let calc_perimeter = |region: &HashSet<(usize, usize)>| -> usize {
        let mut perimeter = 0;

        for &rc in region {
            let plant = get_plant(rc);
            let neighbors = get_neighbors(rc);
            // Add boundary to perimeter
            perimeter += 4 - neighbors.len();
            for n in neighbors {
                if get_plant(n) != plant {
                    perimeter += 1;
                }
            }
        }

        perimeter
    };

    let mut regions: Vec<HashSet<(usize, usize)>> = Vec::new();
    let mut visited_cells: HashSet<(usize, usize)> = HashSet::new();

    for r in 0..num_rows {
        for c in 0..num_cols {
            let rc = (r, c);
            if !visited_cells.contains(&rc) {
                let region = get_region(rc);
                visited_cells.extend(&region);
                regions.push(region);
            }
        }
    }

    let mut total_price = 0;

    for region in &regions {
        let _ = get_plant(*region.iter().next().unwrap());
        let area = region.len();
        let perimeter = calc_perimeter(region);
        let price = area * perimeter;
        total_price += price;
    }

    println!("{}", total_price);
}

// Helper function to read lines from a file
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
