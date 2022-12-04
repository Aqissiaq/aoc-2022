use std::collections::HashSet;

fn priority(c:u8) -> u8 {
    if c > 90 {
        return c - 96;
    } else {
        return c - 38;
    }
}

pub fn solve1(input:&String) -> String {
    let mut total = 0;
    for rucksack in input.lines() {
        let (comp1, comp2) = rucksack.split_at(rucksack.len() / 2);

        let overlap:HashSet<u8> = comp1.bytes()
            .filter(|c| comp2.contains(*c as char)).collect();

        let prios = overlap.iter().map(|c| priority(*c) as u32);
        total += prios.sum::<u32>();

    }
    return total.to_string();
}

pub fn solve2(input:&String) -> String {
    let mut total = 0;
    for group in input.lines()
        .collect::<Vec<&str>>().chunks(3).into_iter() {

            let (elf1, elf2, elf3) = (group[0], group[1], group[2]);

            let overlap:HashSet<u8> = elf1.bytes()
                .filter(|c| elf2.contains(*c as char))
                .filter(|c| elf3.contains(*c as char))
                .collect();

            let prios = overlap.iter().map(|c| priority(*c) as u32);
            total += prios.sum::<u32>();
    }

    return total.to_string();
}

// I tried so hard to do an elegant fold/intersection solution :'(
