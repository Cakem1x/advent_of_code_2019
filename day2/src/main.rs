use permutohedron::Heap;
use std::fs::read_to_string;
use std::io;

#[allow(dead_code)]
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
            run_program(&mut altered_state, Vec::new());
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

#[allow(dead_code)]
fn day5() {
    println!("loading initial state:");
    let mut input_state: Vec<i32> =
        string_to_program_state(&read_to_string("input_day5.txt").unwrap());
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let input_value: i32 = input[..input.len() - 1]
        .parse()
        .expect("Couldn't convert input to i32");
    println!("{:?}", input_state);
    run_program(&mut input_state, [input_value].to_vec());
}

#[allow(dead_code)]
fn day7() {
    println!("loading initial state:");
    let input_state: Vec<i32> = string_to_program_state(&read_to_string("input_day7.txt").unwrap());
    println!("{:?}", input_state);
    let mut max_thruster_value = 0;
    let mut phase_settings = [0, 1, 2, 3, 4];
    for phase_settings_permutation in Heap::new(&mut phase_settings).by_ref() {
        let thruster_value = amplification_circuit(&input_state, phase_settings_permutation);
        if thruster_value > max_thruster_value {
            max_thruster_value = thruster_value;
            println!(
                "Found new max_thruster_value: {}, with sequence {:?}",
                max_thruster_value, phase_settings_permutation
            );
        }
    }
    println! {"max thruster value: {}", max_thruster_value};
}

#[allow(dead_code)]
fn day7_part2() {
    println!("loading initial state:");
    let input_state: Vec<i32> = string_to_program_state(&read_to_string("input_day7.txt").unwrap());
    println!("{:?}", input_state);
    let mut max_thruster_value = 0;
    let mut phase_settings = [5, 6, 7, 8, 9];
    for phase_settings_permutation in Heap::new(&mut phase_settings).by_ref() {
        let thruster_value =
            amplification_circuit_with_feedback(&input_state, phase_settings_permutation);
        if thruster_value > max_thruster_value {
            max_thruster_value = thruster_value;
            println!(
                "Found new max_thruster_value: {}, with sequence {:?}",
                max_thruster_value, phase_settings_permutation
            );
        }
    }
    println! {"max thruster value: {}", max_thruster_value};
}

fn string_to_program_state(input_string: &str) -> Vec<i32> {
    return input_string[..input_string.len() - 1] // get rid of \n character
        .split(",")
        .map(|s| s.parse::<i32>().expect("failed to convert input to i32"))
        .collect();
}

fn main() {
    //day2();
    //day5();
    //day7();
    day7_part2();
}

/// For a given program and a set of phase settings, calculate the resulting thruster value
fn amplification_circuit_with_feedback(program_state: &[i32], phase_settings: [i32; 5]) -> i32 {
    let mut amplifier_state = [
        (program_state.to_vec(), 0, 0),
        (program_state.to_vec(), 0, 0),
        (program_state.to_vec(), 0, 0),
        (program_state.to_vec(), 0, 0),
        (program_state.to_vec(), 0, 0),
    ];
    // set amplifiers' phases
    for (amplifier_id, phase) in phase_settings.iter().enumerate() {
        println!("Setting phase of amplifier {} to {}", amplifier_id, phase);
        amplifier_state[amplifier_id].1 = set_phase(amplifier_state[amplifier_id].1, &mut amplifier_state[amplifier_id].0, *phase);
    }
    // run until amplifiers terminate
    let mut not_terminated = true;
    let mut previous_output = 0;
    let mut feedback_loop_counter = 0;
    while not_terminated {
        for amplifier_id in 0..amplifier_state.len() {
            let result = run_amplification_program(
                amplifier_state[amplifier_id].1,
                &mut amplifier_state[amplifier_id].0,
                previous_output,
            );
            if amplifier_state[amplifier_id].1 == result.0 {
                not_terminated = false;
                println!(
                    "#{} - Amplifier {} terminated.",
                    feedback_loop_counter, amplifier_id
                );
            } else {
                amplifier_state[amplifier_id].1 = result.0;
                amplifier_state[amplifier_id].2 = result.1.unwrap();
                println!(
                    "#{} - Amplifier {}: {} -> {}.",
                    feedback_loop_counter, amplifier_id, previous_output, result.1.unwrap()
                );
                previous_output = result.1.unwrap();
            }
        }
        feedback_loop_counter += 1;
    }
    return amplifier_state[4].2; // thruster value
}

/// For a given program and a set of phase settings, calculate the resulting thruster value
fn amplification_circuit(program_state: &[i32], phase_settings: [i32; 5]) -> i32 {
    let mut previous_output = 0;
    for (_amplifier_id, phase) in phase_settings.iter().enumerate() {
        let input = [previous_output, phase.to_owned()];
        previous_output = run_program(&mut program_state.to_vec(), input.to_vec())[0];
        //println!(
        //    "Amplifier {}: phase setting {} | {} -> {}",
        //    _amplifier_id, input[1], input[0], previous_output
        //);
    }
    return previous_output;
}

/// runs an amplification program using a single input until it yields a single output or until it terminates.
fn run_amplification_program(instruction_pointer: usize, state: &mut [i32], input_value: i32) -> (usize, Option<i32>) {
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

fn run_program(state: &mut [i32], mut input_values: Vec<i32>) -> Vec<i32> {
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

/// runs the program until after an input instruction was executed. Used to set the phase of amplifier programs.
fn set_phase(
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

#[test]
fn program_with_less_than_in_immediate_mode() {
    for input in 0..8 {
        let mut program_state = [1107, input, 8, 1, 4, 1, 99];
        run_program(&mut program_state, Vec::new());
        assert_eq!(program_state[1], 1);
    }
    for input in 8..12 {
        let mut program_state = [1107, input, 8, 1, 4, 1, 99];
        run_program(&mut program_state, Vec::new());
        assert_eq!(program_state[1], 0);
    }
}

#[test]
fn program_with_equals_in_immediate_mode() {
    for input in 0..8 {
        let mut program_state = [1108, input, 8, 1, 4, 1, 99];
        run_program(&mut program_state, Vec::new());
        assert_eq!(program_state[1], 0);
    }
    let input = 8;
    let mut program_state = [1108, input, 8, 1, 4, 1, 99];
    run_program(&mut program_state, Vec::new());
    assert_eq!(program_state[1], 1);
    for input in 9..12 {
        let mut program_state = [1108, input, 8, 1, 4, 1, 99];
        run_program(&mut program_state, Vec::new());
        assert_eq!(program_state[1], 0);
    }
}

#[test]
fn program_with_less_than_in_position_mode() {
    for input in 0..8 {
        let mut program_state = [7, 7, 8, 7, 4, 7, 99, input, 8];
        run_program(&mut program_state, Vec::new());
        assert_eq!(program_state[7], 1);
    }
    for input in 8..12 {
        let mut program_state = [7, 7, 8, 7, 4, 7, 99, input, 8];
        run_program(&mut program_state, Vec::new());
        assert_eq!(program_state[7], 0);
    }
}

#[test]
fn program_with_equals_in_position_mode() {
    for input in 0..8 {
        let mut program_state = [8, 7, 8, 7, 4, 7, 99, input, 8];
        run_program(&mut program_state, Vec::new());
        assert_eq!(program_state[7], 0);
    }
    let input = 8;
    let mut program_state = [8, 7, 8, 7, 4, 7, 99, input, 8];
    run_program(&mut program_state, Vec::new());
    assert_eq!(program_state[7], 1);
    for input in 9..12 {
        let mut program_state = [8, 7, 8, 7, 4, 7, 99, input, 8];
        run_program(&mut program_state, Vec::new());
        assert_eq!(program_state[7], 0);
    }
}

#[test]
fn program_with_jump_in_position_mode() {
    let input = 0;
    let mut program_state = [6, 10, 13, 1, 11, 12, 11, 4, 11, 99, input, 0, 1, 9];
    run_program(&mut program_state, Vec::new());
    assert_eq!(program_state[11], 0);
    let input = 3;
    let mut program_state = [6, 10, 13, 1, 11, 12, 11, 4, 11, 99, input, 0, 1, 9];
    run_program(&mut program_state, Vec::new());
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
    run_program(&mut program_state, Vec::new());
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
    run_program(&mut program_state, Vec::new()); // should output 0, but testing that seems too complicated
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
    run_program(&mut program_state, Vec::new());
    assert_eq!(program_state, [30, 1, 1, 4, 2, 5, 6, 0, 99]);
}

#[test]
fn day2_works() {
    let mut program_state = [
        1, 53, 79, 3, 1, 1, 2, 3, 1, 3, 4, 3, 1, 5, 0, 3, 2, 1, 6, 19, 1, 9, 19, 23, 1, 6, 23, 27,
        1, 10, 27, 31, 1, 5, 31, 35, 2, 6, 35, 39, 1, 5, 39, 43, 1, 5, 43, 47, 2, 47, 6, 51, 1, 51,
        5, 55, 1, 13, 55, 59, 2, 9, 59, 63, 1, 5, 63, 67, 2, 67, 9, 71, 1, 5, 71, 75, 2, 10, 75,
        79, 1, 6, 79, 83, 1, 13, 83, 87, 1, 10, 87, 91, 1, 91, 5, 95, 2, 95, 10, 99, 2, 9, 99, 103,
        1, 103, 6, 107, 1, 107, 10, 111, 2, 111, 10, 115, 1, 115, 6, 119, 2, 119, 9, 123, 1, 123,
        6, 127, 2, 127, 10, 131, 1, 131, 6, 135, 2, 6, 135, 139, 1, 139, 5, 143, 1, 9, 143, 147, 1,
        13, 147, 151, 1, 2, 151, 155, 1, 10, 155, 0, 99, 2, 14, 0, 0,
    ];
    run_program(&mut program_state, Vec::new());
    assert_eq!(program_state[0], 19690720);
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

#[test]
fn get_thruster_signal() {
    let program_state = [
        3, 15, 3, 16, 1002, 16, 10, 16, 1, 16, 15, 15, 4, 15, 99, 0, 0,
    ];
    let sequence = [4, 3, 2, 1, 0];
    let thruster_value = amplification_circuit(&program_state, sequence);
    assert_eq!(thruster_value, 43210);
    let program_state = [
        3, 23, 3, 24, 1002, 24, 10, 24, 1002, 23, -1, 23, 101, 5, 23, 23, 1, 24, 23, 23, 4, 23, 99,
        0, 0,
    ];
    let sequence = [0, 1, 2, 3, 4];
    let thruster_value = amplification_circuit(&program_state, sequence);
    assert_eq!(thruster_value, 54321);
    let program_state = [
        3, 31, 3, 32, 1002, 32, 10, 32, 1001, 31, -2, 31, 1007, 31, 0, 33, 1002, 33, 7, 33, 1, 33,
        31, 31, 1, 32, 31, 31, 4, 31, 99, 0, 0, 0,
    ];
    let sequence = [1, 0, 4, 3, 2];
    let thruster_value = amplification_circuit(&program_state, sequence);
    assert_eq!(thruster_value, 65210);
}
