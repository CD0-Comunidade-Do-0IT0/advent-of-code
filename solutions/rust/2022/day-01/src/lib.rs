use std::fs;

pub fn read_input_file() -> String {
    fs::read_to_string("input.txt").expect("Error reading file")
}

pub fn process_input(input: &str) -> Vec<u32> {
    let mut result = input
        .split("\n\n")
        .map(|items|
            items
                .split("\n")
                .filter(|item| !item.is_empty())
                .map(|item| item.parse::<u32>().unwrap())
                .sum::<u32>()
        )
        .collect::<Vec<u32>>();

    result.sort_by(|a, b| b.cmp(a));
    result
}

pub fn calculate_sum(result: &[u32]) -> u32 {
    result.iter().take(3).sum()
}
