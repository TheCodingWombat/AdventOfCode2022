use std::fs;

fn main() {

    let input: String = fs::read_to_string("src/input.txt").unwrap();
    let mut lines_iter = input.lines().into_iter();

    let mut calories_per_fairy: Vec::<u32> = Vec::new();
    let mut cal_accumulator: u32 = 0;

    for line in lines_iter {
        if line == "" {
            calories_per_fairy.push(cal_accumulator);
            cal_accumulator = 0;
        } else {
            cal_accumulator += line.parse::<u32>().expect("Input should always have integers");
        }
    }

    let max_cal = calories_per_fairy.iter().max().unwrap();

    println!("Total: {:?}", max_cal);
}
