#![allow(dead_code)]
mod utils;
mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;

fn main() {
    let input = utils::get_input(7);
    // let input = utils::read_file(&"src/test.txt".to_string());
    println!("day 7 | part 1\n{}", day7::solve1(&input));
    println!("day 7 | part 2\n{}", day7::solve2(&input));
}
