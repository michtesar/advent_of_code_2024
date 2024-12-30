use regex::Regex;

#[derive(Debug)]
struct Vector2i {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct Robot {
    position: Vector2i,
    velocity: Vector2i,
}

impl Robot {
    fn make_a_move(&mut self, grid: &Vector2i) {
        self.position.x = (self.position.x + self.velocity.x).rem_euclid(grid.x);
        self.position.y = (self.position.y + self.velocity.y).rem_euclid(grid.y);
    }
}

fn load_input() -> Vec<Robot> {
    let content = std::fs::read_to_string("./resources/day_14.txt").unwrap();
    let regex = Regex::new(r"p=(-?\d+),(-?\d+) v=(-?\d+),(-?\d+)").unwrap();
    let mut robots: Vec<Robot> = Vec::new();
    for line in content.lines() {
        let captures = regex.captures(line).expect("Invalid input");
        let px = captures[1].parse::<i32>().unwrap();
        let py = captures[2].parse::<i32>().unwrap();
        let vx = captures[3].parse::<i32>().unwrap();
        let vy = captures[4].parse::<i32>().unwrap();
        let robot = Robot {
            position: Vector2i { x: px, y: py },
            velocity: Vector2i { x: vx, y: vy },
        };
        robots.push(robot);
    }
    robots
}

fn simulate_for_seconds(grid: &Vector2i, robots: &mut Vec<Robot>, seconds: usize) {
    for _ in 0..seconds {
        for robot in robots.iter_mut() {
            robot.make_a_move(grid);
        }
    }
}

fn count_quadrants(grid: &Vector2i, robots: &[Robot]) -> [i32; 4] {
    let mut quadrants = [0; 4];
    let center_x = grid.x / 2;
    let center_y = grid.y / 2;

    for robot in robots {
        let x = robot.position.x;
        let y = robot.position.y;

        if x != center_x && y != center_y {
            if x < center_x && y < center_y {
                quadrants[0] += 1; // Top-left quadrant
            } else if x > center_x && y < center_y {
                quadrants[1] += 1; // Top-right quadrant
            } else if x < center_x && y > center_y {
                quadrants[2] += 1; // Bottom-left quadrant
            } else if x > center_x && y > center_y {
                quadrants[3] += 1; // Bottom-right quadrant
            }
        }
    }

    quadrants
}

fn main() {
    let grid_dimensions = Vector2i { x: 11, y: 7 };
    let mut robots = load_input();

    // Simulate robot movement for 100 seconds
    simulate_for_seconds(&grid_dimensions, &mut robots, 100);

    // Count robots in each quadrant
    let quadrants = count_quadrants(&grid_dimensions, &robots);
    println!("Quadrants: {:?}", quadrants);

    // Calculate safety factor
    let safety_factor: i32 = quadrants.iter().product();
    println!("Safety Factor: {}", safety_factor);
}