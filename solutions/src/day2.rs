fn score(opponent: u8, player: u8) -> u32 {
    match opponent as char {
        'A' => match player as char {
            'X' => 1 + 3,
            'Y' => 2 + 6,
            'Z' => 3 + 0,
            _ => 0
        }
        'B' => match player as char {
            'X' => 1 + 0,
            'Y' => 2 + 3,
            'Z' => 3 + 6,
            _ => 0
        }
        'C' => match player as char {
            'X' => 1 + 6,
            'Y' => 2 + 0,
            'Z' => 3 + 3,
            _ => 0
        }
        _ => 0
    }
}

fn parse_input (input: &String) -> Vec<(u8, u8)> {
    return input.lines()
        .map(|line| {
            let bytes = line.as_bytes();
            (bytes[0], bytes[2])}).collect();
}

pub fn solve1(input: &String) -> String {
    return parse_input(input)
        .iter()
        .map(|(a,b)| score(*a, *b)).sum::<u32>().to_string();
}

fn strat(opponent: u8, player: u8) -> u32 {
    match opponent as char {
        'A' => match player as char {
            'X' => 3 + 0,
            'Y' => 1 + 3,
            'Z' => 2 + 6,
            _ => 0
        }
        'B' => match player as char {
            'X' => 1 + 0,
            'Y' => 2 + 3,
            'Z' => 3 + 6,
            _ => 0
        }
        'C' => match player as char {
            'X' => 2 + 0,
            'Y' => 3 + 3,
            'Z' => 1 + 6,
            _ => 0
        }
        _ => 0
    }
}
pub fn solve2(input: &String) -> String {
    return parse_input(input)
        .iter()
        .map(|(a,b)| strat(*a, *b)).sum::<u32>().to_string();
}
