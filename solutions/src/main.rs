#![allow(dead_code)]
mod utils;
mod day1;
mod day2;
mod day3;
mod day4;
mod day5;

fn main() {
    let input = utils::get_input(5);
    // let input = utils::read_file(&"src/test.txt".to_string());
    println!("day 5 | part 1\n{}", day5::solve1(&input));
    println!("day 5 | part 2\n{}", day5::solve2(&input));
}
