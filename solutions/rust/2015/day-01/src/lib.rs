use std::fs;

pub fn read_input_file() -> String {
    fs::read_to_string("input.txt").expect("Error reading file")
}

pub fn separate_input(input: &str) -> Vec<i32> {
    input.split("\n").map(|i| i.parse::<i32>().unwrap()).collect()
}

pub fn part1(input: &String) -> i32 {
    let mut floor: i32 = 0;
    for c in input.chars() {
        match c {
            '(' => floor += 1,
            ')' => floor -= 1,
            _ => (),
        }
    }
    floor
}

pub fn part2(input: &String) -> i32 {
    let mut floor: i32 = 0;
    for (i, c) in input.chars().enumerate() {
        match c {
            '(' => floor += 1,
            ')' => floor -= 1,
            _ => (),
        }
        if floor == -1 {
            return (i + 1) as i32;
        }
    }
    -1
}
