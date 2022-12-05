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

pub fn solve1(input: &str) -> String {
    let mut crates = parse_crates(input.split("\n\n").collect::<Vec<&str>>()[0]);
    let moves = input.split("\n\n").collect::<Vec<&str>>()[1];

    for line in moves.lines() {
        let words : Vec<_> = line.split_whitespace().collect();
        let (n, from, to) = (words[1].parse::<u32>().unwrap(),
                             words[3].parse::<usize>().unwrap() - 1,
                             words[5].parse::<usize>().unwrap() - 1);

        for _ in 0..n {
            let content = crates[from].pop_front().unwrap();
            crates[to].push_front(content);
        }
    }

    return crates.iter().map(|stack| stack[0]).collect();
}

pub fn solve2(input: &str) -> String {
    let mut crates = parse_crates(input.split("\n\n").collect::<Vec<&str>>()[0]);
    let moves = input.split("\n\n").collect::<Vec<&str>>()[1];

    for line in moves.lines() {
        let words : Vec<_> = line.split_whitespace().collect();
        let (n, from, to) = (words[1].parse::<u32>().unwrap(),
                             words[3].parse::<usize>().unwrap() - 1,
                             words[5].parse::<usize>().unwrap() - 1);

        let mut movestack = VecDeque::new();
        for _ in 0..n {
            movestack.push_front(crates[from].pop_front().unwrap());
        }
        for _ in 0..n {
            let next = movestack.pop_front().unwrap();
            crates[to].push_front(next);
        }
    }

    return crates.iter().map(|stack| stack[0]).collect();
}

// another truly awful solution tbh
