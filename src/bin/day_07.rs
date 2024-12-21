use std::fs;

// Helper function to evaluate an expression with error handling for overflow
fn evaluate_expression(numbers: &[i64], operators: &[char]) -> Option<i64> {
    let mut result = numbers[0];
    for (i, &operator) in operators.iter().enumerate() {
        match operator {
            '+' => {
                result = result.checked_add(numbers[i + 1])?;
            }
            '*' => {
                result = result.checked_mul(numbers[i + 1])?;
            }
            _ => unreachable!("Invalid operator"),
        }
    }
    Some(result)
}

// Generate all possible operator combinations
fn generate_operator_combinations(len: usize) -> Vec<Vec<char>> {
    let mut combinations = Vec::new();
    let max = 1 << len; // 2^len combinations
    for i in 0..max {
        let mut combo = Vec::new();
        for j in 0..len {
            if (i & (1 << j)) != 0 {
                combo.push('*');
            } else {
                combo.push('+');
            }
        }
        combinations.push(combo);
    }
    combinations
}

fn main() {
    let file_content = fs::read_to_string("./resources/day_07.txt").unwrap();
    let calibrations = file_content.lines().filter(|line| !line.is_empty()).collect::<Vec<&str>>();

    let mut total_calibration_result = 0;

    for calibration in calibrations {
        let parts: Vec<&str> = calibration.split(":").collect();
        let calibration_sum: i64 = parts[0].trim().parse().unwrap();
        let calibration_numbers: Vec<i64> = parts[1]
            .trim()
            .split_whitespace()
            .map(|x| x.parse::<i64>().unwrap())
            .collect();

        let num_count = calibration_numbers.len();
        if num_count < 2 {
            continue; // Ignore invalid cases
        }

        let operator_combinations = generate_operator_combinations(num_count - 1);

        let mut found_valid = false;

        for operators in operator_combinations {
            if let Some(result) = evaluate_expression(&calibration_numbers, &operators) {
                if result == calibration_sum {
                    found_valid = true;
                    break;
                }
            }
        }

        if found_valid {
            total_calibration_result += calibration_sum;
        }
    }

    println!("Total Calibration Result: {}", total_calibration_result);
}