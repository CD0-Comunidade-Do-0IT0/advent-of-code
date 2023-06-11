use std::fs;

pub fn read_input_file() -> String {
  fs::read_to_string("input.txt").expect("Error reading file")
}

pub fn part1(input: &str) -> i32 {
  let mut nice_strings = 0;

  for line in input.lines() {
    let mut vowel_count = 0;
    let mut has_double_letter = false;
    let mut has_bad_string = false;

    for (i, c) in line.chars().enumerate() {
      if c == 'a' || c == 'e' || c == 'i' || c == 'o' || c == 'u' {
        vowel_count += 1;
      }

      if i > 0 {
        let prev_char = line.chars().nth(i - 1).unwrap();
        if prev_char == c {
          has_double_letter = true;
        }

        if (prev_char == 'a' && c == 'b') || (prev_char == 'c' && c == 'd') || (prev_char == 'p' && c == 'q') || (prev_char == 'x' && c == 'y') {
          has_bad_string = true;
        }
      }
    }

    if vowel_count >= 3 && has_double_letter && !has_bad_string {
      nice_strings += 1;
    }
  }

  nice_strings
}

pub fn part2(input: &str) -> i32 {
  let mut nice_strings = 0;

  for line in input.lines() {
    let mut has_pair = false;
    let mut has_repeat = false;

    for (i, c) in line.chars().enumerate() {
      if i > 1 {
        let prev_char = line.chars().nth(i - 1).unwrap();
        let prev_prev_char = line.chars().nth(i - 2).unwrap();

        if prev_prev_char == c {
          has_repeat = true;
        }

        let pair = format!("{}{}", prev_char, c);
        if line.matches(&pair).count() > 1 {
          has_pair = true;
        }
      }
    }

    if has_pair && has_repeat {
      nice_strings += 1;
    }
  }

  nice_strings
}
