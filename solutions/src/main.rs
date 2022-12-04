#![allow(dead_code)]
mod utils;
mod day1;
mod day2;
mod day3;
mod day4;

fn main() {
    // let day1_input = utils::get_input(1);
    // println!("day 1 | part 1\n{}", day1::solve1(&day1_input));
    // println!("day 1 | part 2\n{}", day1::solve2(&day1_input));

    // let day2_input = utils::get_input(2);
    // println!("day 2 | part 1\n{}", day2::solve1(&day2_input));
    // println!("day 2 | part 2\n{}", day2::solve2(&day2_input));

    // let day3_input = utils::get_input(3);
    // println!("day 3 | part 1\n{}", day3::solve1(&day3_input));
    // println!("day 3 | part 2\n{}", day3::solve2(&day3_input));

    let day4_input = utils::get_input(4);
    println!("day 4 | part 1\n{}", day4::solve1(&day4_input));
    println!("day 4 | part 2\n{}", day4::solve2(&day4_input));
}
