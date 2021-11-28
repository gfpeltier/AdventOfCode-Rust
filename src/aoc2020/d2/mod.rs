use std::time::Instant;
use crate::util::in_lines;

struct PassPolicy {
    min: usize,
    max: usize,
    ch: char,
    password: String
}

impl PassPolicy {
    // example: "4-5 r: rrrjr"
    pub fn from_str(s: &String) -> PassPolicy {
        let tokens: Vec<&str> = s.split(' ').collect();
        let min_max: Vec<&str> = tokens[0].split('-').collect();
        PassPolicy {
            min: min_max[0].parse().expect("Invalid min num"),
            max: min_max[1].parse().expect("Invalid max num"),
            ch: tokens[1].chars().nth(0).unwrap(),
            password: String::from(tokens[2])
        }
    }

    fn is_valid1(&self) -> bool {
        let range = self.min..=self.max;
        let c_cnt = self.password.matches(self.ch).count();
        return range.contains(&c_cnt);
    }

    fn is_valid2(&self) -> bool {
        let or_one = self.password.chars().nth(self.min - 1) == Some(self.ch) 
            || self.password.chars().nth(self.max - 1) == Some(self.ch);
        let nand_both = !(self.password.chars().nth(self.min - 1) == Some(self.ch) 
            && self.password.chars().nth(self.max - 1) == Some(self.ch));
        return or_one && nand_both;
    }
}

fn part1(pass_pols: &Vec<PassPolicy>) -> usize {
    return pass_pols.iter().filter(|pp| pp.is_valid1()).count();
}

fn part2(pass_pols: &Vec<PassPolicy>) -> usize {
    return pass_pols.iter().filter(|pp| pp.is_valid2()).count();
}

pub fn run() {
    println!("\nDay 2");
    let input = in_lines("input_files/2020/d2.txt");
    let start = Instant::now();
    let passes: Vec<PassPolicy> = input.iter().map(|l| PassPolicy::from_str(l)).collect();
    let a1 = part1(&passes);
    let a2 = part2(&passes);
    let duration = start.elapsed();
    println!("Part 1: {}", a1);
    println!("Part 2: {}", a2);
    println!("Day 2 Calc time: {:?}", duration);
}