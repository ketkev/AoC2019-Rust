use num::range;
use crate::days::day::Day;


pub struct Day2 {
    input: String,
}

impl Day for Day2 {
    fn parse(input_str: &str) -> Box<dyn Day> {
        let input = input_str.to_string();

        Box::new(Day2 { input })
    }

    fn part1(&self) -> String {
        let mut interpreter = crate::interpreter::Interpreter::load_program(self.input.as_str());

        interpreter.memory[1] = 12;
        interpreter.memory[2] = 2;


        interpreter.run();

        let result = interpreter.memory[0];
        result.to_string()
    }

    fn part2(&self) -> String {
        for noun in 0..99 {
            for verb in 0..99 {
                let mut interpreter = crate::interpreter::Interpreter::load_program(self.input.as_str());

                interpreter.memory[1] = noun;
                interpreter.memory[2] = verb;

                interpreter.run();

                let result = interpreter.memory[0];

                if result == 19690720 {
                    return (100 * noun + verb).to_string();
                }
            }
        }

        String::from("Unable to find solution")
    }
}