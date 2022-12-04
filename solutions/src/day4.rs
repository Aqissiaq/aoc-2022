fn parse_ranges(input: &String) -> Vec<Vec<Vec<u32>>> {
    return input
        .lines()
        .map(|line| {
            line.split(',')
                .map(|range| {
                    range
                        .split('-')
                        .map(|num| num.parse::<u32>().unwrap())
                        .collect()
                })
                .collect()
        })
        .collect();
}

pub fn solve1(input: &String) -> String {
    let mut total = 0;
    for pair in parse_ranges(input).iter() {
        let range1start = pair[0][0];
        let range1end = pair[0][1];

        let range2start = pair[1][0];
        let range2end = pair[1][1];

        if range1start <= range2start && range1end >= range2end
            || range2start <= range1start && range2end >= range1end
        {
            total += 1;
        }
    }

    return total.to_string();
}

pub fn solve2(input: &String) -> String {
    let mut total = 0;
    for pair in parse_ranges(input).iter() {
        let range1start = pair[0][0];
        let range1end = pair[0][1];

        let range2start = pair[1][0];
        let range2end = pair[1][1];

        if range1end >= range2start && range2end >= range1start {
            total += 1;
        }
    }
    return total.to_string();
}

//this is ugly but straightforward
