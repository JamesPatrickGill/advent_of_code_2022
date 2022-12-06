use std::collections::HashSet;

fn main() {
    let input = include_str!("../input.txt");

    for idx in 0..input.len() - 5 {
        let char_seq = input.get(idx..idx + 4).unwrap();

        let mut set = HashSet::new();

        for char in char_seq.chars() {
            set.insert(char);
        }

        if set.len() == 4 {
            println!("{:?}", idx + 4);
            break;
        }
    }
}
