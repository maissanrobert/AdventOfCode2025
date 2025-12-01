use crate::utils;

pub fn run(input_loc: Option<String>, part: Option<u8>) {
    println!("Running Day 01!");
    let input = input_loc.unwrap_or(String::from("day01"));
    let part_name = part.map_or_else(|| String::from("test"), |p| p.to_string());
    println!("Part: {}", part_name);

    match utils::read_input(&input, &part_name) {
        Ok(content) => {
            println!("Input loaded successfully!");
            solve(&content, &part_name);
        }
        Err(_) => {
            println!("No input file found.");            
        }
    }
}

fn solve(input: &str, part: &str) {
    let lines = utils::parse_csv(input);
    println!("Solving with input: {:?}", lines);
    match part {
        "1" => part1(&lines[0], &lines[2]),
        "2" => part2(&lines[0], &lines[2]),
        "test" =>  {
            part1(&lines[0], &lines[2]); 
            part2(&lines[0], &lines[2]); 
        },
        _ => panic!("Unknown part: {}", part),
    }
}

fn part1(names: &Vec<String>, instructions: &Vec<String>) {
    println!("Solving Part 1");
    
}

fn part2(names: &Vec<String>, instructions: &Vec<String>) {
    println!("Solving Part 2");
   
}
