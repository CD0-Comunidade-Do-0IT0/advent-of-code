use std::fs;

pub fn read_input_file() -> String {
    fs::read_to_string("input.txt").expect("Error reading file")
}

pub fn separate_input(input: &str) -> Vec<i32> {
    input.split("\n").map(|i| i.parse::<i32>().unwrap()).collect()
}

pub fn calculate_fuel(mass: i32) -> i32 {
    (mass / 3) - 2
}

pub fn calculate_fuel_recursive(mass: i32) -> i32 {
    let fuel = calculate_fuel(mass);

    if fuel <= 0 {
        return 0;
    }

    fuel + calculate_fuel_recursive(fuel)
}

pub fn part1(input: &String) -> i32 {
    let input = separate_input(input);

    input.iter().map(|i| calculate_fuel(*i)).sum()
}

pub fn part2(input: &String) -> i32 {
    let input = separate_input(input);

    input.iter().map(|i| calculate_fuel_recursive(*i)).sum()
}
