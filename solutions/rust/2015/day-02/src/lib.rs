use std::fs;

pub fn read_input_file() -> String {
    fs::read_to_string("input.txt").expect("Error reading file")
}

pub fn calculate_wrapping_paper(l: i32, w: i32, h: i32) -> i32 {
    (2 * l * w) + (2 * w * h) + (2 * h * l) + (l * w).min(w * h).min(h * l)
}

pub fn calculate_ribbon(l: i32, w: i32, h: i32) -> i32 {
    let mut dimensions = vec![l, w, h];
    dimensions.sort();

    let smallest_perimeter = (2 * dimensions[0]) + (2 * dimensions[1]);
    let volume = dimensions[0] * dimensions[1] * dimensions[2];

    smallest_perimeter + volume
}

pub fn part1(input: &str) -> i32 {
    input.lines().map(|line| {
        let mut dimensions = line.split('x').map(|x| x.parse::<i32>().unwrap());
        let l = dimensions.next().unwrap();
        let w = dimensions.next().unwrap();
        let h = dimensions.next().unwrap();

        calculate_wrapping_paper(l, w, h)
    }).sum()
}

pub fn part2(input: &str) -> i32 {
    input.lines().map(|line| {
        let mut dimensions = line.split('x').map(|x| x.parse::<i32>().unwrap());
        let l = dimensions.next().unwrap();
        let w = dimensions.next().unwrap();
        let h = dimensions.next().unwrap();

        calculate_ribbon(l, w, h)
    }).sum()
}