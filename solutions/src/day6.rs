use std::collections::HashSet;

fn general_case(input:&String, n:usize) -> u32 {
    let mut counter = n as u32;

    for window in input.chars().collect::<Vec<_>>().windows(n) {
        let unique : HashSet<_> = window.iter().collect();
        if unique.len() == n {
            break;
        }
        counter += 1;
    }

    return counter;
}

pub fn solve1(input:&String) -> String {
    return general_case(input, 4).to_string();
}

pub fn solve2(input:&String) -> String {
    return general_case(input, 14).to_string();
}
