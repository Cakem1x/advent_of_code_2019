/// Runs the program until it terminates, using a fixed vector of inputs. Returns a vector of output data
pub fn run(state: &mut [i32], mut input_values: Vec<i32>) -> Vec<i32> {
    let mut instruction_pointer = 0;
    let mut _step_counter = 0;
    let mut program_output = Vec::new();
    //println!("Starting new program with input {:?}", input_values);
    loop {
        let input_value = match parse_instruction(state[instruction_pointer]).0 {
            Opcode::Input => input_values.pop(),
            _ => None,
        };
        let (new_instr_ptr, output) = step_program(instruction_pointer, state, input_value);
        if new_instr_ptr == instruction_pointer {
            break;
        }
        if output.is_some() {
            //println! {"#{} &{}: {}", _step_counter, instruction_pointer, output.unwrap()};
            program_output.push(output.unwrap());
        }
        instruction_pointer = new_instr_ptr;
        _step_counter += 1;
    }
    //println! {"#{} &{}: Terminate.", step_counter, instruction_pointer};
    //println! {"Program output: {:?}", program_output};
    return program_output;
}

/// runs an amplification program using a single input until it yields a single output or until it terminates.
pub fn run_until_output_or_termination(instruction_pointer: usize, state: &mut [i32], input_value: i32) -> (usize, Option<i32>) {
    let mut current_instruction_pointer = instruction_pointer;
    loop {
        let (new_instr_ptr, output) = step_program(current_instruction_pointer, state, Some(input_value));
        if output.is_some() {
            return (new_instr_ptr, output);
        } else if new_instr_ptr == current_instruction_pointer {
            assert_eq!(output.is_none(), true);
            return (new_instr_ptr, None);
        }
        current_instruction_pointer = new_instr_ptr;
    }
}

/// runs the program until after an input instruction was executed. Used to set the phase of amplifier programs.
pub fn run_until_after_input_instruction(
    instruction_pointer: usize,
    state: &mut [i32],
    phase_setting: i32,
) -> usize {
    let mut current_instruction_pointer = instruction_pointer;
    loop {
        if parse_instruction(state[current_instruction_pointer]).0 == Opcode::Input {
            let (next_instruction_pointer, output) = step_program(current_instruction_pointer, state, Some(phase_setting));
            assert_eq!(output.is_none(), true);
            return next_instruction_pointer;
        } else {
            let (next_instruction_pointer, output) = step_program(current_instruction_pointer, state, None);
            current_instruction_pointer = next_instruction_pointer;
            assert_eq!(output.is_none(), true); // program shouldn't yield output before phase is set
        }
    }
}

/// Returns a tuple: the new position of the instruction pointer and an option for some output the program may have generated
fn step_program(
    instruction_pointer: usize,
    state: &mut [i32],
    next_input_value: Option<i32>,
) -> (usize, Option<i32>) {
    match parse_instruction(state[instruction_pointer]) {
        (Opcode::Add, pm1, pm2, pm3) => {
            let first_operand_param = state[instruction_pointer + 1];
            let second_operand_param = state[instruction_pointer + 2];
            let result_param = state[instruction_pointer + 3];
            let first_operand_value = match pm1 {
                ParameterMode::Position => state[first_operand_param as usize],
                ParameterMode::Immediate => first_operand_param,
            };
            let second_operand_value = match pm2 {
                ParameterMode::Position => state[second_operand_param as usize],
                ParameterMode::Immediate => second_operand_param,
            };
            let result_value = first_operand_value + second_operand_value;
            //println!("{} + {} = {}", first_operand_value, second_operand_value, result_value);
            let result_pointer = match pm3 {
                ParameterMode::Position => result_param as usize,
                _ => panic!(
                    "at &{} -> {}: result parameter only supports position mode.",
                    instruction_pointer, state[instruction_pointer]
                ),
            };
            state[result_pointer] = result_value;
            return (instruction_pointer + 4, None);
        }
        (Opcode::Mul, pm1, pm2, pm3) => {
            let first_operand_param = state[instruction_pointer + 1];
            let second_operand_param = state[instruction_pointer + 2];
            let result_param = state[instruction_pointer + 3];
            let first_operand_value = match pm1 {
                ParameterMode::Position => state[first_operand_param as usize],
                ParameterMode::Immediate => first_operand_param,
            };
            let second_operand_value = match pm2 {
                ParameterMode::Position => state[second_operand_param as usize],
                ParameterMode::Immediate => second_operand_param,
            };
            let result_value = first_operand_value * second_operand_value;
            //println!("{} * {} = {}", first_operand_value, second_operand_value, result_value);
            let result_pointer = match pm3 {
                ParameterMode::Position => result_param as usize,
                _ => panic!(
                    "at &{} -> {}: result parameter only supports position mode.",
                    instruction_pointer, state[instruction_pointer]
                ),
            };
            state[result_pointer] = result_value;
            return (instruction_pointer + 4, None);
        }
        (Opcode::Input, pm1, _pm2, _pm3) => {
            if next_input_value.is_some() {
                //println!("#{}: got value {} during input instruction", instruction_pointer, next_input_value.unwrap());
                let target_param = state[instruction_pointer + 1];
                let target_pointer = match pm1 {
                    ParameterMode::Position => target_param as usize,
                    _ => panic!(
                        "at &{} -> {}: target parameter only supports position mode.",
                        instruction_pointer, state[instruction_pointer]
                    ),
                };
                state[target_pointer] = next_input_value.unwrap();
                return (instruction_pointer + 2, None);
            } else {
                panic!("Encountered input instruction without having any next given input.");
            }
        }
        (Opcode::Output, pm1, _pm2, _pm3) => {
            let output_param = state[instruction_pointer + 1];
            let output_value = match pm1 {
                ParameterMode::Position => state[output_param as usize],
                ParameterMode::Immediate => output_param,
            };
            return (instruction_pointer + 2, Some(output_value));
        }
        (Opcode::JumpIfTrue, pm1, pm2, _pm3) => {
            let condition_param = state[instruction_pointer + 1];
            let condition_value = match pm1 {
                ParameterMode::Position => state[condition_param as usize],
                ParameterMode::Immediate => condition_param,
            };
            let jump_target_param = state[instruction_pointer + 2];
            let jump_target_value = match pm2 {
                ParameterMode::Position => state[jump_target_param as usize] as usize,
                ParameterMode::Immediate => jump_target_param as usize,
            };
            if condition_value != 0 {
                return (jump_target_value, None);
            } else {
                return (instruction_pointer + 3, None);
            }
        }
        (Opcode::JumpIfFalse, pm1, pm2, _pm3) => {
            let condition_param = state[instruction_pointer + 1];
            let condition_value = match pm1 {
                ParameterMode::Position => state[condition_param as usize],
                ParameterMode::Immediate => condition_param,
            };
            let jump_target_param = state[instruction_pointer + 2];
            let jump_target_value = match pm2 {
                ParameterMode::Position => state[jump_target_param as usize] as usize,
                ParameterMode::Immediate => jump_target_param as usize,
            };
            if condition_value == 0 {
                return (jump_target_value, None);
            } else {
                return (instruction_pointer + 3, None);
            }
        }
        (Opcode::LessThan, pm1, pm2, pm3) => {
            let first_operand_param = state[instruction_pointer + 1];
            let first_operand_value = match pm1 {
                ParameterMode::Position => state[first_operand_param as usize],
                ParameterMode::Immediate => first_operand_param,
            };
            let second_operand_param = state[instruction_pointer + 2];
            let second_operand_value = match pm2 {
                ParameterMode::Position => state[second_operand_param as usize],
                ParameterMode::Immediate => second_operand_param,
            };
            let result_param = state[instruction_pointer + 3];
            let result_ptr = match pm3 {
                ParameterMode::Position => result_param as usize,
                _ => panic!(
                    "at &{} -> {}: result parameter only supports position mode.",
                    instruction_pointer, state[instruction_pointer]
                ),
            };
            if first_operand_value < second_operand_value {
                state[result_ptr] = 1;
            } else {
                state[result_ptr] = 0;
            }
            return (instruction_pointer + 4, None);
        }
        (Opcode::Equals, pm1, pm2, pm3) => {
            let first_operand_param = state[instruction_pointer + 1];
            let first_operand_value = match pm1 {
                ParameterMode::Position => state[first_operand_param as usize],
                ParameterMode::Immediate => first_operand_param,
            };
            let second_operand_param = state[instruction_pointer + 2];
            let second_operand_value = match pm2 {
                ParameterMode::Position => state[second_operand_param as usize],
                ParameterMode::Immediate => second_operand_param,
            };
            let result_param = state[instruction_pointer + 3];
            let result_ptr = match pm3 {
                ParameterMode::Position => result_param as usize,
                _ => panic!(
                    "at &{} -> {}: result parameter only supports position mode.",
                    instruction_pointer, state[instruction_pointer]
                ),
            };
            if first_operand_value == second_operand_value {
                state[result_ptr] = 1;
            } else {
                state[result_ptr] = 0;
            }
            return (instruction_pointer + 4, None);
        }
        (Opcode::Terminate, _pm1, _pm2, _pm3) => {
            return (instruction_pointer, None);
        } //instruction => panic!("opcode {:?} not supported!", instruction.0),
    }
}

#[derive(Debug, PartialEq)]
enum Opcode {
    Add,
    Mul,
    Input,
    Output,
    JumpIfTrue,
    JumpIfFalse,
    LessThan,
    Equals,
    Terminate,
}

#[derive(Debug, PartialEq)]
enum ParameterMode {
    Position,
    Immediate,
}

fn parse_instruction(opcode_int: i32) -> (Opcode, ParameterMode, ParameterMode, ParameterMode) {
    let opcode = match opcode_int % 100 {
        1 => Opcode::Add,
        2 => Opcode::Mul,
        3 => Opcode::Input,
        4 => Opcode::Output,
        5 => Opcode::JumpIfTrue,
        6 => Opcode::JumpIfFalse,
        7 => Opcode::LessThan,
        8 => Opcode::Equals,
        99 => Opcode::Terminate,
        x => panic!("Invalid opcode found: {}!", x),
    };
    let pm_first = match opcode_int / 100 % 10 {
        0 => ParameterMode::Position,
        1 => ParameterMode::Immediate,
        x => panic!("Invalid parameter mode fuond: {}!", x),
    };
    let pm_second = match opcode_int / 1000 % 10 {
        0 => ParameterMode::Position,
        1 => ParameterMode::Immediate,
        x => panic!("Invalid parameter mode fuond: {}!", x),
    };
    let pm_third = match opcode_int / 10000 % 10 {
        0 => ParameterMode::Position,
        1 => ParameterMode::Immediate,
        x => panic!("Invalid parameter mode fuond: {}!", x),
    };
    return (opcode, pm_first, pm_second, pm_third);
}

#[test]
fn program_with_less_than_in_immediate_mode() {
    for input in 0..8 {
        let mut program_state = [1107, input, 8, 1, 4, 1, 99];
        run(&mut program_state, Vec::new());
        assert_eq!(program_state[1], 1);
    }
    for input in 8..12 {
        let mut program_state = [1107, input, 8, 1, 4, 1, 99];
        run(&mut program_state, Vec::new());
        assert_eq!(program_state[1], 0);
    }
}

#[test]
fn program_with_equals_in_immediate_mode() {
    for input in 0..8 {
        let mut program_state = [1108, input, 8, 1, 4, 1, 99];
        run(&mut program_state, Vec::new());
        assert_eq!(program_state[1], 0);
    }
    let input = 8;
    let mut program_state = [1108, input, 8, 1, 4, 1, 99];
    run(&mut program_state, Vec::new());
    assert_eq!(program_state[1], 1);
    for input in 9..12 {
        let mut program_state = [1108, input, 8, 1, 4, 1, 99];
        run(&mut program_state, Vec::new());
        assert_eq!(program_state[1], 0);
    }
}

#[test]
fn program_with_less_than_in_position_mode() {
    for input in 0..8 {
        let mut program_state = [7, 7, 8, 7, 4, 7, 99, input, 8];
        run(&mut program_state, Vec::new());
        assert_eq!(program_state[7], 1);
    }
    for input in 8..12 {
        let mut program_state = [7, 7, 8, 7, 4, 7, 99, input, 8];
        run(&mut program_state, Vec::new());
        assert_eq!(program_state[7], 0);
    }
}

#[test]
fn program_with_equals_in_position_mode() {
    for input in 0..8 {
        let mut program_state = [8, 7, 8, 7, 4, 7, 99, input, 8];
        run(&mut program_state, Vec::new());
        assert_eq!(program_state[7], 0);
    }
    let input = 8;
    let mut program_state = [8, 7, 8, 7, 4, 7, 99, input, 8];
    run(&mut program_state, Vec::new());
    assert_eq!(program_state[7], 1);
    for input in 9..12 {
        let mut program_state = [8, 7, 8, 7, 4, 7, 99, input, 8];
        run(&mut program_state, Vec::new());
        assert_eq!(program_state[7], 0);
    }
}

#[test]
fn program_with_jump_in_position_mode() {
    let input = 0;
    let mut program_state = [6, 10, 13, 1, 11, 12, 11, 4, 11, 99, input, 0, 1, 9];
    run(&mut program_state, Vec::new());
    assert_eq!(program_state[11], 0);
    let input = 3;
    let mut program_state = [6, 10, 13, 1, 11, 12, 11, 4, 11, 99, input, 0, 1, 9];
    run(&mut program_state, Vec::new());
    assert_eq!(program_state[11], 1);
}

#[test]
fn program_with_negative_immediate_values() {
    let mut program_state = [1101, 100, -1, 4, 0];
    let instruction = parse_instruction(program_state[0]);
    assert_eq!(instruction.0, Opcode::Add);
    assert_eq!(instruction.1, ParameterMode::Immediate);
    assert_eq!(instruction.2, ParameterMode::Immediate);
    assert_eq!(instruction.3, ParameterMode::Position);
    run(&mut program_state, Vec::new());
    assert_eq!(program_state, [1101, 100, -1, 4, 99]);
}

#[test]
fn opcode_add() {
    let mut program_state = [1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50];
    assert_eq!(step_program(0, &mut program_state, None).0, 4);
    assert_eq!(program_state, [1, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50]);
    let mut program_state = [1, 0, 0, 0, 99];
    assert_eq!(step_program(0, &mut program_state, None).0, 4);
    assert_eq!(program_state, [2, 0, 0, 0, 99]);
}

#[test]
fn opcode_output_supports_immediate_mode() {
    let mut program_state = [104, 0, 99];
    run(&mut program_state, Vec::new()); // should output 0, but testing that seems too complicated
}

#[test]
fn opcode_mul() {
    let mut program_state = [1, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50];
    assert_eq!(step_program(4, &mut program_state, None).0, 8);
    assert_eq!(
        program_state,
        [3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50]
    );
    let mut program_state = [2, 3, 0, 3, 99];
    assert_eq!(step_program(0, &mut program_state, None).0, 4);
    assert_eq!(program_state, [2, 3, 0, 6, 99]);
    let mut program_state = [2, 4, 4, 5, 99, 0];
    assert_eq!(step_program(0, &mut program_state, None).0, 4);
    assert_eq!(program_state, [2, 4, 4, 5, 99, 9801]);
}

#[test]
fn mini_program() {
    let mut program_state = [1, 1, 1, 4, 99, 5, 6, 0, 99];
    run(&mut program_state, Vec::new());
    assert_eq!(program_state, [30, 1, 1, 4, 2, 5, 6, 0, 99]);
}

#[test]
#[should_panic]
fn parse_instruction_panics() {
    let instruction = 28;
    parse_instruction(instruction);
}

#[test]
fn parse_instruction_add() {
    let instruction = 1;
    assert_eq!(parse_instruction(instruction).0, Opcode::Add);
    assert_eq!(parse_instruction(instruction).1, ParameterMode::Position);
    assert_eq!(parse_instruction(instruction).2, ParameterMode::Position);
    assert_eq!(parse_instruction(instruction).3, ParameterMode::Position);
}

#[test]
fn parse_instruction_mul() {
    let instruction = 2;
    assert_eq!(parse_instruction(instruction).0, Opcode::Mul);
    assert_eq!(parse_instruction(instruction).1, ParameterMode::Position);
    assert_eq!(parse_instruction(instruction).2, ParameterMode::Position);
    assert_eq!(parse_instruction(instruction).3, ParameterMode::Position);
}

#[test]
fn parse_instruction_input() {
    let instruction = 3;
    assert_eq!(parse_instruction(instruction).0, Opcode::Input);
    assert_eq!(parse_instruction(instruction).1, ParameterMode::Position);
}

#[test]
fn parse_instruction_output() {
    let instruction = 4;
    assert_eq!(parse_instruction(instruction).0, Opcode::Output);
    assert_eq!(parse_instruction(instruction).1, ParameterMode::Position);
}

#[test]
fn parse_instruction_jump_if_true() {
    let instruction = 5;
    assert_eq!(parse_instruction(instruction).0, Opcode::JumpIfTrue);
    assert_eq!(parse_instruction(instruction).1, ParameterMode::Position);
    assert_eq!(parse_instruction(instruction).2, ParameterMode::Position);
    let instruction = 1105;
    assert_eq!(parse_instruction(instruction).0, Opcode::JumpIfTrue);
    // parse_instruction doesn't check whether given mode is valid
    assert_eq!(parse_instruction(instruction).1, ParameterMode::Immediate);
    assert_eq!(parse_instruction(instruction).2, ParameterMode::Immediate);
}

#[test]
fn parse_instruction_jump_if_false() {
    let instruction = 6;
    assert_eq!(parse_instruction(instruction).0, Opcode::JumpIfFalse);
}

#[test]
fn parse_instruction_less_than() {
    let instruction = 107;
    assert_eq!(parse_instruction(instruction).0, Opcode::LessThan);
}

#[test]
fn parse_instruction_equals() {
    let instruction = 8;
    assert_eq!(parse_instruction(instruction).0, Opcode::Equals);
}

#[test]
fn parse_instruction_omit_zero() {
    assert_eq!(parse_instruction(1002).0, Opcode::Mul);
    assert_eq!(parse_instruction(1002).1, ParameterMode::Position);
    assert_eq!(parse_instruction(1002).2, ParameterMode::Immediate);
    assert_eq!(parse_instruction(1002).3, ParameterMode::Position);
}
