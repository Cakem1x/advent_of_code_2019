use permutohedron::Heap;
use std::fs::read_to_string;
use std::io;
use intcode_computer;

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
            intcode_computer::run(&mut altered_state, Vec::new());
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
    intcode_computer::run(&mut input_state, [input_value].to_vec());
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
        amplifier_state[amplifier_id].1 = intcode_computer::run_until_after_input_instruction(amplifier_state[amplifier_id].1, &mut amplifier_state[amplifier_id].0, *phase);
    }
    // run until amplifiers terminate
    let mut not_terminated = true;
    let mut previous_output = 0;
    let mut feedback_loop_counter = 0;
    while not_terminated {
        for amplifier_id in 0..amplifier_state.len() {
            let result = intcode_computer::run_until_output_or_termination(
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
        previous_output = intcode_computer::run(&mut program_state.to_vec(), input.to_vec())[0];
        //println!(
        //    "Amplifier {}: phase setting {} | {} -> {}",
        //    _amplifier_id, input[1], input[0], previous_output
        //);
    }
    return previous_output;
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
    intcode_computer::run(&mut program_state, Vec::new());
    assert_eq!(program_state[0], 19690720);
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
