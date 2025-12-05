/// Shared library for every days functions
pub mod day01;
pub mod day02;
pub mod day03;
pub mod day04;
pub mod day05;

/// Common file and string functions used by most days
pub mod utils {
    use std::fs;
    use std::path::Path;

    /// Read input file for a given day
    pub fn read_input(day: &str, part: &str) -> Result<String, std::io::Error> {
        let path = format!("inputs/{}{}.txt", day, part);
        println!("Reading input from: {}", path);
        fs::read_to_string(&path)
    }

    /// Read input file from a custom path
    pub fn read_file<P: AsRef<Path>>(path: P) -> Result<String, std::io::Error> {
        fs::read_to_string(path)
    }

    pub fn parse_csv(input: &str) -> Vec<Vec<String>> {
        let lines = input.lines().collect::<Vec<&str>>();

        lines.iter()
            .map(|line| line.split(',').map(|s| s.to_string()).collect())
            .collect()
    }
}
