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

fn main() {
    let input = utils::get_input(8);
    // let input = utils::read_file(&"src/test.txt".to_string());
    println!("day 8 | part 1\n{}", day8::solve1(&input));
    println!("day 8 | part 2\n{}", day8::solve2(&input));
}
