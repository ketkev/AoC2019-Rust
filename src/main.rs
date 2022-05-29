mod days;
mod input;

use std::time::Instant;
use term_table::{Table, TableStyle};
use term_table::row::Row;
use term_table::table_cell::{Alignment, TableCell};
use days::*;
use crate::day1::Day1;
use crate::day::Day;

fn get_day(day_num: i32) -> impl Day {
    match day_num {
        1 => Day1::parse(include_str!("../data/day1.txt")),
        _ => panic!("Invalid day")
        // TODO: Handle with result
    }
}

fn main() {
    // TODO: Add CLI
    let days = 1..2;
    let mut used_time: u128 = 0;
    let mut table = Table::new();

    prepare_table(&mut table);

    for day_num in days {
        let mut start = Instant::now();

        let day = get_day(day_num);

        let part1 = day.part1();
        let part1_time = start.elapsed().as_millis();

        start = Instant::now();

        let part2 = day.part2();
        let part2_time = start.elapsed().as_millis();

        used_time += part1_time;
        used_time += part2_time;

        add_result(&mut table, day_num, part1, part1_time, part2, part2_time);
    }

    add_timing_summary(&mut table, used_time);

    println!("{}", table.render());
}

fn prepare_table(table: &mut Table) {
    table.style = TableStyle::rounded();

    table.add_row(Row::new(vec![
        TableCell::new_with_alignment("AoC 2019 - Rust", 3, Alignment::Center)
    ]));

    table.add_row(Row::new(vec![
        TableCell::new("[Name]"),
        TableCell::new("[Result]"),
        TableCell::new("[Time]"),
    ]));
}

fn add_timing_summary(table: &mut Table, used_time: u128) {
    let remaining_time = 1_000 - used_time;

    table.add_row(Row::new(vec![
        TableCell::new_with_alignment(format!("Total: {used_time} ms, {remaining_time} ms remaining"), 3, Alignment::Center),
    ]));
}

fn add_result(table: &mut Table, day_num: i32, part1: String, part1_time: u128, part2: String, part2_time: u128) {
    table.add_row(Row::new(vec![
        TableCell::new(format!("Day {day_num} - Part 1")),
        TableCell::new(part1),
        TableCell::new(format!("{part1_time} ms")),
    ]));

    table.add_row(Row::new(vec![
        TableCell::new(format!("Day {day_num} - Part 2")),
        TableCell::new(part2),
        TableCell::new(format!("{part2_time} ms")),
    ]));
}