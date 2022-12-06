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

    for inst in instructions.into_iter() {
        let amount = inst[0];
        // println!("{inst:?}");
        let start_index = inst[1] - 1;
        let end_index = inst[2] - 1;

        let start_stack = parsed_stack_states.get_mut(start_index as usize).unwrap();
        let mut moving_boxes = start_stack.split_off(start_stack.len() - amount as usize);

        let end_stack = parsed_stack_states.get_mut(end_index as usize).unwrap();
        end_stack.append(&mut moving_boxes);

        println!("{parsed_stack_states:?}")
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
