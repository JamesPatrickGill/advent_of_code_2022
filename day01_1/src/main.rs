fn main() {
    let input = include_str!("../input.txt");
    let elves: Vec<&str> = input.split("\n\n").collect();
    let calories: Vec<i32> = elves
        .into_iter()
        .map(|elf| {
            let foods: Vec<i32> = elf
                .split("\n")
                .map(|x| x.to_string().parse::<i32>().unwrap_or(0))
                .collect();
            return foods.iter().sum();
        })
        .collect();
    let max = calories.iter().max();
    println!("{:?}", calories);
    println!("{:?}", max)
}
