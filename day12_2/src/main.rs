use std::collections::{HashMap, HashSet};

#[derive(Debug)]
struct Grid {
    pub width: usize,
    pub height: usize,
    pub starts: Vec<(usize, usize)>,
    pub end: (usize, usize),
    pub chars: Vec<usize>,
}

impl Grid {
    pub fn new(input: &str) -> Self {
        let lines: Vec<&str> = input.split_terminator("\n").collect();
        let width: usize = lines[0].len() as usize;
        let height: usize = lines.len() as usize;

        let mut starts: Vec<(usize, usize)> = vec![];
        let mut end: (usize, usize) = (0, 0);

        let chars: Vec<usize> = input
            .replace("\n", "")
            .char_indices()
            .map(|(idx, ch)| match ch {
                'S' => {
                    let y_idx = idx as usize / width;
                    let x_idx = idx as usize % width;
                    starts.push((x_idx, y_idx));
                    return 1;
                }
                'E' => {
                    let y_idx = idx as usize / width;
                    let x_idx = idx as usize % width;
                    end = (x_idx, y_idx);
                    return 26;
                }
                'a' => {
                    let y_idx = idx as usize / width;
                    let x_idx = idx as usize % width;
                    starts.push((x_idx, y_idx));
                    return 1;
                }
                'b' => 2,
                'c' => 3,
                'd' => 4,
                'e' => 5,
                'f' => 6,
                'g' => 7,
                'h' => 8,
                'i' => 9,
                'j' => 10,
                'k' => 11,
                'l' => 12,
                'm' => 13,
                'n' => 14,
                'o' => 15,
                'p' => 16,
                'q' => 17,
                'r' => 18,
                's' => 19,
                't' => 20,
                'u' => 21,
                'v' => 22,
                'w' => 23,
                'x' => 24,
                'y' => 25,
                'z' => 26,
                _ => unreachable!(),
            })
            .collect();
        Grid {
            width,
            height,
            chars,
            starts,
            end,
        }
    }

    pub fn get_value(&self, pos: (usize, usize)) -> usize {
        self.chars[((self.width * pos.1) + pos.0) as usize]
    }

    pub fn get_next_moves(&self, pos: (usize, usize)) -> HashSet<(usize, usize)> {
        let mut mvs = vec![
            (pos.0 + 1, pos.1),
            (pos.0, pos.1 + 1),
            (pos.0.wrapping_sub(1), pos.1),
            (pos.0, pos.1.wrapping_sub(1)),
        ];
        mvs.retain(|x| !(x.0 >= self.width || x.1 >= self.height));

        let current_value = self.get_value(pos);
        mvs.retain(|x| self.get_value(*x) <= current_value + 1);
        mvs.into_iter().collect::<HashSet<(usize, usize)>>()
    }
}

fn main() {
    let input = include_str!("../input.txt");
    let grid = Grid::new(input);

    let mut results = vec![];
    for start in grid.starts.clone() {
        let mut best_for_pos = HashMap::new();
        best_for_pos.insert(start, 0usize);
        let mut current_moves = vec![start];

        'outer: for depth in 0usize.. {
            let mut next_moves = vec![];
            for mv in current_moves.iter() {
                if mv == &grid.end {
                    results.push(depth);
                    break 'outer;
                }

                let possible_mvs = grid.get_next_moves(*mv);
                for p_mv in possible_mvs.into_iter() {
                    if best_for_pos.contains_key(&p_mv) {
                        continue;
                    }
                    best_for_pos.insert(p_mv, depth);
                    next_moves.push(p_mv);
                }
            }
            if next_moves.is_empty() {
                break;
            }
            current_moves = next_moves;
        }
    }
    let answer = results.iter().min();
    println!("{answer:?}")
}
