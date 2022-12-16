#[derive(Debug, Clone)]
enum Operation {
    MULTIPLY(usize),
    ADD(usize),
    POWER(usize),
}

#[derive(Debug, Clone)]
struct Test {
    divisor: usize,
    success: usize,
    failure: usize,
}

#[derive(Debug, Clone)]
struct Monkey {
    op: Operation,
    test: Test,
}

fn main() {
    let input = include_str!("../input.txt");
    let monkeys_strings: Vec<&str> = input.split_terminator("\n\n").collect();

    let mut cd = 1;
    let mut item_lists = vec![];
    let mut monkeys = vec![];
    for m_str in monkeys_strings.into_iter() {
        let lines: Vec<&str> = m_str.lines().collect();

        let items = lines[1].split_terminator(":").collect::<Vec<&str>>()[1]
            .split_terminator(",")
            .collect::<Vec<&str>>()
            .into_iter()
            .map(|x| x.trim().parse::<usize>().unwrap())
            .collect::<Vec<usize>>();

        let op_parts: Vec<&str> = lines[2].split_terminator("= old").collect::<Vec<&str>>()[1]
            .split_whitespace()
            .collect();

        let op = if op_parts[0] == "*" {
            if op_parts[1] == "old" {
                Operation::POWER(2)
            } else {
                Operation::MULTIPLY(op_parts[1].parse::<usize>().unwrap())
            }
        } else {
            Operation::ADD(op_parts[1].parse::<usize>().unwrap())
        };

        let divisor = lines[3].split_terminator("by").collect::<Vec<&str>>()[1]
            .trim()
            .parse::<usize>()
            .unwrap();
        let success = lines[4].split_terminator("monkey").collect::<Vec<&str>>()[1]
            .trim()
            .parse::<usize>()
            .unwrap();
        let failure = lines[5].split_terminator("monkey").collect::<Vec<&str>>()[1]
            .trim()
            .parse::<usize>()
            .unwrap();

        let test = Test {
            divisor,
            success,
            failure,
        };

        cd *= divisor;
        let monkey = Monkey { op, test };
        monkeys.push(monkey);

        item_lists.push(items);
    }

    let mut inspection_count = vec![0; item_lists.len()];

    for _ in 0..10000 {
        for idx in 0..item_lists.len() {
            let monkey = &monkeys.clone()[idx];
            for jdx in 0..item_lists[idx].len() {
                let mut score = item_lists[idx][jdx];
                match monkey.op {
                    Operation::MULTIPLY(val) => score *= val,
                    Operation::ADD(val) => score += val,
                    Operation::POWER(val) => score = score.pow(val as u32),
                }

                score %= cd;

                let test_passed = score % monkey.test.divisor == 0;
                if test_passed {
                    item_lists[monkey.test.success].push(score);
                } else {
                    item_lists[monkey.test.failure].push(score);
                }
                inspection_count[idx] += 1;
            }
            item_lists[idx] = vec![]
        }
    }
    inspection_count.sort();
    inspection_count.reverse();
    let res = inspection_count.iter().take(2).product::<isize>();

    println!("{res:?}")
}
