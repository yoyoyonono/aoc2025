use std::collections::{HashMap, HashSet};

#[derive(Debug)]
enum Operation {
    Add,
    Multiply,
}

#[derive(Debug)]
struct Problem {
    numbers: Vec<i128>,
    operation: Operation,
}

fn main() {
    let binding = std::fs::read_to_string("input.txt").unwrap();
    let input: Vec<Vec<char>> = binding
        .lines()
        .map(|line| line.chars().collect())
        .step_by(2)
        .collect();

    let start_col = input[0].iter().position(|x| *x == 'S').unwrap();
    println!("{start_col}");

    let mut beams: HashMap<usize, i128> = HashMap::new();
    beams.insert(start_col, 1);
    for line in input[1..].iter() {
        println!("{:?}", beams);
        for (col, c) in line.iter().enumerate() {
            if *c == '^' && beams.contains_key(&col) {
                let count = beams.get(&col).unwrap().clone();
                let count_before = beams.get(&(col - 1)).unwrap_or(&0).clone();
                let count_after = beams.get(&(col + 1)).unwrap_or(&0).clone();
                beams.insert(col - 1, count_before + count);
                beams.insert(col + 1, count_after + count);
                beams.remove(&col);
            }
        }
    }
    let total: i128 = beams.values().sum();
    println!("{:?}", total);
}
