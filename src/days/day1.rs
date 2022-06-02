use crate::days::day::Day;
use crate::input;

pub struct Day1 {
    modules: Vec<i32>,
}

impl Day1 {
    fn calculate_fuel(mass: &i32) -> i32 {
        (mass / 3) - 2
    }

    fn calculate_fuel_recursively(mass: &i32) -> i32 {
        let required_fuel = (mass / 3) - 2;

        if Day1::calculate_fuel(&required_fuel) > 0 {
            Day1::calculate_fuel_recursively(&required_fuel) + required_fuel
        } else {
            required_fuel
        }
    }
}

impl Day for Day1 {
    fn parse(input_str: &str) -> Box<dyn Day> {
        let modules = input::list_of_digits::<i32>(input_str, '\n');

        Box::new(
            Day1 { modules }
        )
    }

    fn part1(&self) -> String {
        let result: i32 = self.modules.iter().map(Day1::calculate_fuel).sum();
        result.to_string()
    }

    fn part2(&self) -> String {
        let result: i32 = self.modules.iter().map(Day1::calculate_fuel_recursively).sum();
        result.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calculate_fuel_test() {
        assert_eq!(Day1::calculate_fuel(&12), 2);
        assert_eq!(Day1::calculate_fuel(&14), 2);
        assert_eq!(Day1::calculate_fuel(&1969), 654);
        assert_eq!(Day1::calculate_fuel(&100756), 33583);
    }

    #[test]
    fn calculate_fuel_recursively_test() {
        assert_eq!(Day1::calculate_fuel_recursively(&14), 2);
        assert_eq!(Day1::calculate_fuel_recursively(&1969), 966);
        assert_eq!(Day1::calculate_fuel_recursively(&100756), 50346);
    }
}