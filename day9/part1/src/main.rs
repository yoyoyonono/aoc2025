use std::{cmp::max, collections::{HashMap, HashSet}};

fn main() {
    let binding = std::fs::read_to_string("input.txt").unwrap();
    let input: Vec<[i128; 2]> = binding
        .lines()
        .map(|line| {
            let split: Vec<i128> = line.split(',').map(|x| x.parse().unwrap()).collect();
            [split[0], split[1]]
        })
        .collect();

    let mut maximum_size = 0;
    for (i, first) in input.iter().enumerate() {
        for second in input[i + 1..].iter() {
            let try_size = ((first[0] - second[0]).abs() + 1) * ((first[1] - second[1]).abs() + 1);
            if try_size > maximum_size {
                maximum_size = try_size;
            }
        }
    }
    
    println!("{maximum_size}");

}
