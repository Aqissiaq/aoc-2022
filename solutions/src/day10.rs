enum Command {
    Noop,
    Addx(i32)
}

fn parse_command(comm: &str) -> Option<Command> {
    match comm.split_whitespace().collect::<Vec<_>>()[..] {
        ["noop"] => Some(Command::Noop),
        ["addx", n] => Some(Command::Addx(n.parse().unwrap())),
        _ => None
    }
}

pub fn solve1(input:&String) -> String {
    let program = input.lines()
        .flat_map(|line| parse_command(line));

    let mut x = 1;
    let mut cycle = 0;
    let mut signal_strength = 0;

    let record_cycles = vec![20, 60, 100, 140, 180, 220];
    for comm in program {
        match comm {
            Command::Noop => {
                cycle += 1;
                if record_cycles.contains(&cycle) {
                    signal_strength += x * cycle;
                }
            }
            Command::Addx(n) => {
                if record_cycles.contains(&(cycle + 1)) {
                    signal_strength += x * (cycle + 1);
                }
                cycle += 2;
                if record_cycles.contains(&cycle) {
                    signal_strength += x * cycle;
                }
                x += n
            }
        }
    }

    signal_strength.to_string()
}

pub fn solve2(input:&String) -> String {
    let program = input.lines()
        .flat_map(|line| parse_command(line));

    let mut cycle = 0;
    let mut x: i32 = 1;
    let mut screen = vec![];

    for comm in program {
        match comm {
            Command::Noop => {
                cycle += 1;
                cycle %= 40;
                if x.abs_diff(cycle - 1) < 2 {
                    screen.push('#');
                } else {
                    screen.push(' ');
                }
            }
            Command::Addx(n) => {
                if x.abs_diff(cycle) < 2 {
                    screen.push('#');
                } else {
                    screen.push(' ');
                }

                cycle += 2;
                cycle %= 40;

                if x.abs_diff(cycle - 1) < 2 {
                    screen.push('#');
                } else {
                    screen.push(' ');
                }
                x += n
            }
        }
    }

    for line in screen.chunks(40) {
        println!("{:?}", line.iter().collect::<String>());
    }
    "Ssssshhhhh, don't worry about it".to_string()
}
