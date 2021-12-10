use std::{fs};

use bit_vec::BitVec;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    let mut lines = vec![];
    for line in input.lines() {
        let line_vec = line.chars().into_iter().map(|d| d.to_digit(10).unwrap()).collect();
        lines.push(line_vec);
    }

    lines = transpose(lines);


    let mut sums: BitVec = lines.iter().map(|l| {
        let sum: u32 = l.iter().sum();
        sum * 2 > l.len().try_into().unwrap()
    }).collect(); 
    println!("bit_vec: {:?}", sums);

    let to_dec = |bit_vec: &BitVec| bit_vec.iter().fold(0, |acc, b| acc*2 + b as u32);

    let gamma_rate = to_dec(&sums);

    sums.negate();
    let epsilon_rate = to_dec(&sums);

    println!("gamma_rate: {}", gamma_rate);
    println!("epsilon_rate: {}", epsilon_rate);
    println!("power consumption: {}", gamma_rate * epsilon_rate);
}

fn transpose<T: Copy>(input: Vec<Vec<T>>) -> Vec<Vec<T>> {
    let mut transposed = vec![];

    for i in 0..input[0].len() {
        let mut row = vec![];
        for j in 0..input.len() {
            row.push(input[j][i]);
        }
        transposed.push(row);
    }

    transposed
}