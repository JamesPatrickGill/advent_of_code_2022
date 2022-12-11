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

    let mut crt_pixel = 0;
    let mut current_row = vec![];
    let mut crt_screen = vec![];

    for inst in instructions.into_iter() {
        let num_cycles = match inst {
            Instruction::ADDX(_) => 2,
            Instruction::NOOP => 1,
        };

        for _ in 0..num_cycles {
            // println!("{x:?}, {crt_pixel:?}");
            if (crt_pixel % 40) - x >= -1 && (crt_pixel % 40) - x <= 1 {
                current_row.push("#");
            } else {
                current_row.push(" ");
            }
            cycles += 1;
            crt_pixel += 1;
            if cycles % 40 == 1 {
                crt_screen.push(current_row);
                current_row = vec![];
            }
        }

        match inst {
            Instruction::ADDX(amount) => x += amount,
            Instruction::NOOP => (),
        };
    }

    let result: Vec<String> = crt_screen.into_iter().map(|row| row.join("")).collect();
    for i in result.into_iter() {
        println!("{i:?}");
    }
}
