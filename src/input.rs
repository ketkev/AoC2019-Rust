pub fn list_of_digits(input: &str) -> Vec<i32> {
    input.split_whitespace().map(|line| line.parse::<i32>().unwrap()).collect()
}
