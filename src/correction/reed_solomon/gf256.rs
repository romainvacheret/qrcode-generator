use std::{fs::File, io::{BufRead, BufReader}};

// TODO: should no longer be public later one
pub fn get_log_antilog() -> Vec<Vec<u16>> {
    let file = File::open("log-antilog.csv").expect("Cannot open file");

    BufReader::new(file)
        .lines()
        .skip(1) // skip header
        .filter_map(Result::ok)
        .map(|line| line.split(',')
                        .map(|s| s.parse::<u16>().unwrap())
                        .collect::<Vec<u16>>())
        .collect::<Vec<Vec<u16>>>()
}
