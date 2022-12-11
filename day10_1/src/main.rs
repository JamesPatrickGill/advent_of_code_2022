#[derive(Debug)]
enum Instruction {
    ADDX(isize),
    NOOP,
}

fn main() {
    let input = include_str!("../input.txt");
    let instructions: Vec<Instruction> = input
        .split_terminator("\n")
        .map(|line| {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts[0] == "noop" {
                return Instruction::NOOP;
            }

            return Instruction::ADDX(parts[1].parse().unwrap());
        })
        .collect();

    let mut x = 1;
    let mut cycles = 1;

    let mut sig_values = vec![];

    for inst in instructions.into_iter() {
        let num_cycles = match inst {
            Instruction::ADDX(_) => 2,
            Instruction::NOOP => 1,
        };

        for _ in 0..num_cycles {
            if cycles == 20
                || cycles == 60
                || cycles == 100
                || cycles == 140
                || cycles == 180
                || cycles == 220
            {
                sig_values.push(x * cycles);
            }
            cycles += 1
        }

        match inst {
            Instruction::ADDX(amount) => x += amount,
            Instruction::NOOP => (),
        };
    }

    println!("{sig_values:?}");
    println!("{:?}", sig_values.iter().sum::<isize>());
}
