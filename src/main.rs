#![allow(dead_code)]

mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;

use crate::day7::day7::do_it;
use std::fs::read_to_string;

fn main() {
    do_it()
}

fn input_to_vec(day: u8) -> Vec<String> {
    read_to_string(format!("inputs/day{day}.txt"))
        .unwrap()
        .lines()
        .map(String::from)
        .collect()
}
