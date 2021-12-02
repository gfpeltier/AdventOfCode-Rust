use std::time::Instant;
use crate::util::in_lines;

fn check_slope(lines: &Vec<String>, slope: (usize, usize)) -> u64 {
	let mut coords: (usize, usize) = (0, 0);
	let mut trees = 0;
	while coords.1 < lines.len() {
		let x = coords.0 % lines[0].len();
		if lines[coords.1].chars().nth(x).expect("Invalid index") == '#' {
			trees += 1;
		}
		coords = (coords.0 + slope.0, coords.1 + slope.1)
	}
	trees
}

fn part1(lines: &Vec<String>) -> u64 {
	check_slope(lines, (3, 1))
}

fn part2(lines: &Vec<String>) -> u64 {
	let slopes = vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
	slopes
		.into_iter()
		.map(|s| check_slope(lines, s))
		.collect::<Vec<u64>>()
		.iter()
		.product()
}

pub fn run() {
	println!("\n2020 Day 3");
	let input = in_lines("input_files/2020/d3.txt");
	let start = Instant::now();
	let a1 = part1(&input);
	let a2 = part2(&input);
	let duration = start.elapsed();
	println!("Part 1: {}", a1);
	println!("Part 2: {}", a2);
	println!("Calc time: {:?}", duration);
}