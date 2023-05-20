use std::fs;

pub fn read_input_file() -> String {
    fs::read_to_string("input.txt").expect("Error reading file")
}

pub fn part1(input: &String) -> i32 {
    let input = input.trim();

    let mut sum = 0;

    for (i, c) in input.chars().enumerate() {
        if c == input.chars().nth((i + 1) % input.len()).unwrap() {
            sum += c.to_digit(10).unwrap() as i32;
        }
    }

    sum
}

pub fn part2(input: &String) -> i32 {
    let input = input.trim();

    let mut sum = 0;

    for (i, c) in input.chars().enumerate() {
        if c == input.chars().nth((i + input.len() / 2) % input.len()).unwrap() {
            sum += c.to_digit(10).unwrap() as i32;
        }
    }

    sum
}

