use std::collections::HashSet;
use std::time::Instant;
use crate::util::in_lines;

fn part1(num_set: &HashSet<i32>) -> i32 {
    for num in num_set {
        let target = 2020 - num;
        if num_set.contains(&target) {
            return num * target;
        }
    }
    return -1;
}

fn part2(num_set: &HashSet<i32>) -> i32 {
    for n1 in num_set {
        let t1 = 2020 - n1;
        for n2 in num_set {
            let target = t1 - n2;
            if num_set.contains(&target) {
                return n1 * n2 * target;
            }
        }

    }
    return -1;
}

pub fn run() {
    println!("\n2020 Day 1");
    let input = in_lines("input_files/2020/d1.txt");
    let start = Instant::now();
    let nums: Vec<i32> = input.iter().map(|l| l.parse().unwrap()).collect();
    let num_set: HashSet<i32> = HashSet::from_iter(nums);
    let a1 = part1(&num_set);
    let a2 = part2(&num_set);
    let duration = start.elapsed();
    println!("Part 1: {}", a1);
    println!("Part 2: {}", a2);
    println!("Calc time: {:?}", duration);
}