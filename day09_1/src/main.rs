use std::collections::HashSet;

fn main() {
    let input = include_str!("../input.txt");
    let instructions: Vec<(&str, isize)> = input
        .split_terminator("\n")
        .map(|line| {
            let parts: Vec<&str> = line.split_whitespace().collect();
            return (parts[0], parts[1].parse().unwrap());
        })
        .collect();

    let start_point: (isize, isize) = (0, 0);
    let mut visited: HashSet<(isize, isize)> = HashSet::new();
    visited.insert(start_point);

    let mut head_pos: (isize, isize) = (0, 0);
    let mut tail_pos: (isize, isize) = (0, 0);

    for inst in instructions.into_iter() {
        for _idx in 0..inst.1 {
            match inst.0 {
                "L" => head_pos = (head_pos.0 - 1, head_pos.1),
                "U" => head_pos = (head_pos.0, head_pos.1 + 1),
                "R" => head_pos = (head_pos.0 + 1, head_pos.1),
                "D" => head_pos = (head_pos.0, head_pos.1 - 1),
                _ => (),
            }

            let x_distance = head_pos.0 - tail_pos.0;
            let y_distance = head_pos.1 - tail_pos.1;

            if x_distance > 1 || x_distance < -1 || y_distance > 1 || y_distance < -1 {
                // non diag
                if tail_pos.0 == head_pos.0 || tail_pos.1 == head_pos.1 {
                    if head_pos.0 > tail_pos.0 {
                        tail_pos = (tail_pos.0 + 1, tail_pos.1);
                    } else if head_pos.0 < tail_pos.0 {
                        tail_pos = (tail_pos.0 - 1, tail_pos.1);
                    } else if head_pos.1 > tail_pos.1 {
                        tail_pos = (tail_pos.0, tail_pos.1 + 1);
                    } else if head_pos.1 < tail_pos.1 {
                        tail_pos = (tail_pos.0, tail_pos.1 - 1);
                    }
                } else {
                    //diag
                    if (x_distance == 1 && y_distance == 2) || (x_distance == 2 && y_distance == 1)
                    {
                        tail_pos = (tail_pos.0 + 1, tail_pos.1 + 1);
                    } else if (x_distance == 1 && y_distance == -2)
                        || (x_distance == 2 && y_distance == -1)
                    {
                        tail_pos = (tail_pos.0 + 1, tail_pos.1 - 1);
                    } else if (x_distance == -1 && y_distance == -2)
                        || (x_distance == -2 && y_distance == -1)
                    {
                        tail_pos = (tail_pos.0 - 1, tail_pos.1 - 1);
                    } else if (x_distance == -1 && y_distance == 2)
                        || (x_distance == -2 && y_distance == 1)
                    {
                        tail_pos = (tail_pos.0 - 1, tail_pos.1 + 1);
                    }
                }
            }
            visited.insert(tail_pos);
        }
    }
    println!("{}", visited.len());
}
