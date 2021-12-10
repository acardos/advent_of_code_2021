use std::fs;

fn main() {
    let content = fs::read_to_string("input.txt").unwrap();
    let values: Vec<u32> = content.lines().map(|line| line.parse()).filter(|v| v.is_ok()).map(|v| v.unwrap()).collect();

    let sum_part1 = count(&values);
    println!("Sum part 1: {}", sum_part1);

    let windows = compute_windows(&values, 3);
    println!("windows 2: {:?}", windows);

    let sum_part2 = count(&windows);
    println!("Sum part 2: {}", sum_part2);
}

fn count(values: &Vec<u32>) -> u32 {
    let mut previous: Option<u32> = None;
    let mut sum = 0;
    for v in values {
        match previous {
            None => previous = Some(*v),
            Some(x) => {
                previous = Some(*v);
                if *v > x {
                    sum += 1;
                }
            }
        };
    }

    sum
}


fn compute_windows(values: &Vec<u32>, window_size: usize) -> Vec<u32> {
    let mut window_sums: Vec<u32> = vec![];
    for i in window_size..=values.len() {
        let window_sum = values[i - window_size..i].iter().sum();
        window_sums.push(window_sum);
    }
    
    window_sums
}