// Utility functions for advent of code 2022
use std::fs;

pub fn get_input(day: u8) -> String {
    let input_path = "../inputs/day".to_owned() + &day.to_string();

    let contents = fs::read_to_string(input_path)
        .unwrap();

    return contents;
}

pub fn read_file(path: &String) -> String {
    let contents = fs::read_to_string(path)
        .unwrap();

    return contents;
}

pub fn get_column<'a, T>(arr: &'a Vec<Vec<T>>, idx: usize) -> Vec<&'a T> {
    arr.iter().map(|row| &row[idx]).collect()
}

