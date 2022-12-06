#![allow(dead_code)]
mod utils;
mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;

fn main() {
    let input = utils::get_input(6);
    // let input = utils::read_file(&"src/test.txt".to_string());
    println!("day 6 | part 1\n{}", day6::solve1(&input));
    println!("day 6 | part 2\n{}", day6::solve2(&input));
}
