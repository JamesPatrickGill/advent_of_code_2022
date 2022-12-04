fn main() {
    let input = include_str!("../input.txt");
    let ranges: Vec<&str> = input.split_terminator("\n").collect();

    let mut result_count = 0;
    for range in ranges.into_iter() {
        let split_ranges: Vec<&str> = range.split(",").collect();

        let range1: Vec<i32> = split_ranges[0]
            .split("-")
            .map(|x| x.parse::<i32>().unwrap())
            .collect();
        let range2: Vec<i32> = split_ranges[1]
            .split("-")
            .map(|x| x.parse::<i32>().unwrap())
            .collect();

        if range1[1] >= range2[0] && range1[0] <= range2[1] {
            result_count = result_count + 1;
        }
    }
    println!("{result_count:?}")
}
