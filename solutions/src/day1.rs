
fn sum_calories(input: &str) -> Vec<u32> {
    return input.split("\n\n")
        .map(|elf| elf.lines()
             .map(|cal| cal.parse::<u32>().unwrap()).sum::<u32>())
        .collect();
}

pub fn solve1(input:&String) -> String {
    return sum_calories(input).iter().max().unwrap().to_string();
}

pub fn solve2(input:&String) -> String {
    let mut elves = sum_calories(input);
    elves.sort();
    return elves.iter().rev().take(3).sum::<u32>().to_string();

}
