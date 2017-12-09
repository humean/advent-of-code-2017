extern crate failure;
extern crate itertools;

use failure::Error;

use std::io::{self, Read};
use std::process;
use itertools::Itertools;

fn main() {
    let input = parse_input().unwrap_or_else(|err| {
        eprintln!("Problem parsing input: {}", err);
        process::exit(1);
    });

    part_one(&input);
    part_two(&input);
}

fn part_one(v: &Vec<Vec<u32>>) {
    let checksum: u32 = v.iter()
        .map(|row| {
            let max = match row.iter().max() {
                Some(&num) => num,
                None => 0,
            };

            let min = match row.iter().min() {
                Some(&num) => num,
                None => 0,
            };

            max - min
        })
        .sum();


    println!("Part one solution: {}", checksum);
}

fn part_two(v: &Vec<Vec<u32>>) {
    let sum_of_even_divisons: u32 = v.iter()
        .map(|inner_v| {
            inner_v
                .iter()
                .tuple_combinations()
                .map(|(a, b)| {
                    let mut sum = 0;
                    if a % b == 0 {
                        sum = a / b
                    }

                    if b % a == 0 {
                        sum = b / a + sum
                    }

                    sum
                })
                .sum::<u32>()
        })
        .sum();

    println!("Part two solution: {}", sum_of_even_divisons);
}

fn parse_input() -> Result<Vec<Vec<u32>>, Error> {
    let mut stdin = io::stdin();

    let mut buff = String::new();

    stdin.read_to_string(&mut buff)?;

    let table: Vec<Vec<u32>> = buff.lines()
        .map(|line| {
            line.split_whitespace()
                .filter_map(|elem| elem.parse().ok())
                .collect()
        })
        .collect();

    Ok(table)
}
