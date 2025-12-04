use crate::utils;

pub fn run(input_loc: Option<String>, part: Option<u8>) {
    println!("Running Day 03!");
    let input = input_loc.unwrap_or(String::from("day03"));
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
    println!("Solving Part 1!");
    let mut joltage: u32 = 0;

    for line in lines {
        let mut index_highest: usize = usize::MAX;
        let mut index_second_highest: usize = usize::MAX;
        let mut highest: u32 = 0;
        let mut second_highest: u32 = 0;

        //find the index of the highest digit
        for (index, character) in line.chars().enumerate() {
            let digit = character.to_digit(10).unwrap();
            if digit > highest {
                index_highest = index;
                highest = digit;
            }
            if digit == 9 {
                break;
            }
        }

        //find the index of the second highest digit
        //start at the index of the highest digit + 1 if it was not the last digit
        let start_index = if index_highest == line.len() - 1 { 0 } else { index_highest + 1 };
        let end_index = if index_highest == line.len() - 1 { line.len() - 1 } else { line.len() };

        for index in start_index..end_index {
            let digit = line.chars().nth(index).unwrap().to_digit(10).unwrap();
            if digit > second_highest && index != index_highest {
                index_second_highest = index;
                second_highest = digit;
            }
            if digit == 9 {
                break;
            }
        }

        let line_joltage: u32;
        if index_highest < index_second_highest {
            line_joltage = (highest * 10) + second_highest;
        } else {
            line_joltage = (second_highest * 10) + highest;
        }
        println!("Line Joltage: {}", line_joltage);
        joltage += line_joltage;
    }
    println!("Joltage: {}", joltage);
}

fn part2(lines: &Vec<&str>) {
    println!("Solving Part 2!");
    let mut joltage: u64 = 0;

    for line in lines {
        println!("Line: {}", line);
        let mut selected: Vec<u64> = Vec::new();

        let length = line.len();
        for index in 0..length {
            let digit: u64 = line.chars().nth(index).unwrap().to_digit(10).unwrap().into();

            //Always append the first digit
            if selected.len() == 0 {
                selected.push(digit);
                continue;
            }

            //Is this higher than any of the selected digits?
            let is_higher = selected.iter().any(|&x| digit > x);
            //If it is not higher than all the others and than are less than 12, append it
            if !is_higher && selected.len() < 12 {
                selected.push(digit);
                continue;
            } else if !is_higher {
                //if it is lower than all the others and there are 12, skip it
                continue;
            }

            let mut index_lower = selected.iter().position(|&x| x < digit).unwrap();
            let characters_to_trim = 12 - index_lower;

            if (index+characters_to_trim) > length {
                //we can only trim the list if there are enough characters left                
                //For example, there are 3 items left.  This would be insterted at 6
                //That would cause 5 items to be trimmed - we can't do that
                //But we can insert it 8 and trim 3
                index_lower = 12 - (length - index);                
            }
            selected.insert(index_lower, digit);
            selected = selected[..=index_lower].to_vec();
        }
        println!("Selected: {:?}", selected);
        let mut local_joltage: u64 = 0;
        let mut exponent: u32 = 12;
        for i in 0..selected.len() {
            exponent -= 1;
            local_joltage += selected[i] * (10 as u64).pow(exponent);
        }
        println!("Local Joltage: {}", local_joltage);
        joltage += local_joltage;
    }
    println!("Joltage: {}", joltage);
}