use crate::utils::get_column;
use take_until::TakeUntilExt;

const TEST_INPUT : &str = "30373\n\
                           25512\n\
                           65332\n\
                           33549\n\
                           35390\n";

fn build_grid(input:&String) -> Vec<Vec<u32>> {
    return input.lines()
        .map(|line|
             line.chars()
             .flat_map(|c| c.to_digit(10)).collect())
        .collect();
}

fn is_visible<'a>(grid: &'a Vec<Vec<u32>>, x: usize, y:usize) -> bool {
    let height = grid[y][x];
    let (row, col) =
        (&grid[y], get_column(grid, x));
    let (left, right, above, below) =
        (&row[..x], &row[(x+1)..], &col[..y], &col[(y+1)..]);

    if left.iter().all(|&x| x < height)
    || right.iter().all(|&x| x < height)
    || above.iter().all(|&x| *x < height)
    || below.iter().all(|&x| *x < height) {
        return true;
    }
    return false;
}

fn scenic_score<'a>(grid: &'a Vec<Vec<u32>>, x: usize, y:usize) -> u32 {
    let height = grid[y][x];
    let (row, col) =
        (&grid[y], get_column(grid, x));
    let (left, right, above, below) =
        (&row[..x], &row[(x+1)..], &col[..y], &col[(y+1)..]);

    let left_score = left.iter().rev()
        .take_until(|&x| *x >= height)
        .collect::<Vec<_>>().len();
    let right_score = right.iter()
        .take_until(|&x| *x >= height)
        .collect::<Vec<_>>().len();
    let above_score = above.iter().rev()
        .take_until(|&x| **x >= height)
        .collect::<Vec<_>>().len();
    let below_score = below.iter()
        .take_until(|&x| **x >= height)
        .collect::<Vec<_>>().len();

    (left_score * right_score * above_score * below_score) as u32
}

pub fn solve1(input:&String) -> String {
    // let grid = build_grid(&TEST_INPUT.to_string());
    let grid = build_grid(input);

    let mut total = 0;
    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            if is_visible(&grid, x, y) {
                total += 1;
            }
        }
    }
    return total.to_string();
}

pub fn solve2(input:&String) -> String {
    // let grid = build_grid(&TEST_INPUT.to_string());
    let grid = build_grid(input);

    let mut max_score = 0;
    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            let score = scenic_score(&grid, x, y);
            if score > max_score {
                max_score = score;
            }
        }
    }
    return max_score.to_string();

}
