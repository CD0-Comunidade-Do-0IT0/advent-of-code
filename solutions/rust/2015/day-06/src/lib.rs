use std::fs;

pub fn read_input_file() -> String {
  fs::read_to_string("input.txt").expect("Error reading file")
}

pub fn part1(input: &str) -> i32 {
  let mut lights = [[false; 1000]; 1000];

  for line in input.lines() {
    let splited_line: Vec<&str> = line.split(" ").collect();

    let (command, initial_position, final_position) = match splited_line.len() {
      4 => (splited_line[0], splited_line[1], splited_line[3]),
      5 => (splited_line[1], splited_line[2], splited_line[4]),
      _ => panic!("Invalid input"),
    };

    let initial_position: Vec<&str> = initial_position.split(",").collect();
    let final_position: Vec<&str> = final_position.split(",").collect();

    let initial_x = initial_position[0].parse::<usize>().unwrap();
    let initial_y = initial_position[1].parse::<usize>().unwrap();
    let final_x = final_position[0].parse::<usize>().unwrap();
    let final_y = final_position[1].parse::<usize>().unwrap();

    for x in initial_x..=final_x {
      for y in initial_y..=final_y {
        match command {
          "on" => lights[x][y] = true,
          "off" => lights[x][y] = false,
          "toggle" => lights[x][y] = !lights[x][y],
          _ => panic!("Invalid command"),
        }
      }
    }
  }

  let mut count = 0;

  for x in 0..1000 {
    for y in 0..1000 {
      if lights[x][y] {
        count += 1;
      }
    }
  }

  count
}

pub fn part2(input: &str) -> i32 {
  let mut lights = [[0; 1000]; 1000];

  for line in input.lines() {
    let splited_line: Vec<&str> = line.split(" ").collect();

    let (command, initial_position, final_position) = match splited_line.len() {
      4 => (splited_line[0], splited_line[1], splited_line[3]),
      5 => (splited_line[1], splited_line[2], splited_line[4]),
      _ => panic!("Invalid input"),
    };

    let initial_position: Vec<&str> = initial_position.split(",").collect();
    let final_position: Vec<&str> = final_position.split(",").collect();

    let initial_x = initial_position[0].parse::<usize>().unwrap();
    let initial_y = initial_position[1].parse::<usize>().unwrap();
    let final_x = final_position[0].parse::<usize>().unwrap();
    let final_y = final_position[1].parse::<usize>().unwrap();

    for x in initial_x..=final_x {
      for y in initial_y..=final_y {
        match command {
          "on" => lights[x][y] += 1,
          "off" => lights[x][y] = if lights[x][y] > 0 { lights[x][y] - 1 } else { 0 },
          "toggle" => lights[x][y] += 2,
          _ => panic!("Invalid command"),
        }
      }
    }
  }

  let mut count = 0;

  for x in 0..1000 {
    for y in 0..1000 {
      count += lights[x][y];
    }
  }

  count
}
