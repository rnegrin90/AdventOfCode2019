fn opcode_1(operator_1: i32, operator_2: i32) -> i32 {
    operator_1 + operator_2
}

fn opcode_2(operator_1: i32, operator_2: i32) -> i32 {
    operator_1 * operator_2
}

fn evaluate_memory(instructions: &mut Vec<i32>) -> i32 {
    let mut current_position = 0;
    let result: i32;
    loop {
        match instructions[current_position] {
            1 => {
                let result_location = instructions[current_position + 3] as usize;
                let operand_1_location = instructions[current_position + 1] as usize;
                let operand_2_location = instructions[current_position + 2] as usize;
                instructions[result_location] = opcode_1(
                    instructions[operand_1_location], 
                    instructions[operand_2_location]
                );
                current_position += 4;
            },
            2 => {
                let result_location = instructions[current_position + 3] as usize;
                let operand_1_location = instructions[current_position + 1] as usize;
                let operand_2_location = instructions[current_position + 2] as usize;
                instructions[result_location] = opcode_2(
                    instructions[operand_1_location], 
                    instructions[operand_2_location]
                );
                current_position += 4;
            },
            99 => {
                result = instructions[0];
                break;
            },
            _ => panic!("Wrong opcode!")
        };
    }

    result
}

fn find_noun_and_verb(instructions: Vec<i32>, target_output: i32) -> (i32, i32) {
    let mut result: (i32, i32) = (0, 0);
    let mut current_instructions = instructions.clone();
    'noun: for noun in 0..=99 {
        'verb: for verb in 0..=99 {
            current_instructions[1] = noun;
            current_instructions[2] = verb;
            if evaluate_memory(&mut current_instructions) == target_output {
                result = (noun, verb);
                break 'noun
            }
            current_instructions = instructions.clone();
        }
    }

    result
}

pub fn solution(star: i8, input: &String) -> i32 {
    let mut instructions: Vec<i32> = input.split(',').map(|x| x.parse::<i32>().unwrap()).collect();

    let result: i32 = match star {
        1 => {
            instructions[1] = 12;
            instructions[2] = 2;
            evaluate_memory(&mut instructions)
        },
        2 => {
            let (noun, verb) = find_noun_and_verb(instructions, 19690720);

            100 * noun + verb
        },
        _ => 0
    };

    result
}

#[cfg(test)]
mod tests;
