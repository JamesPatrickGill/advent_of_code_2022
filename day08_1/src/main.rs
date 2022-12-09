use std::collections::HashSet;

fn main() {
    let input = include_str!("../input.txt");
    let grid: Vec<Vec<u32>> = input
        .split_terminator("\n")
        .map(|row| row.chars().map(|el| el.to_digit(10).unwrap()).collect())
        .collect();

    let mut visible_trees = 0;

    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            let height = grid[y][x];
            if x == 0 || y == 0 || x == grid.len() - 1 || y == grid.len() - 1 {
                visible_trees += 1;
                continue;
            }

            // Check left
            let mut visible = 1;
            for x_inner in 0..x {
                let new_height = grid[y][x_inner];
                if new_height >= height {
                    visible = 0;
                    break;
                }
            }
            if visible == 1 {
                visible_trees += 1;
                continue;
            }

            // Check right
            visible = 1;
            for x_inner in (x + 1)..grid[y].len() {
                let new_height = grid[y][x_inner];
                if new_height >= height {
                    visible = 0;
                    break;
                }
            }
            if visible == 1 {
                visible_trees += 1;
                continue;
            }

            // Check right
            visible = 1;
            for y_inner in 0..y {
                let new_height = grid[y_inner][x];
                if new_height >= height {
                    visible = 0;
                    break;
                }
            }
            if visible == 1 {
                visible_trees += 1;
                continue;
            }

            // Check right
            visible = 1;
            for y_inner in (y + 1)..grid.len() {
                let new_height = grid[y_inner][x];
                if new_height >= height {
                    visible = 0;
                    break;
                }
            }
            if visible == 1 {
                visible_trees += 1;
                continue;
            }
        }
    }

    println!("{visible_trees:?}");
}
