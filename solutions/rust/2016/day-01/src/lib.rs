use std::{fs, collections::HashSet};

pub fn read_input_file() -> String {
    fs::read_to_string("input.txt").expect("Error reading file")
}

pub fn part1(input: &String) -> i32 {
    let mut x = 0;
    let mut y = 0;
    let mut direction = 0;

    for instruction in input.split(", ") {
        let turn = instruction.chars().next().unwrap();
        let blocks: i32 = instruction[1..].parse().unwrap();

        if turn == 'R' {
            direction = (direction + 1) % 4;
        } else if turn == 'L' {
            direction = (direction + 3) % 4;
        }

        match direction {
            0 => y += blocks,
            1 => x += blocks,
            2 => y -= blocks,
            3 => x -= blocks,
            _ => panic!("Invalid direction!"),
        }
    }

    x.abs() + y.abs()
}

// Then, you notice the instructions continue on the back of the Recruiting Document. Easter Bunny HQ is actually at the first location you visit twice.

// For example, if your instructions are R8, R4, R4, R8, the first location you visit twice is 4 blocks away, due East.

// How many blocks away is the first location you visit twice?

pub fn part2(input: &String) -> i32 {
    let mut x = 0;
    let mut y = 0;
    let mut direction = 0;
    let mut visited = HashSet::new();

    for instruction in input.split(", ") {
        let turn = instruction.chars().next().unwrap();
        let blocks: i32 = instruction[1..].parse().unwrap();

        if turn == 'R' {
            direction = (direction + 1) % 4;
        } else if turn == 'L' {
            direction = (direction + 3) % 4;
        }

        for _ in 0..blocks {
            match direction {
                0 => y += 1,
                1 => x += 1,
                2 => y -= 1,
                3 => x -= 1,
                _ => panic!("Invalid direction!"),
            }

            if !visited.insert((x, y)) {
                return x + y;
            }
        }
    }

    panic!("No revisited location found!")
}