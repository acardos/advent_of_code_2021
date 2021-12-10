use std::{fs, collections::HashMap};

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    
    let axes = HashMap::from([
        ("forward", ("x", 1)),
        ("up", ("y", -1)),
        ("down", ("y", 1)),
    ]);

    let data: Vec<(&&str, i32)> = input.lines()
            .map(|l| l.split(" ").collect())
            .filter(|l: &Vec<&str>| l.len() == 2)
            .map(|l| {
                // println!("{:?}", l);
                let (axis, factor) = axes.get(l[0])?;
                let value: i32 = l[1].parse().unwrap_or(0);
                Some((axis, value * factor))
            }).filter(|r| r.is_some()).map(|o| o.unwrap()).collect();
    
    let mut x = 0;
    let mut y = 0;
    let mut aim = 0;
    for (axis, value) in &data {
        if **axis == "x" {
            x += value;
            y += aim * value;
        } else {
            aim += value;
        }

        println!("aim: {}", aim);
    }

    println!("({}, {})", x, y);
    println!("result: {}", x * y);
}
