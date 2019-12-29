use std::io;
use std::fs::read_to_string;
use intcode_computer;

fn main() {
    println!("loading initial state:");
    let code = intcode_computer::parse_program_str(&read_to_string("input.txt").unwrap());
    let mut program = intcode_computer::Program::init_from_vec(code);
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let input_value: i32 = input[..input.len() - 1]
        .parse()
        .expect("Couldn't convert input to i32");
    let output = program.run([input_value].to_vec());
    println!("Output: {:?}", output);
}

#[test]
fn day5_part1_works() {
    println!("loading initial state:");
    let code = intcode_computer::parse_program_str(&read_to_string("input.txt").unwrap());
    let mut program = intcode_computer::Program::init_from_vec(code);
    let output = program.run([1].to_vec());
    assert_eq!(*output.last().unwrap(), 13787043);
}

#[test]
fn day5_part2_works() {
    println!("loading initial state:");
    let code = intcode_computer::parse_program_str(&read_to_string("input.txt").unwrap());
    let mut program = intcode_computer::Program::init_from_vec(code);
    let output = program.run([5].to_vec());
    assert_eq!(*output.last().unwrap(), 3892695);
}
