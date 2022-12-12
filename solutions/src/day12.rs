use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;

pub fn solve1(input:&String) -> String {
    // hardcoded because I can't set variables from inside a closure??
    let start: (i32, i32) = (0, 20);
    let end: (i32, i32) = (146, 20);

    // build the map
    let map: HashMap<(i32, i32), i32> = input.lines()
        .enumerate()
        .flat_map(|(y, line)|
             line.chars()
             .enumerate()
             .map(move |(x, c)| match c {
                 'S' => {println!("start: ({x}, {y})");//start = (x, y);
                         ((x as i32, y as i32), 'a' as i32)},
                 'E' => {println!("end: ({x}, {y})");//end = (x, y);
                         ((x as i32, y as i32), 'z' as i32)},
                 c => ((x as i32, y as i32), c as i32),
             })
        ).collect();

    // BFS
    let mut queue = VecDeque::from([start]);
    let mut explored = HashSet::from([start]);
    let mut parents: HashMap<(i32, i32), (i32, i32)> = HashMap::new();

    let mut c_x;
    let mut c_y;
    let mut current_height;

    while !queue.is_empty() {
         (c_x, c_y) = queue.pop_front().unwrap();
        current_height = map.get(&(c_x, c_y)).unwrap();
        if (c_x, c_y) == end {
            break;
        }
        let dirs = [(c_x - 1, c_y),
                    (c_x + 1, c_y),
                    (c_x, c_y - 1),
                    (c_x, c_y + 1),];

        for d in dirs {
            if !explored.contains(&d) {
                match map.get(&d) {
                    Some(height) => if height - current_height <= 1 {
                        explored.insert(d);
                        queue.push_back(d);
                        parents.insert(d, (c_x, c_y));
                    },
                     None => ()
                    }
                }
            }
    }
    let mut counter = 0;
    let mut current = end;
    loop {
        counter += 1;
        current = *parents.get(&current).unwrap();
        if current == start {
            break;
        }
    }
    counter.to_string()
}

pub fn solve2(input:&String) -> String {
    let end: (i32, i32) = (146, 20);

    // build the map
    let map: HashMap<(i32, i32), i32> = input.lines()
        .enumerate()
        .flat_map(|(y, line)|
             line.chars()
             .enumerate()
             .map(move |(x, c)| match c {
                 'S' => {println!("start: ({x}, {y})");//start = (x, y);
                         ((x as i32, y as i32), 'a' as i32)},
                 'E' => {println!("end: ({x}, {y})");//end = (x, y);
                         ((x as i32, y as i32), 'z' as i32)},
                 c => ((x as i32, y as i32), c as i32),
             })
        ).collect();

    // BFS
    let mut queue = VecDeque::from([end]);
    let mut explored = HashSet::from([end]);
    let mut parents: HashMap<(i32, i32), (i32, i32)> = HashMap::new();

    let mut c_x = 0;
    let mut c_y = 0;
    let mut current_height;

    while !queue.is_empty() {
         (c_x, c_y) = queue.pop_front().unwrap();
        current_height = map.get(&(c_x, c_y)).unwrap();
        if *current_height == 'a' as i32 {
            break;
        }
        let dirs = [(c_x - 1, c_y),
                    (c_x + 1, c_y),
                    (c_x, c_y - 1),
                    (c_x, c_y + 1),];

        for d in dirs {
            if !explored.contains(&d) {
                match map.get(&d) {
                    Some(height) => if current_height - height <= 1 {
                        explored.insert(d);
                        queue.push_back(d);
                        parents.insert(d, (c_x, c_y));
                    },
                     None => ()
                    }
                }
            }
    }
    let mut counter = 0;
    let mut current = (c_x, c_y);
    loop {
        counter += 1;
        current = *parents.get(&current).unwrap();
        if current == end {
            break;
        }
    }
    counter.to_string()
}
