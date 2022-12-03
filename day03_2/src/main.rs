use std::collections::HashSet;
fn main() {
    let input = include_str!("../input.txt");
    let bags: Vec<&str> = input.split_terminator("\n").collect();

    let mut badges = vec![];
    let bag_triplets: Vec<&[&str]> = bags.chunks(3).collect();
    for trips in bag_triplets.into_iter() {
        let mut possible = HashSet::new();

        let set: HashSet<char> = trips.get(0).unwrap().chars().collect();
        let set2: HashSet<char> = trips.get(1).unwrap().chars().collect();
        let set3: HashSet<char> = trips.get(2).unwrap().chars().collect();

        for item in set2.into_iter() {
            if set.contains(&item) {
                possible.insert(item);
            }
        }
        for item in set3.into_iter() {
            if possible.contains(&item) {
                badges.push(item);
            }
        }
    }
    score(badges);
}

fn score(error: Vec<char>) {
    let mut total = 0;
    for char in error.into_iter() {
        let str_char = &String::from(char)[..];
        match str_char {
            "a" => total = total + 1,
            "b" => total = total + 2,
            "c" => total = total + 3,
            "d" => total = total + 4,
            "e" => total = total + 5,
            "f" => total = total + 6,
            "g" => total = total + 7,
            "h" => total = total + 8,
            "i" => total = total + 9,
            "j" => total = total + 10,
            "k" => total = total + 11,
            "l" => total = total + 12,
            "m" => total = total + 13,
            "n" => total = total + 14,
            "o" => total = total + 15,
            "p" => total = total + 16,
            "q" => total = total + 17,
            "r" => total = total + 18,
            "s" => total = total + 19,
            "t" => total = total + 20,
            "u" => total = total + 21,
            "v" => total = total + 22,
            "w" => total = total + 23,
            "x" => total = total + 24,
            "y" => total = total + 25,
            "z" => total = total + 26,
            "A" => total = total + 27,
            "B" => total = total + 28,
            "C" => total = total + 29,
            "D" => total = total + 30,
            "E" => total = total + 31,
            "F" => total = total + 32,
            "G" => total = total + 33,
            "H" => total = total + 34,
            "I" => total = total + 35,
            "J" => total = total + 36,
            "K" => total = total + 37,
            "L" => total = total + 38,
            "M" => total = total + 39,
            "N" => total = total + 40,
            "O" => total = total + 41,
            "P" => total = total + 42,
            "Q" => total = total + 43,
            "R" => total = total + 44,
            "S" => total = total + 45,
            "T" => total = total + 46,
            "U" => total = total + 47,
            "V" => total = total + 48,
            "W" => total = total + 49,
            "X" => total = total + 50,
            "Y" => total = total + 51,
            "Z" => total = total + 52,
            _ => total = total,
        }
    }
    println!("{:?}", total)
}
