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
    let input: Vec<Vec<&str>> = binding
        .lines()
        .map(|line| line.split_ascii_whitespace().collect())
        .collect();

    let mut problems: Vec<Problem> = Vec::new();

    for col in 0..input[0].len() {
        problems.push(Problem {
            numbers: vec![],
            operation: Operation::Add,
        });
        let last_problem = problems.len() - 1;
        for row in 0..input.len() - 1 {
            println!("{}", input[row][col]);
            problems[last_problem]
                .numbers
                .push(input[row][col].parse().unwrap());
        }
        problems[last_problem].operation = match input[input.len() - 1][col] {
            "*" => Operation::Multiply,
            _ => Operation::Add,
        };
    }

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
