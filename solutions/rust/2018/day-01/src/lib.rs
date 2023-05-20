use std::fs;
use std::collections::HashSet;

pub fn read_input_file() -> String {
    fs::read_to_string("input.txt").expect("Error reading file")
}

pub fn separate_input(input: &str) -> Vec<i32> {
    input.split("\n").map(|i| i.parse::<i32>().unwrap()).collect()
}

pub fn part1(input: &String) -> i32 {
    let input = separate_input(input);

    input.iter().sum()
}
// You notice that the device repeats the same frequency change list over and over. To calibrate the device, you need to find the first frequency it reaches twice.

// For example, using the same list of changes above, the device would loop as follows:

// Current frequency  0, change of +1; resulting frequency  1.
// Current frequency  1, change of -2; resulting frequency -1.
// Current frequency -1, change of +3; resulting frequency  2.
// Current frequency  2, change of +1; resulting frequency  3.
// (At this point, the device continues from the start of the list.)
// Current frequency  3, change of +1; resulting frequency  4.
// Current frequency  4, change of -2; resulting frequency  2, which has already been seen.
// In this example, the first frequency reached twice is 2. Note that your device might need to repeat its list of frequency changes many times before a duplicate frequency is found, and that duplicates might be found while in the middle of processing the list.

// Here are other examples:

// +1, -1 first reaches 0 twice.
// +3, +3, +4, -2, -4 first reaches 10 twice.
// -6, +3, +8, +5, -6 first reaches 5 twice.
// +7, +7, -2, -7, -4 first reaches 14 twice.

pub fn part2(input: &String) -> i32 {
    let input = separate_input(input);

    let mut current_frequency = 0;
    let mut frequencies = HashSet::new();
    frequencies.insert(0);

    loop {
        for i in &input {
            current_frequency += i;

            if frequencies.contains(&current_frequency) {
                return current_frequency;
            }

            frequencies.insert(current_frequency);
        }
    }
}
