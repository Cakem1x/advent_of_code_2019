use intcode_computer;
use permutohedron::Heap;
use std::fs::read_to_string;

fn main() {
    //day7();
    day7_part2();
}

#[allow(dead_code)]
fn day7() {
    println!("loading initial state:");
    let code = intcode_computer::parse_program_str(&read_to_string("input.txt").unwrap());
    let mut max_thruster_value = 0;
    let mut phase_settings = [0, 1, 2, 3, 4];
    for phase_settings_permutation in Heap::new(&mut phase_settings).by_ref() {
        let thruster_value = amplification_circuit(&code, phase_settings_permutation);
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

fn day7_part2() {
    println!("loading initial state:");
    let code = intcode_computer::parse_program_str(&read_to_string("input.txt").unwrap());
    let mut max_thruster_value = 0;
    let mut phase_settings = [5, 6, 7, 8, 9];
    for phase_settings_permutation in Heap::new(&mut phase_settings).by_ref() {
        let thruster_value =
            amplification_circuit_with_feedback(&code, phase_settings_permutation);
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

/// For a given program and a set of phase settings, calculate the resulting thruster value
fn amplification_circuit_with_feedback(program_state: &[i64], phase_settings: [i64; 5]) -> i64 {
    let mut amplifier_programs = [
        intcode_computer::Program::init(program_state),
        intcode_computer::Program::init(program_state),
        intcode_computer::Program::init(program_state),
        intcode_computer::Program::init(program_state),
        intcode_computer::Program::init(program_state),
    ];
    // set amplifiers' phases
    for (amplifier_id, phase) in phase_settings.iter().enumerate() {
        println!("Setting phase of amplifier {} to {}", amplifier_id, phase);
        amplifier_programs[amplifier_id].run_until_input(*phase);
    }
    // run until amplifiers terminate
    let mut not_terminated = true;
    let mut previous_output = 0;
    let mut feedback_loop_counter = 0;
    while not_terminated {
        for (amplifier_id, program) in amplifier_programs.iter_mut().enumerate() {
            if program.will_terminate() {
                not_terminated = false;
                println!(
                    "#{} - Amplifier {} terminated.",
                    feedback_loop_counter, amplifier_id
                );
            } else {
                program.run_until_input(previous_output);
                let new_output = program.run_until_output_or_terminate();
                assert_eq!(new_output.is_some(), true);
                println!(
                    "#{} - Amplifier {}: {} -> {}.",
                    feedback_loop_counter,
                    amplifier_id,
                    previous_output,
                    new_output.unwrap(),
                );
                previous_output = new_output.unwrap();
            }
        }
        feedback_loop_counter += 1;
    }
    return previous_output; // thruster value
}

/// For a given program and a set of phase settings, calculate the resulting thruster value
fn amplification_circuit(program_state: &[i64], phase_settings: [i64; 5]) -> i64 {
    let mut previous_output = 0;
    for (_amplifier_id, phase) in phase_settings.iter().enumerate() {
        let mut amplifier_program = intcode_computer::Program::init(program_state);
        let input = [previous_output, phase.to_owned()];
        previous_output = amplifier_program.run(input.to_vec())[0];
        //println!(
        //    "Amplifier {}: phase setting {} | {} -> {}",
        //    _amplifier_id, input[1], input[0], previous_output
        //);
    }
    return previous_output;
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

#[test]
fn day7_part1_works() {
    let code = intcode_computer::parse_program_str(&read_to_string("input.txt").unwrap());
    let mut max_thruster_value = 0;
    let mut phase_settings = [0, 1, 2, 3, 4];
    for phase_settings_permutation in Heap::new(&mut phase_settings).by_ref() {
        let thruster_value = amplification_circuit(&code, phase_settings_permutation);
        if thruster_value > max_thruster_value {
            max_thruster_value = thruster_value;
            println!(
                "Found new max_thruster_value: {}, with sequence {:?}",
                max_thruster_value, phase_settings_permutation
            );
        }
    }
    assert_eq!(max_thruster_value, 118936);
}

#[test]
fn day7_part2_works() {
    let code = intcode_computer::parse_program_str(&read_to_string("input.txt").unwrap());
    let mut max_thruster_value = 0;
    let mut phase_settings = [5, 6, 7, 8, 9];
    for phase_settings_permutation in Heap::new(&mut phase_settings).by_ref() {
        let thruster_value =
            amplification_circuit_with_feedback(&code, phase_settings_permutation);
        if thruster_value > max_thruster_value {
            max_thruster_value = thruster_value;
            println!(
                "Found new max_thruster_value: {}, with sequence {:?}",
                max_thruster_value, phase_settings_permutation
            );
        }
    }
    assert_eq!(max_thruster_value, 57660948);
}
