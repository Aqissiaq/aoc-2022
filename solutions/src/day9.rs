use std::collections::HashSet;

enum Move {
    U, D, L, R
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Coord {
    x : i32, y: i32
}

impl Coord {
    fn new(x: i32, y:i32) -> Coord {
        Coord{x, y}
    }

    fn make_move(&self, vec:Coord) -> Coord {
        Coord::new(self.x + vec.x, self.y + vec.y)
    }

    fn follow(&self, other: &Coord) -> Coord {
        if self.x.abs_diff(other.x) > 1 || self.y.abs_diff(other.y) > 1 {
            let x = self.x + (other.x - self.x).signum();
            let y = self.y + (other.y - self.y).signum();
            return Coord::new(x, y);
        }
        return self.clone();
        }

}

fn parse_move(line:&str) -> Option<(Move, i32)> {
    let tokens : Vec<_> = line.split_whitespace().collect();
    let dir = tokens[0];
    let dist : i32 = tokens[1].parse().unwrap();

    match dir {
        "U" => Some((Move::U, dist)),
        "D" => Some((Move::D, dist)),
        "L" => Some((Move::L, dist)),
        "R" => Some((Move::R, dist)),
         _ => None
    }
}

pub fn solve1(input:&String) -> String {
    let moves = input.lines()
        .flat_map(|line| parse_move(line));

    let mut visited: HashSet<Coord> = HashSet::new();
    let mut head_pos = Coord::new(0,0);
    let mut tail_pos = Coord::new(0,0);

    let mut dir: Coord;
    for (m, dist) in moves {
        match m {
            Move::U => dir = Coord::new(0, 1),
            Move::D => dir = Coord::new(0, -1),
            Move::L => dir = Coord::new(-1, 0),
            Move::R => dir = Coord::new(1, 0)
        }

        for _ in 0..dist {
            head_pos = head_pos.make_move(dir);
            tail_pos = tail_pos.follow(&head_pos);
            visited.insert(tail_pos);
        }
    }

    return visited.len().to_string();
}

pub fn solve2(input:&String) -> String {
    let moves = input.lines()
        .flat_map(|line| parse_move(line));

    let mut visited: HashSet<Coord> = HashSet::new();
    let mut head_pos = Coord::new(0,0);
    let mut tail_pos : Vec<Coord> = vec![Coord::new(0,0); 9];

    let mut dir: Coord;
    for (m, dist) in moves {
        match m {
            Move::U => dir = Coord::new(0, 1),
            Move::D => dir = Coord::new(0, -1),
            Move::L => dir = Coord::new(-1, 0),
            Move::R => dir = Coord::new(1, 0)
        }

        for _ in 0..dist {
            head_pos = head_pos.make_move(dir);
            tail_pos[0] = tail_pos[0].follow(&head_pos);
            for i in 1..tail_pos.len() {
                tail_pos[i] = tail_pos[i].follow(&tail_pos[i-1]);
            }
            visited.insert(tail_pos[8]);
        }
    }

    return visited.len().to_string();
}
