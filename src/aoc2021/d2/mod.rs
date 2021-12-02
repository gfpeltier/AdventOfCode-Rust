use std::time::Instant;
use crate::util::in_lines;

fn part1(instr_strs: &Vec<String>) -> i32 {
    let mut depth: i32 = 0;
    let mut dist: i32 = 0;
    for instr_str in instr_strs {
        let parts = instr_str.split(' ').collect::<Vec<&str>>();
        match parts[0] {
            "forward" => { dist += parts[1].parse::<i32>().expect("Invalid number"); }
            "up" => { depth -= parts[1].parse::<i32>().expect("Invalid number"); }
            "down" => { depth += parts[1].parse::<i32>().expect("Invalid number"); }
            _ => { panic!(); }
        }
    }
	depth * dist
}

fn part2(instr_strs: &Vec<String>) -> i32 {
	let mut depth: i32 = 0;
    let mut dist: i32 = 0;
    let mut aim: i32 = 0;
    for instr_str in instr_strs {
        let parts = instr_str.split(' ').collect::<Vec<&str>>();
        match parts[0] {
            "forward" => {
                let delta = parts[1].parse::<i32>().expect("Invalid number");
                dist += delta;
                depth += delta * aim;
            }
            "up" => { aim -= parts[1].parse::<i32>().expect("Invalid number"); }
            "down" => { aim += parts[1].parse::<i32>().expect("Invalid number"); }
            _ => { panic!(); }
        }
    }
	depth * dist
}

pub fn run() {
	println!("\n2021 Day 2");
	let input = in_lines("input_files/2021/d2.txt");
	let start = Instant::now();
	let a1 = part1(&input);
	let a2 = part2(&input);
	let duration = start.elapsed();
	println!("Part 1: {}", a1);
	println!("Part 2: {}", a2);
	println!("Calc time: {:?}", duration);
}