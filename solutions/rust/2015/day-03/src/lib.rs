use std::fs;

pub fn read_input_file() -> String {
  fs::read_to_string("input.txt").expect("Error reading file")
}

pub fn part1(input: &str) -> i32 {
  let commands: Vec<char> = input.chars().collect();

  let mut position: (i32, i32) = (0, 0);
  let mut visited: Vec<(i32, i32)> = vec![(0, 0)];

  for command in commands {
    let (x, y) = position;
    let new_position = match command {
      '^' => (x, y + 1),
      'v' => (x, y - 1),
      '>' => (x + 1, y),
      '<' => (x - 1, y),
      _ => (x, y),
    };

    position = new_position;

    if !visited.iter().any(|&p| p == new_position) {
      visited.push(new_position);
    }
  }

  visited.len() as i32
}

pub fn part2(input: &str) -> i32 {
  let commands: Vec<char> = input.chars().collect();

  let mut santa_position: (i32, i32) = (0, 0);
  let mut robot_position: (i32, i32) = (0, 0);
  let mut visited: Vec<(i32, i32)> = vec![(0, 0)];

  for (i, command) in commands.iter().enumerate() {
    let (x, y) = if i % 2 == 0 {
      santa_position
    } else {
      robot_position
    };

    let new_position = match command {
      '^' => (x, y + 1),
      'v' => (x, y - 1),
      '>' => (x + 1, y),
      '<' => (x - 1, y),
      _ => (x, y),
    };

    if i % 2 == 0 {
      santa_position = new_position;
    } else {
      robot_position = new_position;
    }

    if !visited.iter().any(|&p| p == new_position) {
      visited.push(new_position);
    }
  }

  visited.len() as i32
}
