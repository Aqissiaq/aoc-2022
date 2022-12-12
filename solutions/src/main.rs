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
// mod day11;
mod day12;

fn main() {
    let input = utils::get_input(12);
    // let input = utils::read_file(&"src/test.txt".to_string());
    println!("day 12 | part 1\n{}", day12::solve1(&input));
    println!("day 12 | part 2\n{}", day12::solve2(&input));
}
