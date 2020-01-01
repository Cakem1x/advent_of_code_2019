use intcode_computer;
use std::fs::read_to_string;

fn main() {
    let code = intcode_computer::parse_program_str(&read_to_string("input.txt").unwrap());
    let mut basic_program = intcode_computer::Program::init(&code);
    let output = basic_program.run(vec!(2));
    println!("{:?}", output);
}

#[test]
fn day9_part1_works() {
    let code = intcode_computer::parse_program_str(&read_to_string("input.txt").unwrap());
    let mut basic_program = intcode_computer::Program::init(&code);
    let output = basic_program.run(vec!(1));
    assert_eq!(output, [3598076521]);
}

#[test]
fn day9_part2_works() {
    let code = intcode_computer::parse_program_str(&read_to_string("input.txt").unwrap());
    let mut basic_program = intcode_computer::Program::init(&code);
    let output = basic_program.run(vec!(2));
    assert_eq!(output, [90722]);
}
