use std::fs;

pub fn read_input_file() -> String {
    fs::read_to_string("input.txt").expect("Error reading file")
}

pub fn separate_input(input: &str) -> Vec<u32> {
    input.split("\n").map(|i| i.parse::<u32>().unwrap()).collect()
}

pub fn find_numbers(input: &Vec<u32>) -> (u32, u32) {
    for i in input {
        for j in input {
            if i + j == 2020 {
                return (*i, *j);
            }
        }
    }
    (0, 0)
}

pub fn part1(input: &String) -> u32 {
    let input = separate_input(input);
    for i in &input {
        for j in &input {
            if i + j == 2020 {
                return i * j;
            }
        }
    }
    0
}

pub fn part2(input: &String) -> u32 {
    let input = separate_input(input);
    for i in &input {
        for j in &input {
            for k in &input {
                if i + j + k == 2020 {
                    return i * j * k;
                }
            }
        }
    }
    0
}
