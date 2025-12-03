use std::thread::current;

use itertools::Itertools;

fn main() {
    let mut input: Vec<Vec<u128>> = std::fs::read_to_string("input.txt")
        .unwrap()
        .lines()
        .map(|x| x.chars().map(|z| z.to_digit(10).unwrap().into()).collect())
        .collect();

    for line in input.iter_mut() {
        println!("{:?}", line);
        let counts: Vec<usize> = (0..10)
            .map(|i| line.iter().filter(|x| **x == i).count())
            .collect();
        println!("{:?}", counts);
        for number in (1..9) {
            if counts[number] <= 12 {
                continue;
            }
            let mut amount_left = counts[number];
            let mut current_index = 0;
            while amount_left > 12 && current_index < line.len() {
                if line[current_index] == number.try_into().unwrap() {
                    line.remove(current_index);
                    amount_left -= 1;
                } else {
                    current_index += 1;
                }
            }
            println!("Optimized {}: {:?}", number, line);
        }
        println!("Fully Optimized: {:?}", line);
        println!();
    }

    let total: u128 = input
        .iter()
        .map(|line| make_number(line.to_vec(), 12))
        .sum();

    println!("{}", total);
}

fn make_number(line: Vec<u128>, remaining_length: usize) -> u128 {
    println!("{}; {:?}", remaining_length, line);
    if remaining_length == 1 {
        return *line.iter().max().unwrap();
    }

    let mut to_try: Vec<usize> = Vec::new();
    let counts: Vec<usize> = (0..10)
        .map(|i| line.iter().filter(|x| **x == i).count())
        .collect();
    println!("{:?}", counts);
    for digit in (1..10).rev() {
        let mut occurences: Vec<usize> = line
            .iter()
            .enumerate()
            .filter(|(k, v)| **v == digit)
            .map(|(k, v)| k)
            .collect();
        to_try.append(&mut occurences);
    }
    let mut maximum = 0;
    for i in to_try {
        if line.len() - i < remaining_length {
            continue;
        }
        let attempt = make_number(line[i + 1..].to_vec(), remaining_length - 1);
        let ten: u128 = 10;
        let new_total = ten.pow((remaining_length - 1).try_into().unwrap()) * line[i] + attempt;
        if new_total > maximum {
            maximum = new_total;
            break;
        }
    }
    maximum
}
