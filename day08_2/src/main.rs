use std::collections::HashSet;

fn main() {
    let input = include_str!("../input.txt");
    let grid: Vec<Vec<u32>> = input
        .split_terminator("\n")
        .map(|row| row.chars().map(|el| el.to_digit(10).unwrap()).collect())
        .collect();

    let mut highest_score = 0;

    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            let height = grid[y][x];
            if x == 0 || y == 0 || x == grid.len() - 1 || y == grid.len() - 1 {
                continue;
            }

            // Check left
            let mut left_score = 0;
            for x_inner in (0..x).rev() {
                left_score += 1;
                let new_height = grid[y][x_inner];
                if new_height >= height {
                    break;
                }
            }

            let mut right_score = 0;
            for x_inner in (x + 1)..grid[y].len() {
                right_score += 1;
                let new_height = grid[y][x_inner];
                if new_height >= height {
                    break;
                }
            }

            let mut top_score = 0;
            for y_inner in (0..y).rev() {
                top_score += 1;
                let new_height = grid[y_inner][x];
                if new_height >= height {
                    break;
                }
            }

            let mut bottom_score = 0;
            for y_inner in (y + 1)..grid.len() {
                bottom_score += 1;
                let new_height = grid[y_inner][x];
                if new_height >= height {
                    break;
                }
            }

            let total = left_score * right_score * top_score * bottom_score;
            if total > highest_score {
                highest_score = total
            }
        }
    }

    println!("{highest_score:?}");
}
