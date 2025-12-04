use crate::utils;

pub fn run(input_loc: Option<String>, part: Option<u8>) {
    println!("Running Day 04!");
    let input = input_loc.unwrap_or(String::from("day04"));
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

    let mut accessible_rolls : u32 = 0;
    
    let row_count = lines.len();
    let col_count = lines[0].len();

    for row in 0..row_count {
        for col in 0..col_count {
            if lines[row].chars().nth(col).unwrap() != '@' {
                continue;
            }

            let neighbours = get_neighbours(row, col, row_count-1, col_count-1);
            let mut neighbour_count : u32 = 0;
            
            //count how many neighbours are '@' sign
            for (row, col) in neighbours {
                if lines[row].chars().nth(col).unwrap() == '@' {
                    neighbour_count += 1;
                }
            }
            //check we have the right number of neighbours
            if neighbour_count < 4 {
                accessible_rolls += 1;
            }
        }
    }

    println!("Accessible Rolls: {}", accessible_rolls);
}

fn get_neighbours(row: usize, col: usize, max_row: usize, max_col: usize) -> Vec<(usize,usize)> {
    let mut neighbours : Vec<(usize,usize)> = Vec::new();

    //Get above neighbours
    if row > 0 {
        neighbours.push((row-1, col)); // Directly above
        if col > 0 {
            neighbours.push((row-1, col-1)); // Diagonal left
        }
        if col < max_col {
            neighbours.push((row-1, col+1)); // Diagonal right
        }
    }

    //Below neighbours
    if row < max_row {
        neighbours.push((row+1, col)); // Directly below
        if col > 0 {
            neighbours.push((row+1, col-1)); // Diagonal left
        }
        if col < max_col {
            neighbours.push((row+1, col+1)); // Diagonal right
        }
    }

    //Get left
    if col > 0 {
        neighbours.push((row, col-1)); // Left
    }
    //Get right
    if col < max_col {
        neighbours.push((row, col+1)); // Right
    }

    neighbours
}

fn part2(lines: &Vec<&str>) {
    println!("Solving Part 2!");
    
    //Convert to list of list of chars so we can replace chars later
    let mut grid : Vec<Vec<char>> = Vec::new();
    for line in lines {
        grid.push(line.chars().collect());
    }

    let mut accessible_rolls : u32 = usize::MAX as u32;
    let mut removed_rolls : u32 = 0;
    
    let row_count = lines.len();
    let col_count = lines[0].len();

    //Convert to 2d array
    let mut grid : Vec<Vec<char>> = Vec::new();
    for line in lines {
        grid.push(line.chars().collect());
    }

    while accessible_rolls > 0 {
        accessible_rolls = 0;
        let mut to_remove : Vec<(usize,usize)> = Vec::new();

        for row in 0..row_count {
            for col in 0..col_count {
                if grid[row][col] != '@' {
                    continue;
                }

                let neighbours = get_neighbours(row, col, row_count-1, col_count-1);
                let mut neighbour_count : u32 = 0;
            
                //count how many neighbours are '@' sign
                for (row, col) in neighbours {
                    if grid[row][col] == '@' {
                        neighbour_count += 1;
                    }
                }
                //check we have the right number of neighbours
                if neighbour_count < 4 {
                    accessible_rolls += 1;
                    to_remove.push((row, col));
                }
            }   
        }
        for (row, col) in to_remove {
            grid[row][col] = '.';
        }
        removed_rolls += accessible_rolls;
    }

    println!("Removed Rolls: {}", removed_rolls);
}