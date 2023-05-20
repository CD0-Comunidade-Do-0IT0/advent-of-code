use day_01::{read_input_file, process_input, calculate_sum};

fn main() {
    let input = read_input_file();

    let result = process_input(&input);

    let summed = calculate_sum(&result);

    println!("Part 1 result: {}", result[0]);
    println!("Part 2 result: {}", summed);
}
