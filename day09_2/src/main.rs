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

    let mut rope = [(0, 0); 10];

    for inst in instructions.into_iter() {
        for _ in 0..inst.1 {
            for knot_idx in 0..rope.len() {
                let mut current_knot = rope.get_mut(knot_idx).unwrap().to_owned();
                if knot_idx == 0 {
                    match inst.0 {
                        "L" => current_knot = (current_knot.0 - 1, current_knot.1),
                        "U" => current_knot = (current_knot.0, current_knot.1 + 1),
                        "R" => current_knot = (current_knot.0 + 1, current_knot.1),
                        "D" => current_knot = (current_knot.0, current_knot.1 - 1),
                        _ => (),
                    }
                } else {
                    let leading_knot = rope.get_mut(knot_idx - 1).unwrap().to_owned();
                    let x_distance = leading_knot.0 - current_knot.0;
                    let y_distance = leading_knot.1 - current_knot.1;
                    // println!(
                    //     "{current_knot:?} => {leading_knot:?} | {} , {}",
                    //     x_distance, y_distance
                    // );

                    if x_distance > 1 || x_distance < -1 || y_distance > 1 || y_distance < -1 {
                        // non diag
                        if current_knot.0 == leading_knot.0 || current_knot.1 == leading_knot.1 {
                            if leading_knot.0 > current_knot.0 {
                                current_knot = (current_knot.0 + 1, current_knot.1);
                            } else if leading_knot.0 < current_knot.0 {
                                current_knot = (current_knot.0 - 1, current_knot.1);
                            } else if leading_knot.1 > current_knot.1 {
                                current_knot = (current_knot.0, current_knot.1 + 1);
                            } else if leading_knot.1 < current_knot.1 {
                                current_knot = (current_knot.0, current_knot.1 - 1);
                            }
                        } else {
                            //diag
                            if (x_distance == 1 && y_distance == 2)
                                || (x_distance == 2 && y_distance == 1)
                                || (x_distance == 2 && y_distance == 2)
                            {
                                current_knot = (current_knot.0 + 1, current_knot.1 + 1);
                            } else if (x_distance == 1 && y_distance == -2)
                                || (x_distance == 2 && y_distance == -1)
                                || (x_distance == 2 && y_distance == -2)
                            {
                                current_knot = (current_knot.0 + 1, current_knot.1 - 1);
                            } else if (x_distance == -1 && y_distance == -2)
                                || (x_distance == -2 && y_distance == -1)
                                || (x_distance == -2 && y_distance == -2)
                            {
                                current_knot = (current_knot.0 - 1, current_knot.1 - 1);
                            } else if (x_distance == -1 && y_distance == 2)
                                || (x_distance == -2 && y_distance == 1)
                                || (x_distance == -2 && y_distance == 2)
                            {
                                current_knot = (current_knot.0 - 1, current_knot.1 + 1);
                            }
                        }
                    }
                }
                rope[knot_idx] = current_knot;
            }
            visited.insert(rope[9]);
        }
    }
    println!("{}", visited.len());
}
