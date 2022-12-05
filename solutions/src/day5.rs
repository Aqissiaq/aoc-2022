use std::collections::VecDeque;

fn parse_crates(input:&str) -> Vec<VecDeque<char>> {
    let mut crates : Vec<VecDeque<char>> = vec![VecDeque::new(); 9];
    for line in input.lines() {
        for i in 0..9 {
            let content = line.chars() .collect::<Vec<_>>()[i * 4 + 1];
            if content != ' ' && !content.is_numeric() {
                crates[i].push_back(content);
            }
        }
    }
    return crates;
}

fn parse_moves(input:&str) -> Vec<(u32, usize, usize)> {
    return input.lines().map(|line| {
        let words : Vec<_> = line.split_whitespace().collect();
        (words[1].parse().unwrap(),
         words[3].parse::<usize>().unwrap() - 1,
         words[5].parse::<usize>().unwrap() - 1)}).collect();
}

pub fn solve1(input: &str) -> String {
    let inputs = input.split("\n\n").collect::<Vec<&str>>();
    let (crate_input, moves_input) = (inputs[0], inputs[1]);
    let mut crates = parse_crates(crate_input);
    let moves = parse_moves(moves_input);

    for (n, from, to) in moves.iter() {
        for _ in 0..*n {
            let content = crates[*from].pop_front().unwrap();
            crates[*to].push_front(content);
        }
    }
    return crates.iter().map(|stack| stack[0]).collect();
}

pub fn solve2(input: &str) -> String {
    let inputs = input.split("\n\n").collect::<Vec<&str>>();
    let (crate_input, moves_input) = (inputs[0], inputs[1]);
    let mut crates = parse_crates(crate_input);
    let moves = parse_moves(moves_input);

    for (n, from, to) in moves.iter() {
        let mut movestack = VecDeque::new();
        for _ in 0..*n {
            movestack.push_front(crates[*from].pop_front().unwrap());
        }
        for _ in 0..*n {
            let next = movestack.pop_front().unwrap();
            crates[*to].push_front(next);
        }
    }
    return crates.iter().map(|stack| stack[0]).collect();
}
