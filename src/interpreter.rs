use crate::input::list_of_digits;
use std::convert::TryInto;
use rstest::rstest;


enum Opcode {
    Add = 1,
    Multiply = 2,
    Terminate = 99,
}

enum StepResult {
    Continue,
    Terminate,
}

impl TryFrom<usize> for Opcode {
    type Error = ();

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        match value {
            x if x == Opcode::Add as usize => Ok(Opcode::Add),
            x if x == Opcode::Multiply as usize => Ok(Opcode::Multiply),
            x if x == Opcode::Terminate as usize => Ok(Opcode::Terminate),
            _ => Err(())
        }
    }
}

pub struct Interpreter {
    pub memory: Vec<usize>,
    pub position: usize,
}

impl Interpreter {
    pub fn load_program(program: &str) -> Self {
        Interpreter {
            memory: list_of_digits::<usize>(program, ','),
            position: 0,
        }
    }

    pub fn run(&mut self) {
        let mut result = StepResult::Continue;

        while matches!(result, StepResult::Continue) {
            result = self.step();
        }
    }

    fn step(&mut self) -> StepResult {
        match self.memory[self.position].try_into() {
            Ok(Opcode::Add) => self.add(),
            Ok(Opcode::Multiply) => self.multiply(),
            Ok(Opcode::Terminate) => return StepResult::Terminate,
            Err(_) => panic!("Encountered invalid Opcode")
        }

        StepResult::Continue
    }

    fn add(&mut self) {
        let lhs = self.memory[self.memory[self.position + 1]];
        let rhs = self.memory[self.memory[self.position + 2]];
        let output_address = self.memory[self.position + 3];

        self.memory[output_address] = lhs + rhs;
        self.position += 4;
    }

    fn multiply(&mut self) {
        let lhs = self.memory[self.memory[self.position + 1]];
        let rhs = self.memory[self.memory[self.position + 2]];
        let output_address = self.memory[self.position + 3];

        self.memory[output_address] = lhs * rhs;
        self.position += 4;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[rstest]
    #[case("1,9,10,3,2,3,11,0,99,30,40,50", "[3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50]")]
    #[case("1,0,0,0,99", "[2, 0, 0, 0, 99]")]
    #[case("2,3,0,3,99", "[2, 3, 0, 6, 99]")]
    #[case("2,4,4,5,99,0", "[2, 4, 4, 5, 99, 9801]")]
    #[case("1,1,1,4,99,5,6,0,99", "[30, 1, 1, 4, 2, 5, 6, 0, 99]")]
    fn day2_part1_examples_work(#[case] input: &str, #[case] expected: &str) {
        let mut interpreter = Interpreter::load_program(input);

        interpreter.run();

        let memory = &interpreter.memory;
        let actual_result = format!("{memory:?}");

        assert_eq!(expected, actual_result);
    }
}