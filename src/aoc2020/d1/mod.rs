use std::collections::HashSet;
use crate::util::in_lines;

fn part1(lines: &Vec<String>) -> i32 {
    let mut nums = HashSet::from_iter(lines);
}

fn part2(lines: &Vec<String>) -> i32 {
    1
}

pub fn run() {
    let input = in_lines("input_files/2020/d1.txt");
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}