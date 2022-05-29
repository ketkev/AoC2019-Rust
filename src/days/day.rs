pub trait Day {
    fn parse(input_str: &str) -> Self;
    fn part1(&self) -> String;
    fn part2(&self) -> String;
}