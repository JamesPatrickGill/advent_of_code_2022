use std::collections::HashSet;

fn main() {
    let input = include_str!("../input.txt");
    let path_defns: Vec<Vec<(i32, i32)>> = input
        .split_terminator("\n")
        .map(|path| {
            path.split("->")
                .map(|val| val.trim())
                .map(|el| el.split(",").collect())
                .map(|el: Vec<&str>| (el[0].parse::<i32>().unwrap(), el[1].parse::<i32>().unwrap()))
                .collect()
        })
        .collect();

    let mut rocks: HashSet<(i32, i32)> = HashSet::new();
    let mut lowest_rock_y = 0;
    for p in path_defns {
        for idx in 0..p.len() - 1 {
            let start = p[idx];
            let end = p[idx + 1];

            if start.0 == end.0 {
                if start.1 < end.1 {
                    if end.1 > lowest_rock_y {
                        lowest_rock_y = end.1
                    }
                    for jdx in start.1..=end.1 {
                        rocks.insert((start.0, jdx));
                    }
                } else {
                    if start.1 > lowest_rock_y {
                        lowest_rock_y = end.1
                    }
                    for jdx in end.1..=start.1 {
                        rocks.insert((start.0, jdx));
                    }
                }
            } else if start.1 == end.1 {
                if start.1 > lowest_rock_y {
                    lowest_rock_y = end.1
                }
                if start.0 < end.0 {
                    for jdx in start.0..=end.0 {
                        rocks.insert((jdx, start.1));
                    }
                } else {
                    for jdx in end.0..=start.0 {
                        rocks.insert((jdx, start.1));
                    }
                }
            }
        }
    }

    for idx in 0..1000 {
        rocks.insert((idx, lowest_rock_y + 2));
    }

    println!("{:?}", rocks.len());

    let mut sand_at_rest: HashSet<(i32, i32)> = HashSet::new();
    'sand_drop: loop {
        let mut sand_pos = (500, 0);
        let sand_rest_clone = sand_at_rest.clone();
        let blocking: HashSet<&(i32, i32)> = rocks.union(&sand_rest_clone).collect();
        loop {
            // Check blocked
            if blocking.contains(&(500, 0)) {
                break 'sand_drop;
            }

            // Check Below
            if !blocking.contains(&(sand_pos.0, sand_pos.1 + 1)) {
                sand_pos = (sand_pos.0, sand_pos.1 + 1);
                continue;
            };

            // Check below left
            if !blocking.contains(&(sand_pos.0 - 1, sand_pos.1 + 1)) {
                sand_pos = (sand_pos.0 - 1, sand_pos.1 + 1);
                continue;
            };

            // Check below right
            if !blocking.contains(&(sand_pos.0 + 1, sand_pos.1 + 1)) {
                sand_pos = (sand_pos.0 + 1, sand_pos.1 + 1);
                continue;
            };

            sand_at_rest.insert(sand_pos);
            break;
        }
    }
    println!("{:?}", sand_at_rest.len());
}
