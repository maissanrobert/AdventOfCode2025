use crate::utils;
use std::collections::HashSet;

pub fn run(input_loc: Option<String>, part: Option<u8>) {
    println!("Running Day 05!");
    let input = input_loc.unwrap_or(String::from("day05"));
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
    let (fresh_ranges, ingredients) = separate_lines(lines);
    println!("Ranges: {:?}", fresh_ranges);
    println!("Ingredients: {:?}", ingredients);

    let mut fresh_count = 0;
    for ingredient in ingredients.iter() {
        let mut is_fresh = false;
        let mut i: usize = 0;
        //ingredients are fresh if they are in any range
        while !is_fresh && i < fresh_ranges.len() {
            if *ingredient >= fresh_ranges[i].0 && *ingredient <= fresh_ranges[i].1 {
                //number is in the fresh range
                is_fresh = true;
            }
            i += 1;
        }
        if is_fresh {
            fresh_count += 1;
        }
    }
    println!("Fresh count: {}", fresh_count);
}

fn part2(lines: &Vec<&str>) {
    println!("Solving Part 2!");

    let (mut fresh_ranges, _) = separate_lines(lines);
    //println!("Ranges: {:?}", fresh_ranges);
    
    let mut valid_fresh_ranges: Vec<(u64,u64)> = Vec::new();
    
    //sort ranges by start
    fresh_ranges.sort_by(|a, b| a.0.cmp(&b.0));
    valid_fresh_ranges.push(fresh_ranges[0]);
    let mut last_high: u64 = valid_fresh_ranges[0].1;

    for range in fresh_ranges.iter().skip(1) {
        //check if the low is higher than the last high
        //if so, it is an entirely new range
        if range.0 > last_high {
            valid_fresh_ranges.push(*range);
            last_high = range.1;
            continue;
        }

        //check if the high is lower than the last high 
        //if so, it is a subset of the last range
        if range.1 <= last_high {
            continue;
        }

        //it must overlap the last range - extend the last range
        valid_fresh_ranges.last_mut().unwrap().1 = range.1;
        last_high = range.1;
    }
    
    //now that we have unified all the valid ranges, lets count the total
    let mut total_fresh: u64 = 0;
    for range in valid_fresh_ranges.iter() {
        total_fresh += range.1 - range.0 + 1;
    }

    println!("Total fresh: {}", total_fresh);    
}

fn separate_lines(lines: &Vec<&str>) -> (Vec<(u64,u64)>, Vec<u64>) {
    let mut ranges: Vec<(u64,u64)> = Vec::new();
    let mut numbers: Vec<u64> = Vec::new();

    for line in lines.iter() {
        if line.contains("-") {
            let (first, second) = line.split_once("-").unwrap();
            ranges.push((first.parse::<u64>().unwrap(), second.parse::<u64>().unwrap()));
        } else if !line.is_empty() {
            numbers.push(line.parse::<u64>().unwrap());
        }
    }

    (ranges, numbers)
}
