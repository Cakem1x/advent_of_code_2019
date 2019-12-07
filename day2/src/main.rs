use std::fs::read_to_string;

fn day2() {
    println!("loading initial state:");
    let input_state: Vec<i32> = string_to_program_state(&read_to_string("input_day2.txt").unwrap());
    println!("{:?}", input_state);
    'outer: for noun in 0..100 {
        for verb in 0..100 {
            let mut altered_state = input_state.to_vec();
            altered_state[1] = noun;
            altered_state[2] = verb;
            //println!("Altered state:\n{:?}", altered_state);
            run_program(&mut altered_state);
            //println!("Final state:\n{:?}", altered_state);
            let result = altered_state[0];
            println!(
                "With noun {} and verb {}: result is {}.",
                noun, verb, result
            );
            if result == 19690720 {
                println!(
                    "Found a correct noun, verb combination! 100 * noun + verb = {}",
                    100 * noun + verb
                );
                break 'outer;
            }
        }
    }
}

fn string_to_program_state(input_string: &str) -> Vec<i32> {
    return input_string[..input_string.len() - 1] // get rid of \n character
           .split(",")
           .map(|s| s.parse::<i32>().expect("failed to convert input to i32"))
           .collect();
}

fn main() {
    day2()
}

fn run_program(state: &mut [i32]) {
    let mut instruction_pointer = 0;
    let mut step_counter = 0;
    loop {
        let (new_instr_ptr, output) = step_program(instruction_pointer, state);
        if new_instr_ptr == instruction_pointer {
            break;
        }
        if output.is_some() {
            println!{"#{} &{}: {}", step_counter, instruction_pointer, output.unwrap()};
        }
        instruction_pointer = new_instr_ptr;
        step_counter += 1;
    }
    println!{"#{} &{}: Terminate.", step_counter, instruction_pointer};
}

/// Returns a tuple: the new position of the instruction pointer and an option for some output the program may have generated
fn step_program(instruction_pointer: usize, state: &mut [i32]) -> (usize, Option<i32>) {
    if state[instruction_pointer] == 1 {
        // addition
        let first_operand_pointer = state[instruction_pointer + 1] as usize;
        let second_operand_pointer = state[instruction_pointer + 2] as usize;
        let result_pointer = state[instruction_pointer + 3] as usize;
        let first_operand = state[first_operand_pointer];
        let second_operand = state[second_operand_pointer];
        let result = first_operand + second_operand;
        state[result_pointer] = result;
        //println!("{} + {} = {}", first_operand, second_operand, result);
        return (instruction_pointer + 4, None);
    } else if state[instruction_pointer] == 2 {
        // multiplication
        let first_operand_pointer = state[instruction_pointer + 1] as usize;
        let second_operand_pointer = state[instruction_pointer + 2] as usize;
        let result_pointer = state[instruction_pointer + 3] as usize;
        let first_operand = state[first_operand_pointer];
        let second_operand = state[second_operand_pointer];
        let result = first_operand * second_operand;
        state[result_pointer] = result;
        //println!("{} * {} = {}", first_operand, second_operand, result);
        return (instruction_pointer + 4, None);
    }
    if state[instruction_pointer] == 99 {
        // termination
        return (instruction_pointer, None);
    }
    panic!("Invalid opcode found!");
}

#[test]
fn test_opcode1_add() {
    let mut program_state = [1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50];
    assert_eq!(step_program(0, &mut program_state).0, 4);
    assert_eq!(program_state, [1, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50]);
    let mut program_state = [1, 0, 0, 0, 99];
    assert_eq!(step_program(0, &mut program_state).0, 4);
    assert_eq!(program_state, [2, 0, 0, 0, 99]);
}

#[test]
fn test_opcode1_mul() {
    let mut program_state = [1, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50];
    assert_eq!(step_program(4, &mut program_state).0, 8);
    assert_eq!(
        program_state,
        [3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50]
    );
    let mut program_state = [2, 3, 0, 3, 99];
    assert_eq!(step_program(0, &mut program_state).0, 4);
    assert_eq!(program_state, [2, 3, 0, 6, 99]);
    let mut program_state = [2, 4, 4, 5, 99, 0];
    assert_eq!(step_program(0, &mut program_state).0, 4);
    assert_eq!(program_state, [2, 4, 4, 5, 99, 9801]);
}

#[test]
fn test_run_program() {
    let mut program_state = [1, 1, 1, 4, 99, 5, 6, 0, 99];
    run_program(&mut program_state);
    assert_eq!(program_state, [30, 1, 1, 4, 2, 5, 6, 0, 99]);
}

#[test]
fn test_day2_works() {
    let mut program_state = [1, 53, 79, 3, 1, 1, 2, 3, 1, 3, 4, 3, 1, 5, 0, 3, 2, 1, 6, 19, 1, 9, 19, 23, 1, 6, 23, 27, 1, 10, 27, 31, 1, 5, 31, 35, 2, 6, 35, 39, 1, 5, 39, 43, 1, 5, 43, 47, 2, 47, 6, 51, 1, 51, 5, 55, 1, 13, 55, 59, 2, 9, 59, 63, 1, 5, 63, 67, 2, 67, 9, 71, 1, 5, 71, 75, 2, 10, 75, 79, 1, 6, 79, 83, 1, 13, 83, 87, 1, 10, 87, 91, 1, 91, 5, 95, 2, 95, 10, 99, 2, 9, 99, 103, 1, 103, 6, 107, 1, 107, 10, 111, 2, 111, 10, 115, 1, 115, 6, 119, 2, 119, 9, 123, 1, 123, 6, 127, 2, 127, 10, 131, 1, 131, 6, 135, 2, 6, 135, 139, 1, 139, 5, 143, 1, 9, 143, 147, 1, 13, 147, 151, 1, 2, 151, 155, 1, 10, 155, 0, 99, 2, 14, 0, 0];
    run_program(&mut program_state);
    assert_eq!(program_state[0], 19690720);
}
