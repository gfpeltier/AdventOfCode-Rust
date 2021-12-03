use std::time::Instant;
use crate::util::in_lines;

fn part1(blines: &Vec<String>) -> i32 {
    let mut col_freqs = vec![(0, 0); blines[0].len()];
    for line in blines {
        for (i, c) in line.chars().enumerate() {
            match c {
                '0' => { col_freqs[i].0 += 1; }
                '1' => { col_freqs[i].1 += 1; }
                _ => panic!()
            }
        }
    }
    let gamma = col_freqs.iter().enumerate().fold(
        0,
        |out, (i, cf)| if cf.0 >= cf.1 {
            out
        } else {
            out | (1 << col_freqs.len() - 1 - i)
        }
    );
    let epsilon = col_freqs.iter().enumerate().fold(
        0,
        |out, (i, cf)| if cf.0 < cf.1 {
            out
        } else {
            out | (1 << col_freqs.len() - 1 - i)
        }
    );
    epsilon * gamma
}

fn filter_max_pos(blines: Vec<&String>, pos: usize) -> Vec<&String> {
    let mut col_freqs = (0, 0);
    for line in &blines {
        match line.chars().nth(pos).unwrap() {
            '0' => { col_freqs.0 += 1; }
            '1' => { col_freqs.1 += 1; }
            _ => panic!()
        }
    }
    blines
        .into_iter()
        .filter(|bl| (bl.chars().nth(pos).unwrap() == '1' && col_freqs.1 >= col_freqs.0) 
            || (bl.chars().nth(pos).unwrap() == '0' && col_freqs.0 > col_freqs.1)
        )
        .collect()
}

fn filter_min_pos(blines: Vec<&String>, pos: usize) -> Vec<&String> {
    let mut col_freqs = (0, 0);
    for line in &blines {
        match line.chars().nth(pos).unwrap() {
            '0' => { col_freqs.0 += 1; }
            '1' => { col_freqs.1 += 1; }
            _ => panic!()
        }
    }
    blines
        .into_iter()
        .filter(|bl| (bl.chars().nth(pos).unwrap() == '0' && col_freqs.0 <= col_freqs.1) 
            || (bl.chars().nth(pos).unwrap() == '1' && col_freqs.1 < col_freqs.0)
        )
        .collect()
}

fn part2(blines: &Vec<String>) -> i32 {
    let mut oxy_lines: Vec<&String> = filter_max_pos(blines.iter().map(|s| s as &String).collect(), 0);
    let mut pos = 1;
    while oxy_lines.len() > 1 {
        oxy_lines = filter_max_pos(oxy_lines, pos);
        pos += 1;
    }

    let mut co2_lines: Vec<&String> = filter_min_pos(blines.iter().map(|s| s as &String).collect(), 0);
    pos = 1;
    while co2_lines.len() > 1 {
        co2_lines = filter_min_pos(co2_lines, pos);
        pos += 1;
    }
    i32::from_str_radix(oxy_lines[0], 2).unwrap() * i32::from_str_radix(co2_lines[0], 2).unwrap()
}

pub fn run() {
    println!("\n2021 Day 3");
    let input = in_lines("input_files/2021/d3.txt");
    let start = Instant::now();
    let a1 = part1(&input);
    let a2 = part2(&input);
    let duration = start.elapsed();
    println!("Part 1: {}", a1);
    println!("Part 2: {}", a2);
    println!("Calc time: {:?}", duration);
}