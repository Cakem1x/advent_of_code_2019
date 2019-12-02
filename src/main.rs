use std::fs::read_to_string;

fn main() {
    println!("loading initial state:");
    let input_string = read_to_string("input.txt").unwrap();
    let mut input_state: Vec<u32> = input_string[..input_string.len() - 1] // get rid of \n character
        .split(",")
        .map(|s| s.parse::<u32>().expect("failed to convert input to u32"))
        .collect();
    println!("{:?}", input_state);
    input_state[1] = 12;
    input_state[2] = 2;
    println!("Altered state:\n{:?}", input_state);
    run_program(&mut input_state);
    println!("Final state:\n{:?}", input_state);
}

fn run_program(state: &mut [u32]) {
    let mut instruction_pointer = 0;
    while !step_program(instruction_pointer, state) {
        instruction_pointer += 4;
    }
}

/// Returns whether the program should terminate
fn step_program(instruction_pointer: usize, state: &mut [u32]) -> bool {
    if state[instruction_pointer] == 1 {
        // addition
        let first_operand_pointer = state[instruction_pointer + 1] as usize;
        let second_operand_pointer = state[instruction_pointer + 2] as usize;
        let result_pointer = state[instruction_pointer + 3] as usize;
        let first_operand = state[first_operand_pointer];
        let second_operand = state[second_operand_pointer];
        let result = first_operand + second_operand;
        state[result_pointer] = result;
        println!("{} + {} = {}", first_operand, second_operand, result);
        return false;
    } else if state[instruction_pointer] == 2 {
        // multiplication
        let first_operand_pointer = state[instruction_pointer + 1] as usize;
        let second_operand_pointer = state[instruction_pointer + 2] as usize;
        let result_pointer = state[instruction_pointer + 3] as usize;
        let first_operand = state[first_operand_pointer];
        let second_operand = state[second_operand_pointer];
        let result = first_operand * second_operand;
        state[result_pointer] = result;
        println!("{} * {} = {}", first_operand, second_operand, result);
        return false;
    }
    if state[instruction_pointer] == 99 {
        // termination
        return true;
    }
    panic!("Invalid opcode found!");
}

#[test]
fn test_opcode1_add() {
    let mut program_state = [1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50];
    assert_eq!(step_program(0, &mut program_state), false);
    assert_eq!(program_state, [1, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50]);
    let mut program_state = [1, 0, 0, 0, 99];
    assert_eq!(step_program(0, &mut program_state), false);
    assert_eq!(program_state, [2, 0, 0, 0, 99]);
}

#[test]
fn test_opcode1_mul() {
    let mut program_state = [1, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50];
    assert_eq!(step_program(4, &mut program_state), false);
    assert_eq!(
        program_state,
        [3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50]
    );
    let mut program_state = [2, 3, 0, 3, 99];
    assert_eq!(step_program(0, &mut program_state), false);
    assert_eq!(program_state, [2, 3, 0, 6, 99]);
    let mut program_state = [2, 4, 4, 5, 99, 0];
    assert_eq!(step_program(0, &mut program_state), false);
    assert_eq!(program_state, [2, 4, 4, 5, 99, 9801]);
}

#[test]
fn test_run_program() {
    let mut program_state = [1, 1, 1, 4, 99, 5, 6, 0, 99];
    run_program(&mut program_state);
    assert_eq!(program_state, [30, 1, 1, 4, 2, 5, 6, 0, 99]);
}
