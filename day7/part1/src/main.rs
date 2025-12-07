use std::collections::HashSet;

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

    let mut split_count = 0;
    let mut beams: HashSet<usize> = HashSet::new();
    beams.insert(start_col);
    for line in input[1..].iter() {
        for (col, c) in line.iter().enumerate() {
            if *c == '^' && beams.contains(&col) {
                beams.remove(&col);
                beams.insert(col + 1);
                beams.insert(col - 1);
                split_count += 1;
            }
        }
    }
    println!("{}", split_count);
}
