use std::fs;

fn main() {
    let file_content = fs::read_to_string("./resources/day_07.txt").unwrap();
    let calibrations = file_content.split("\n").collect::<Vec<&str>>();

    for calibration in calibrations {
        println!("Calibration sequence: {}", calibration);
        let calibration_sum: i32 = calibration.split(":").next().unwrap().parse::<i32>().unwrap();
        println!("Calibration sum: {:?}", calibration_sum);
        let calibration_numbers: Vec<_> = calibration.split(":").last().unwrap().split(" ").skip(1).map(|x| x.parse::<i32>().unwrap()).collect();
        println!("Calibration numbers: {:?}", calibration_numbers);
    }
}