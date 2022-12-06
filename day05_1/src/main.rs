fn main() {
    let input = include_str!("../input.txt");
    let inputs: Vec<&str> = input.split_terminator("\n\n").collect();

    let mut state: Vec<&str> = inputs[0].split_terminator("\n").collect();
    state.reverse();

    let mut parsed_stack_states = vec![];
    for (idx, char) in state[0].chars().enumerate() {
        if char.is_numeric() {
            let mut stack = vec![];
            for column in 1..state.len() {
                let item = state[column].chars().nth(idx).unwrap();
                if item.is_alphabetic() {
                    stack.push(item);
                }
            }
            parsed_stack_states.push(stack)
        }
    }
    println!("{parsed_stack_states:?}");

    let instructions: Vec<Vec<u32>> = inputs[1]
        .split_terminator("\n")
        .map(|x| {
            let mut instructions_numbers: Vec<u32> = vec![];
            let mut split_inst: Vec<&str> = x.split_terminator(" ").collect();
            for word in split_inst.iter_mut() {
                if word.chars().all(char::is_numeric) {
                    instructions_numbers.push(word.parse::<u32>().unwrap())
                }
            }
            return instructions_numbers;
        })
        .collect();

    println!("{instructions:?}");

    for inst in instructions.into_iter() {
        let amount = inst[0];
        // println!("{inst:?}");
        let start_index = inst[1] - 1;
        let end_index = inst[2] - 1;

        for _ in 1..=amount {
            let moving_box = parsed_stack_states
                .get_mut(start_index as usize)
                .unwrap()
                .pop()
                .unwrap();
            parsed_stack_states
                .get_mut(end_index as usize)
                .unwrap()
                .push(moving_box);
        }
    }

    let top_boxes: String = parsed_stack_states
        .into_iter()
        .map(|stack| {
            let top = stack.last().unwrap();
            return top.clone();
        })
        .collect();

    println!("{top_boxes:?}")
}
