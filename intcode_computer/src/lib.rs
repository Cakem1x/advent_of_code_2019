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
    fn step(&mut self, input: Option<i32>) -> Option<i32> {
        let mut output = None;
        match parse_instruction(self.memory[self.instruction_pointer]) {
            (Opcode::Add, pm1, pm2, pm3) => {
                let first_operand = self.resolve_parameter_to_value(1, pm1);
                let second_operand = self.resolve_parameter_to_value(2, pm2);
                let result_address = self.resolve_parameter_to_result_address(3, pm3);
                let result_value = first_operand + second_operand;
                self.memory[result_address] = result_value;
                self.instruction_pointer += 4;
            }
            (Opcode::Mul, pm1, pm2, pm3) => {
                let first_operand = self.resolve_parameter_to_value(1, pm1);
                let second_operand = self.resolve_parameter_to_value(2, pm2);
                let result_address = self.resolve_parameter_to_result_address(3, pm3);
                let result_value = first_operand * second_operand;
                self.memory[result_address] = result_value;
                self.instruction_pointer += 4;
                return None;
            }
            (Opcode::Input, pm1, _pm2, _pm3) => {
                if input.is_some() {
                    //println!("#{}: got value {} during input instruction", self.instruction_pointer, input.unwrap());
                    let target_address = self.resolve_parameter_to_result_address(1, pm1);
                    self.memory[target_address] = input.unwrap();
                    self.instruction_pointer += 2;
                } else {
                    panic!("Encountered input instruction without having any next given input.");
                }
            }
            (Opcode::Output, pm1, _pm2, _pm3) => {
                output = Some(self.resolve_parameter_to_value(1, pm1));
                self.instruction_pointer += 2;
            }
            (Opcode::JumpIfTrue, pm1, pm2, _pm3) => {
                let condition = self.resolve_parameter_to_value(1, pm1);
                let jump_target = self.resolve_parameter_to_jump_address(2, pm2);
                if condition != 0 {
                    self.instruction_pointer = jump_target;
                } else {
                    self.instruction_pointer += 3;
                }
            }
            (Opcode::JumpIfFalse, pm1, pm2, _pm3) => {
                let condition = self.resolve_parameter_to_value(1, pm1);
                let jump_target = self.resolve_parameter_to_jump_address(2, pm2);
                if condition == 0 {
                    self.instruction_pointer = jump_target;
                } else {
                    self.instruction_pointer += 3;
                }
            }
            (Opcode::LessThan, pm1, pm2, pm3) => {
                let first_operand = self.resolve_parameter_to_value(1, pm1);
                let second_operand = self.resolve_parameter_to_value(2, pm2);
                let result_ptr = self.resolve_parameter_to_result_address(3, pm3);
                if first_operand < second_operand {
                    self.memory[result_ptr] = 1;
                } else {
                    self.memory[result_ptr] = 0;
                }
                self.instruction_pointer += 4;
            }
            (Opcode::Equals, pm1, pm2, pm3) => {
                let first_operand = self.resolve_parameter_to_value(1, pm1);
                let second_operand = self.resolve_parameter_to_value(2, pm2);
                let result_address = self.resolve_parameter_to_result_address(3, pm3);
                if first_operand == second_operand {
                    self.memory[result_address] = 1;
                } else {
                    self.memory[result_address] = 0;
                }
                self.instruction_pointer += 4;
            }
            (Opcode::RelativeBaseOffset, pm1, _pm2, _pm3) => {
                let offset = self.resolve_parameter_to_value(1, pm1);
                self.relative_base = usize::try_from(self.relative_base as i32 + offset).expect("RelativeBaseOffset reduced program's relative base below zero.");
                self.instruction_pointer += 2;
            }
            (Opcode::Terminate, _pm1, _pm2, _pm3) => ()
        }
        return output;
    }

    fn next_opcode(&self) -> Opcode {
        return parse_instruction(self.memory[self.instruction_pointer]).0;
    }

    /// Resolves a parameter into the value it describes, depending on its parameter mode.
    fn resolve_parameter_to_value(&self, parameter_id: usize, parameter_mode: ParameterMode) -> i32 {
        match parameter_mode {
            ParameterMode::Immediate => return self.memory[self.instruction_pointer + parameter_id],
            ParameterMode::Position => {
                let address = usize::try_from(self.memory[self.instruction_pointer + parameter_id]).expect("Parameter in position mode tried to access a negative address.");
                return self.memory[address];
            }
        }
    }

    fn resolve_parameter_to_jump_address(&self, parameter_id: usize, parameter_mode: ParameterMode) -> usize {
        return usize::try_from(self.resolve_parameter_to_value(parameter_id, parameter_mode)).expect("invalid address as target of a jump instruction.");
    }

    /// Resolves a parameter into the address it describes, depending on its parameter mode.
    fn resolve_parameter_to_result_address(&self, parameter_id: usize, parameter_mode: ParameterMode) -> usize {
        assert_eq!(parameter_mode, ParameterMode::Position); // addresses only support position mode
        return usize::try_from(self.memory[self.instruction_pointer + parameter_id]).expect("A parameter that is interpreted as an address is negative.");
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

    #[test]
    fn test_program_relative_base_duplicates_itself() {
        let code = [109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99];
        let mut program = Program::init(&code);
        let output = program.run(Vec::new());
        assert_eq!(output, code);
    }
}
