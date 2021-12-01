use std::time::Instant;
use crate::util::in_lines;

fn part1(nums: &Vec<i32>) -> i32 {
	let mut cnt = 0;
	for i in 1..nums.len() {
		if nums[i] > nums[i - 1] {
			cnt += 1;
		}
	}
	cnt
}

fn part2(nums: &Vec<i32>) -> i32 {
	let mut cnt = 0;
	for i in 3..nums.len() {
		if (nums[i] + nums[i - 1] + nums[i - 2]) > (nums[i - 1] + nums[i - 2] + nums[i - 3]) {
			cnt += 1;
		}
	}
	cnt
}

pub fn run() {
	println!("\n 2021 Day 1");
	let input = in_lines("input_files/2021/d1.txt");
	let start = Instant::now();
	let nums: Vec<i32> = input.iter().map(|s| s.parse::<i32>().unwrap()).collect::<Vec<_>>();
	let a1 = part1(&nums);
	let a2 = part2(&nums);
	let duration = start.elapsed();
	println!("Part 1: {}", a1);
	println!("Part 2: {}", a2);
	println!("Day 1 Calc time: {:?}", duration);
}