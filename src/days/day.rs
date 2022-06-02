pub trait Day {
    fn parse(input_str: &str) -> Box<dyn Day> where Self: Sized;
    fn part1(&self) -> String;
    fn part2(&self) -> String;
}