use std::fs;
use md5;

pub fn read_input_file() -> String {
  fs::read_to_string("input.txt").expect("Error reading file")
}

#[allow(unused_assignments)]
pub fn find_hash(input: &str, prefix: &str) -> i32 {
  let mut i = 0;
  let mut result = String::new();
  let mut found = false;

  while !found {
    let input = format!("{}{}", input, i);
    let digest = md5::compute(input);
    result = format!("{:x}", digest);

    if result.starts_with(prefix) {
      found = true;
    } else {
      i += 1;
    }
  }

  i
}

pub fn part1(input: &str) -> i32 {
  find_hash(input, "00000")
}

pub fn part2(input: &str) -> i32 {
  find_hash(input, "000000")
}
