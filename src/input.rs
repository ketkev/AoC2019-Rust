use std::fmt::Debug;
use std::str::FromStr;

pub fn list_of_digits<T: num::Num + FromStr>(input: &str, delimiter: char) -> Vec<T> where <T as FromStr>::Err: Debug {
    input.trim().split(delimiter).map(|line| line.parse::<T>().unwrap()).collect()
}
