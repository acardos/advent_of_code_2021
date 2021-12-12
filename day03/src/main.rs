use std::{fs};

use bit_vec::BitVec;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let mut original_lines = vec![];
    for line in input.lines() {
        let line_vec = line.chars().into_iter().map(|d| d.to_digit(10).unwrap()).collect();
        original_lines.push(line_vec);
    }

    let lines = original_lines.iter().collect();

    let lines_transposed = transpose(&lines);
    let mut sums: BitVec = lines_transposed.iter().map(|l| {
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

    let oxygen_rating  = find_rating(&lines, false, 1);
    let co2_rating = find_rating(&lines, true, 0);

    let oxygen_rating = to_dec(&oxygen_rating);
    let co2_rating = to_dec(&co2_rating);

    println!("oxygen_rating: {}", oxygen_rating);
    println!("co2_rating: {}", co2_rating);
    println!("life support rating: {}", co2_rating * oxygen_rating);
}


fn find_rating<'a>(lines: &'a Vec<&Vec<u32>>, least_common: bool, when_eq: u32) -> BitVec {
    let mut i = 0;
    let mut lines: Vec<&Vec<u32>> = lines.clone();
    while lines.len() > 1 {
        let transposed = transpose(&lines);
        let bit_criterion  = get_criterion(&transposed[i], least_common, when_eq);
        lines = lines.iter().filter(|l| l[i] == bit_criterion).map(|l| *l).collect();
        i += 1;
    }

    lines[0].iter().map(|l| *l == 1).collect()
}


fn get_criterion(line: &Vec<u32>, least_common: bool, when_eq: u32) -> u32 {
    let sum: u32 = line.iter().sum();
    let sum = sum * 2;
    let n = line.len().try_into().unwrap();

    if sum == n {
        when_eq
    } else if !least_common && sum > n || least_common && sum < n {
        1
    } else {
        0
    }
}

fn transpose<T: Copy>(input: &Vec<&Vec<T>>) -> Vec<Vec<T>> {
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