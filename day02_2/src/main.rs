fn main() {
    let input = include_str!("../input.txt");
    let rounds: Vec<&str> = input.split("\n").collect();

    let mut score = 0;
    for game in rounds.into_iter() {
        match game {
            "A X" => score = score + 0 + 3,
            "A Y" => score = score + 3 + 1,
            "A Z" => score = score + 6 + 2,
            "B X" => score = score + 0 + 1,
            "B Y" => score = score + 3 + 2,
            "B Z" => score = score + 6 + 3,
            "C X" => score = score + 0 + 2,
            "C Y" => score = score + 3 + 3,
            "C Z" => score = score + 6 + 1,
            _ => score = score + 0,
        };
    }

    println!("{:?}", score)
}
