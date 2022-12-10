#![allow(dead_code)]
mod utils;
mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
// mod day7;
mod day8;
mod day9;
mod day10;

fn main() {
    let input = utils::get_input(10);
    // let input = utils::read_file(&"src/test.txt".to_string());
    println!("day 10 | part 1\n{}", day10::solve1(&input));
    println!("day 10 | part 2\n{}", day10::solve2(&input));
}
