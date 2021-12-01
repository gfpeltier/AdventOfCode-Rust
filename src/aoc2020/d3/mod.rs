use std::time::Instant;
use crate::util::in_lines;

fn part1(lines: &Vec<String>) -> i32 {
	1
}

fn part2(lines: &Vec<String>) -> i32 {
	1
}

pub fn run() {
	println!("\n2020 Day 3");
	let input = in_lines("input_files/2020/d1.txt");
	let start = Instant::now();
	let a1 = part1(&input);
	let a2 = part2(&input);
	let duration = start.elapsed();
	println!("Part 1: {}", a1);
	println!("Part 2: {}", a2);
	println!("Day 1 Calc time: {:?}", duration);
}