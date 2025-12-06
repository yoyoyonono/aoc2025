use itertools::all;

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
    let input: Vec<Vec<char>> = binding.lines().map(|line| line.chars().collect()).collect();

    let mut problems: Vec<Problem> = Vec::new();

    let mut problem_starting_cols: Vec<usize> = vec![0];
    let mut the_rest: Vec<usize> = (0..input[0].len())
        .filter_map(|col| {
            if all(0..input.len(), |row| input[row][col] == ' ') {
                return Some(col + 1);
            }
            None
        })
        .collect();
    problem_starting_cols.append(&mut the_rest);

    for (index, &first_problem_col) in problem_starting_cols.iter().enumerate() {
        let last = if index == problem_starting_cols.len() - 1 {
            input[0].len()
        } else {
            problem_starting_cols[index + 1]
        };
        let operation = if (input[input.len() - 1][first_problem_col]) == '*' {
            Operation::Multiply
        } else {
            Operation::Add
        };
        let mut numbers: Vec<i128> = Vec::new();
        for number_col in first_problem_col..last {
            let mut new_number: i128 = 0;
            let mut seen_number_yet = false;
            for row in 0..input.len() - 1 {
                println!("{row} {number_col}");
                if input[row][number_col] == ' ' {
                    if !seen_number_yet {
                        continue;
                    }
                    break;
                }
                seen_number_yet = true;
                new_number *= 10;
                let new_digit: i128 = input[row][number_col]
                    .to_digit(10)
                    .unwrap()
                    .try_into()
                    .unwrap();
                new_number += new_digit;
            }
            numbers.push(new_number);
        }
        if numbers.contains(&0) {
            numbers.pop();
        }
        problems.push(Problem { numbers, operation });
    }

    println!("{:#?}", problems);

    let total: i128 = problems
        .iter()
        .map(|problem| {
            match problem.operation {
                Operation::Add => {
                    return problem.numbers.iter().sum();
                }
                Operation::Multiply => {
                    return problem.numbers.iter().product();
                }
            }
            return 0;
        })
        .sum();

    println!("{}", total);
}
