use crate::utils::{read_input, parse_csv};

pub fn run(input_loc: Option<String>, part: Option<u8>) {
    println!("Running Day 02!");
    let input = input_loc.unwrap_or(String::from("day02"));
    let part_name: &str;
    if part.is_some() {
        part_name = "";
    } else {
        part_name = "_test";
    }

    println!("Part: {}", part_name);

    match read_input(&input, &part_name) {
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
    let lines = parse_csv(input);

    println!("Solving with input: {:?}", lines);
    match part {
        Some(1) => part1(&lines[0]),
        Some(2) => part2(&lines[0]),
        None =>  {
            part1(&lines[0]); 
            part2(&lines[0]); 
        },
        _ => panic!("Unknown part."),
    }
}

fn part1(id_ranges: &Vec<String>) {
    println!("Solving Part 1");
    let mut sum_of_invalid_ids: i64 = 0;
    for id_range in id_ranges {
        println!("Line: {}", id_range);
        let converted = id_range.split("-")
            .map(|x| x.parse::<i64>().unwrap())
            .collect::<Vec<i64>>();

        let invalid_ids = get_invalid_ids_part1(converted[0], converted[1]);
        println!("Invalid IDs: {:?}", invalid_ids);
        sum_of_invalid_ids += invalid_ids.iter().sum::<i64>();
    }
    println!("Sum of invalid IDs: {}", sum_of_invalid_ids);
}

fn part2(id_ranges: &Vec<String>) {
    println!("Solving Part 2");
    let mut sum_of_invalid_ids: i64 = 0;
    for id_range in id_ranges {
        println!("Line: {}", id_range);
        let converted = id_range.split("-")
            .map(|x| x.parse::<i64>().unwrap())
            .collect::<Vec<i64>>();

        let invalid_ids = get_invalid_ids_part2(converted[0], converted[1]);
        println!("Invalid IDs: {:?}", invalid_ids);
        sum_of_invalid_ids += invalid_ids.iter().sum::<i64>();
    }
    println!("Sum of invalid IDs: {}", sum_of_invalid_ids);
}

fn get_invalid_ids_part1(low: i64, high: i64) -> Vec<i64> {
    let mut invalid_ids = Vec::new();
    for id in low..=high {
        let id_str = id.to_string();
        let len = id_str.len();

        //if length is odd, we can't get a pair of repeating digits
        let is_odd_length = len % 2;
        if is_odd_length == 1 { 
            continue;
        }

        let half_len = len/2 as usize;
        let mut is_invalid = true;
        let mut i = 0 as usize;
        while i < half_len && is_invalid {
            if id_str.chars().nth(i) != id_str.chars().nth(i+half_len) {
                is_invalid = false;
            }
            i += 1;
        }

        if is_invalid {
            invalid_ids.push(id);
        }
    }
    invalid_ids
}

fn get_invalid_ids_part2(low: i64, high: i64) -> Vec<i64> {
    let mut invalid_ids = Vec::new();
    
    for id in low..=high {
        let id_str = id.to_string();
        let len = id_str.len();
        match len {
        2 => if id % 11 == 0 {
            invalid_ids.push(id);
        },
        3 => if id % 111 == 0 {
            invalid_ids.push(id);
        },
        4 => if id % 101 == 0 {
            //Only need to use 1001 to check if all the same digit because
            //11 * 101 = 1111
            //this can be used for all the even digit checks
            invalid_ids.push(id);
        },
        5 => if id % 11111 == 0 {
            invalid_ids.push(id);
        },
        6 => if id % 1001 == 0 || id % 10101 == 0 {
            //Re-using the 101 check from 4 - but extending it
            //10101 pattern checks for ababab pattern but not aaaaaa
            invalid_ids.push(id);
        },
        7 => if id % 1111111 == 0 {
            //No patterns in 7 digit numbers except all the same digit
            invalid_ids.push(id);
        },
        8 => if id % 10001 == 0 || id %  1010101 == 0 {
            //Re-using the 101 check from 4 - but extending it
            //1010101 pattern checks for abababab pattern but not aaaaaaaa
            invalid_ids.push(id);
        },
        9 => if id % 1001001 == 0 {
            //9 digit numbers can have all the same or
            //the same number duplicated 3 times.
            //1001001 checks for both since all the same digit is 3 sets of aaa
            invalid_ids.push(id);
        },
        10 => if id % 100001 == 0 || id % 101010101 == 0 {
            //Re-using the 101 check from 4 - but extending it
            //101010101 pattern checks for ababababab pattern but not aaaaaaaaaa
            invalid_ids.push(id);
        },
        _ => println!("Invalid length"),
        }
    }
    invalid_ids 
}
