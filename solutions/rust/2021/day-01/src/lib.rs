use std::fs;

pub fn read_input_file() -> String {
    fs::read_to_string("input.txt").expect("Error reading file")
}

pub fn separate_input(input: &str) -> Vec<u32> {
    input.split("\n").map(|i| i.parse::<u32>().unwrap()).collect()
}

pub fn count_larger(input: &Vec<u32>) -> u32 {
    let mut count = 0;
    let mut previous = 0;
    for measurement in input {
        if measurement > &previous {
            count += 1;
        }
        previous = *measurement;
    }
    count - 1
}

pub fn sum_measurements_parts(input: &Vec<u32>) -> Vec<u32> {
    input.iter().enumerate().map(|(i, e)| {
        if i >= input.len() - 2 {
            0
        } else {
            e + input[i + 1] + input[i + 2]
        }
    }).filter(|i| *i != 0).collect()
}

pub fn part1(input: &String) -> u32 {
    let input = separate_input(&input);
    count_larger(&input)
}

pub fn part2(input: &String) -> u32 {
    let input = separate_input(&input);
    let summed = sum_measurements_parts(&input);
    count_larger(&summed)
}
