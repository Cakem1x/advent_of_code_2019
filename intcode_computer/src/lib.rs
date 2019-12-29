use std::convert::TryFrom;

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
    RelativeBaseOffset,
    Terminate,
}

#[derive(Debug, PartialEq)]
enum ParameterMode {
    Position,
    Immediate,
}

pub fn parse_program_str(input_string: &str) -> Vec<i32> {
    return input_string[..input_string.len() - 1] // get rid of \n character
        .split(",")
        .map(|s| s.parse::<i32>().expect("failed to convert input to i32"))
        .collect();
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
        9 => Opcode::RelativeBaseOffset,
        99 => Opcode::Terminate,
        x => panic!("Invalid opcode found: {}!", x),
    };
    let pm_first = match opcode_int / 100 % 10 {
        0 => ParameterMode::Position,
        1 => ParameterMode::Immediate,
        x => panic!("Invalid parameter mode found: {}!", x),
    };
    let pm_second = match opcode_int / 1000 % 10 {
        0 => ParameterMode::Position,
        1 => ParameterMode::Immediate,
        x => panic!("Invalid parameter mode found: {}!", x),
    };
    let pm_third = match opcode_int / 10000 % 10 {
        0 => ParameterMode::Position,
        1 => ParameterMode::Immediate,
        x => panic!("Invalid parameter mode found: {}!", x),
    };
    return (opcode, pm_first, pm_second, pm_third);
}

#[derive(Clone)]
pub struct Program {
    memory: Vec<i32>,
    instruction_pointer: usize,
    relative_base: usize,
}

impl Program {
    pub fn init(code: &[i32]) -> Program {
        return Program {
            memory: code.to_vec(),
            instruction_pointer: 0,
            relative_base: 0,
        };
    }

    pub fn init_from_vec(code: Vec<i32>) -> Program {
        return Program {
            memory: code,
            instruction_pointer: 0,
            relative_base: 0,
        };
    }

    pub fn set_memory(&mut self, at: usize, new_value: i32) {
        self.memory[at] = new_value;
    }

    pub fn read_memory(&self, at: usize) -> i32 {
        return self.memory[at];
    }

    pub fn will_terminate(&self) -> bool {
        return self.next_opcode() == Opcode::Terminate;
    }

    fn next_opcode(&self) -> Opcode {
        return parse_instruction(self.memory[self.instruction_pointer]).0;
    }

    /// Runs the program until it terminates, using a fixed vector of inputs. Returns a vector of output data.
    pub fn run(&mut self, mut input_values: Vec<i32>) -> Vec<i32> {
        let mut program_output = Vec::new();
        loop {
            match self.next_opcode() {
                Opcode::Input => {
                    self.step(input_values.pop());
                }
                Opcode::Output => program_output.push(self.step(None).unwrap()),
                Opcode::Terminate => return program_output,
                _ => {
                    self.step(None);
                }
            }
        }
    }

    /// runs the program input until it yields a single output or until it terminates.
    pub fn run_until_output_or_terminate(&mut self) -> Option<i32> {
        loop {
            match self.next_opcode() {
                Opcode::Output => return self.step(None),
                Opcode::Terminate => return None,
                Opcode::Input => panic!("Encountered Input instruction durign run_until_output_or_terminate!"),
                _ => self.step(None),
            };
        }
    }

    /// Runs the program until after an input instruction was executed and takes exactly one input instruction.
    /// Panics when Termination or Output happens during execution.
    pub fn run_until_input(&mut self, input: i32) {
        loop {
            match self.next_opcode() {
                Opcode::Input => {
                    self.step(Some(input));
                    return;
                }
                Opcode::Output | Opcode::Terminate => panic!("Encountered Output or Terminate instruction durign run_until_input!"),
                _ => self.step(None),
            };
        }
    }

    /// Executes exactly one instruction, may use a provided input if an input instruction is executed. May provide some output if an output instruction is executed.
    pub fn step(&mut self, input: Option<i32>) -> Option<i32> {
        let mut output = None;
        match parse_instruction(self.memory[self.instruction_pointer]) {
            (Opcode::Add, pm1, pm2, pm3) => {
                let first_operand_param = self.memory[self.instruction_pointer + 1];
                let second_operand_param = self.memory[self.instruction_pointer + 2];
                let result_param = self.memory[self.instruction_pointer + 3];
                let first_operand_value = match pm1 {
                    ParameterMode::Position => self.memory[first_operand_param as usize],
                    ParameterMode::Immediate => first_operand_param,
                };
                let second_operand_value = match pm2 {
                    ParameterMode::Position => self.memory[second_operand_param as usize],
                    ParameterMode::Immediate => second_operand_param,
                };
                let result_value = first_operand_value + second_operand_value;
                //println!("{} + {} = {}", first_operand_value, second_operand_value, result_value);
                let result_pointer = match pm3 {
                    ParameterMode::Position => result_param as usize,
                    _ => panic!(
                        "at &{} -> {}: result parameter only supports position mode.",
                        self.instruction_pointer, self.memory[self.instruction_pointer]
                    ),
                };
                self.memory[result_pointer] = result_value;
                self.instruction_pointer += 4;
            }
            (Opcode::Mul, pm1, pm2, pm3) => {
                let first_operand_param = self.memory[self.instruction_pointer + 1];
                let second_operand_param = self.memory[self.instruction_pointer + 2];
                let result_param = self.memory[self.instruction_pointer + 3];
                let first_operand_value = match pm1 {
                    ParameterMode::Position => self.memory[first_operand_param as usize],
                    ParameterMode::Immediate => first_operand_param,
                };
                let second_operand_value = match pm2 {
                    ParameterMode::Position => self.memory[second_operand_param as usize],
                    ParameterMode::Immediate => second_operand_param,
                };
                let result_value = first_operand_value * second_operand_value;
                //println!("{} * {} = {}", first_operand_value, second_operand_value, result_value);
                let result_pointer = match pm3 {
                    ParameterMode::Position => result_param as usize,
                    _ => panic!(
                        "at &{} -> {}: result parameter only supports position mode.",
                        self.instruction_pointer, self.memory[self.instruction_pointer]
                    ),
                };
                self.memory[result_pointer] = result_value;
                self.instruction_pointer += 4;
                return None;
            }
            (Opcode::Input, pm1, _pm2, _pm3) => {
                if input.is_some() {
                    //println!("#{}: got value {} during input instruction", self.instruction_pointer, input.unwrap());
                    let target_param = self.memory[self.instruction_pointer + 1];
                    let target_pointer = match pm1 {
                        ParameterMode::Position => target_param as usize,
                        _ => panic!(
                            "at &{} -> {}: target parameter only supports position mode.",
                            self.instruction_pointer, self.memory[self.instruction_pointer]
                        ),
                    };
                    self.memory[target_pointer] = input.unwrap();
                    self.instruction_pointer += 2;
                } else {
                    panic!("Encountered input instruction without having any next given input.");
                }
            }
            (Opcode::Output, pm1, _pm2, _pm3) => {
                let output_param = self.memory[self.instruction_pointer + 1];
                let output_value = match pm1 {
                    ParameterMode::Position => self.memory[output_param as usize],
                    ParameterMode::Immediate => output_param,
                };
                self.instruction_pointer += 2;
                output = Some(output_value);
            }
            (Opcode::JumpIfTrue, pm1, pm2, _pm3) => {
                let condition_param = self.memory[self.instruction_pointer + 1];
                let condition_value = match pm1 {
                    ParameterMode::Position => self.memory[condition_param as usize],
                    ParameterMode::Immediate => condition_param,
                };
                let jump_target_param = self.memory[self.instruction_pointer + 2];
                let jump_target_value = match pm2 {
                    ParameterMode::Position => self.memory[jump_target_param as usize] as usize,
                    ParameterMode::Immediate => jump_target_param as usize,
                };
                if condition_value != 0 {
                    self.instruction_pointer = jump_target_value;
                } else {
                    self.instruction_pointer += 3;
                }
            }
            (Opcode::JumpIfFalse, pm1, pm2, _pm3) => {
                let condition_param = self.memory[self.instruction_pointer + 1];
                let condition_value = match pm1 {
                    ParameterMode::Position => self.memory[condition_param as usize],
                    ParameterMode::Immediate => condition_param,
                };
                let jump_target_param = self.memory[self.instruction_pointer + 2];
                let jump_target_value = match pm2 {
                    ParameterMode::Position => self.memory[jump_target_param as usize] as usize,
                    ParameterMode::Immediate => jump_target_param as usize,
                };
                if condition_value == 0 {
                    self.instruction_pointer = jump_target_value;
                } else {
                    self.instruction_pointer += 3;
                }
            }
            (Opcode::LessThan, pm1, pm2, pm3) => {
                let first_operand_param = self.memory[self.instruction_pointer + 1];
                let first_operand_value = match pm1 {
                    ParameterMode::Position => self.memory[first_operand_param as usize],
                    ParameterMode::Immediate => first_operand_param,
                };
                let second_operand_param = self.memory[self.instruction_pointer + 2];
                let second_operand_value = match pm2 {
                    ParameterMode::Position => self.memory[second_operand_param as usize],
                    ParameterMode::Immediate => second_operand_param,
                };
                let result_param = self.memory[self.instruction_pointer + 3];
                let result_ptr = match pm3 {
                    ParameterMode::Position => result_param as usize,
                    _ => panic!(
                        "at &{} -> {}: result parameter only supports position mode.",
                        self.instruction_pointer, self.memory[self.instruction_pointer]
                    ),
                };
                if first_operand_value < second_operand_value {
                    self.memory[result_ptr] = 1;
                } else {
                    self.memory[result_ptr] = 0;
                }
                self.instruction_pointer += 4;
            }
            (Opcode::Equals, pm1, pm2, pm3) => {
                let first_operand_param = self.memory[self.instruction_pointer + 1];
                let first_operand_value = match pm1 {
                    ParameterMode::Position => self.memory[first_operand_param as usize],
                    ParameterMode::Immediate => first_operand_param,
                };
                let second_operand_param = self.memory[self.instruction_pointer + 2];
                let second_operand_value = match pm2 {
                    ParameterMode::Position => self.memory[second_operand_param as usize],
                    ParameterMode::Immediate => second_operand_param,
                };
                let result_param = self.memory[self.instruction_pointer + 3];
                let result_ptr = match pm3 {
                    ParameterMode::Position => result_param as usize,
                    _ => panic!(
                        "at &{} -> {}: result parameter only supports position mode.",
                        self.instruction_pointer, self.memory[self.instruction_pointer]
                    ),
                };
                if first_operand_value == second_operand_value {
                    self.memory[result_ptr] = 1;
                } else {
                    self.memory[result_ptr] = 0;
                }
                self.instruction_pointer += 4;
            }
            (Opcode::RelativeBaseOffset, pm1, _pm2, _pm3) => {
                let offset_param = self.memory[self.instruction_pointer + 1];
                let offset_value = match pm1 {
                    ParameterMode::Position => self.memory[offset_param as usize],
                    ParameterMode::Immediate => offset_param,
                };
                self.relative_base = usize::try_from(self.relative_base as i32 + offset_value).expect("RelativeBaseOffset reduced program's relative base below zero.");
                self.instruction_pointer += 2;
            }
            (Opcode::Terminate, _pm1, _pm2, _pm3) => ()
        }
        return output;
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn program_with_less_than_in_immediate_mode() {
        for input in 0..8 {
            let mut program = Program::init(&[1107, input, 8, 1, 4, 1, 99]);
            program.run(Vec::new());
            assert_eq!(program.memory[1], 1);
        }
        for input in 8..12 {
            let mut program = Program::init(&[1107, input, 8, 1, 4, 1, 99]);
            program.run(Vec::new());
            assert_eq!(program.memory[1], 0);
        }
    }

    #[test]
    fn program_with_equals_in_immediate_mode() {
        for input in 0..8 {
            let mut program = Program::init(&[1108, input, 8, 1, 4, 1, 99]);
            program.run(Vec::new());
            assert_eq!(program.memory[1], 0);
        }
        let input = 8;
        let mut program = Program::init(&[1108, input, 8, 1, 4, 1, 99]);
        program.run(Vec::new());
        assert_eq!(program.memory[1], 1);
        for input in 9..12 {
            let mut program = Program::init(&[1108, input, 8, 1, 4, 1, 99]);
            program.run(Vec::new());
            assert_eq!(program.memory[1], 0);
        }
    }

    #[test]
    fn program_with_less_than_in_position_mode() {
        for input in 0..8 {
            let mut program = Program::init(&[7, 7, 8, 7, 4, 7, 99, input, 8]);
            program.run(Vec::new());
            assert_eq!(program.memory[7], 1);
        }
        for input in 8..12 {
            let mut program = Program::init(&[7, 7, 8, 7, 4, 7, 99, input, 8]);
            program.run(Vec::new());
            assert_eq!(program.memory[7], 0);
        }
    }

    #[test]
    fn program_with_equals_in_position_mode() {
        for input in 0..8 {
            let mut program = Program::init(&[8, 7, 8, 7, 4, 7, 99, input, 8]);
            program.run(Vec::new());
            assert_eq!(program.memory[7], 0);
        }
        let input = 8;
        let mut program = Program::init(&[8, 7, 8, 7, 4, 7, 99, input, 8]);
        program.run(Vec::new());
        assert_eq!(program.memory[7], 1);
        for input in 9..12 {
            let mut program = Program::init(&[8, 7, 8, 7, 4, 7, 99, input, 8]);
            program.run(Vec::new());
            assert_eq!(program.memory[7], 0);
        }
    }

    #[test]
    fn program_with_jump_in_position_mode() {
        let input = 0;
        let mut program = Program::init(&[6, 10, 13, 1, 11, 12, 11, 4, 11, 99, input, 0, 1, 9]);
        program.run(Vec::new());
        assert_eq!(program.memory[11], 0);
        let input = 3;
        let mut program = Program::init(&[6, 10, 13, 1, 11, 12, 11, 4, 11, 99, input, 0, 1, 9]);
        program.run(Vec::new());
        assert_eq!(program.memory[11], 1);
    }

    #[test]
    fn program_with_negative_immediate_values() {
        let mut program = Program::init(&[1101, 100, -1, 4, 0]);
        let instruction = parse_instruction(program.memory[0]);
        assert_eq!(instruction.0, Opcode::Add);
        assert_eq!(instruction.1, ParameterMode::Immediate);
        assert_eq!(instruction.2, ParameterMode::Immediate);
        assert_eq!(instruction.3, ParameterMode::Position);
        program.run(Vec::new());
        assert_eq!(program.memory, [1101, 100, -1, 4, 99]);
    }

    #[test]
    fn opcode_add() {
        let mut program = Program::init(&[1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50]);
        program.step(None);
        assert_eq!(program.instruction_pointer, 4);
        assert_eq!(program.memory, [1, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50]);
        let mut program = Program::init(&[1, 0, 0, 0, 99]);
        program.step(None);
        assert_eq!(program.instruction_pointer, 4);
        assert_eq!(program.memory, [2, 0, 0, 0, 99]);
    }

    #[test]
    fn opcode_output_supports_immediate_mode() {
        let mut program = Program::init(&[104, 0, 99]);
        program.run(Vec::new()); // should output 0, but testing that seems too complicated
    }

    #[test]
    fn opcode_mul() {
        let mut program = Program::init(&[1, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50]);
        program.instruction_pointer = 4;
        program.step(None);
        assert_eq!(program.instruction_pointer, 8);
        assert_eq!(
            program.memory,
            [3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50]
        );
        let mut program = Program::init(&[2, 3, 0, 3, 99]);
        program.step(None);
        assert_eq!(program.instruction_pointer, 4);
        assert_eq!(program.memory, [2, 3, 0, 6, 99]);
        let mut program = Program::init(&[2, 4, 4, 5, 99, 0]);
        program.step(None);
        assert_eq!(program.instruction_pointer, 4);
        assert_eq!(program.memory, [2, 4, 4, 5, 99, 9801]);
    }

    #[test]
    fn relative_base_starts_at_zero() {
        let program = Program::init(&[109, -200]);
        assert_eq!(program.relative_base, 0);
    }

    #[test]
    fn opcode_relative_base_increase() {
        let mut program = Program::init(&[109, 19]);
        program.relative_base = 2000;
        program.step(None);
        assert_eq!(program.instruction_pointer, 2);
        assert_eq!(program.relative_base, 2019);
    }

    #[test]
    fn opcode_relative_base_decrease() {
        let mut program = Program::init(&[109, -200]);
        program.relative_base = 2000;
        program.step(None);
        assert_eq!(program.instruction_pointer, 2);
        assert_eq!(program.relative_base, 1800);
    }

    #[test]
    #[should_panic]
    fn opcode_relative_base_panics() {
        let mut program = Program::init(&[109, -200]);
        program.relative_base = 199;
        program.step(None);
    }

    #[test]
    fn mini_program() {
        let mut program = Program::init(&[1, 1, 1, 4, 99, 5, 6, 0, 99]);
        program.run(Vec::new());
        assert_eq!(program.memory, [30, 1, 1, 4, 2, 5, 6, 0, 99]);
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
}
