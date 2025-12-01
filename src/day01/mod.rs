use crate::utils;

pub fn run(input_loc: Option<String>, part: Option<u8>) {
    println!("Running Day 01!");
    let input = input_loc.unwrap_or(String::from("day01"));
    let part_name: &str;
    if part.is_some() {
        part_name = "";
    } else {
        part_name = "_test";
    }

    println!("Part: {}", part_name);

    match utils::read_input(&input, &part_name) {
        Ok(content) => {
            println!("Input loaded successfully!");
            solve(&content, part);
        }
        Err(_) => {
            println!("No input file found.");            
        }
    }
}

fn solve(input: &str, part: Option<u8>) {
    let lines = input.lines()
        .filter(|line| !line.is_empty())
        .collect::<Vec<&str>>();

    println!("Solving with input: {:?}", lines);
    match part {
        Some(1) => part1(&lines),
        Some(2) => part2(&lines),
        None =>  {
            part1(&lines); 
            part2(&lines); 
        },
        _ => panic!("Unknown part."),
    }
}

fn part1(lines: &Vec<&str>) {
    println!("Solving Part 1");
    let mut position: i32 = 50; //start at 50
    let mut zero_count: i32 = 0; // number of times we stop at 0
    let numbers: i32 = 100; //0 to 99
    
    for line in lines {
        let (direction, amount) = rotate_instruction(line);
        let move_amount = amount % numbers;
        if direction == -1 {
            position = position - move_amount;
            if position < 0 {
                position += numbers;
            }
        }
        else {
            position = position + move_amount;
            if position >= numbers {
                position -= numbers;
            }
        }

        if position == 0 {
            zero_count += 1;
        }
    }
    println!("Zero count: {}", zero_count);
    println!("Result: {}", position);
}

fn part2(lines: &Vec<&str>) {
    println!("Solving Part 2");
    let mut position: i32 = 50; //start at 50
    let mut zero_count: i32 = 0; // number of times we stop at 0
    let numbers: i32 = 100; //0 to 99
    
    for line in lines {
        let (direction, amount) = rotate_instruction(line);
        let mut rotations: i32 = amount / numbers as i32;
        let move_amount = amount % numbers;
        //println!("Move Amount: {}", rotations);
        if direction == -1 {
            if position == 0 {
                rotations -= 1; //Account for 0 moving backwards to not double count it
            }
            position = position - move_amount;
            if position < 0 {
                rotations += 1;
                position += numbers;
            }
        }
        else {
            position = position + move_amount;
            if position >= numbers {
                rotations += 1;
                position -= numbers;
            }
            if position == 0 {
                rotations -= 1; //Account for 100 being counted during rotations
            }
        }

        zero_count += rotations;
        if position == 0 {
            zero_count += 1;
        }
        println!("Instruction: {} Position: {} Zero counts: {} times", line, position, zero_count);
    }
    println!("Zero count: {}", zero_count);
    println!("Result: {}", position);
}

//Retruns (direction, amount)
//Direction: -1 = left, 1 = right
fn rotate_instruction(instruction: &str) -> (i32, i32) {
    let direction_str = &instruction[..1];
    let direction = match direction_str {
        "L" => -1,
        "R" => 1,
        _ => panic!("Unknown direction: {}", direction_str),
    };
    let amount = instruction[1..].parse::<i32>().unwrap();
    
    (direction, amount)
}
