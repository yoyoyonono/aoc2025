use std::collections::HashSet;

fn main() {
    let input: Vec<Vec<char>> = std::fs::read_to_string("input.txt")
        .unwrap()
        .lines()
        .map(|x| x.chars().collect())
        .collect();

    let mut papers: HashSet<[isize; 2]> = HashSet::new();

    for (y, line) in input.iter().enumerate() {
        for (x, character) in line.iter().enumerate() {
            if *character == '@' {
                papers.insert([y.try_into().unwrap(), x.try_into().unwrap()]);
            }
        }
    }

    println!("{:?}", papers);

    let mut space: Vec<Vec<char>> = input.clone();

    let total: i32 = papers
        .iter()
        .map(|[y, x]| {
            let mut count = 0;
            if papers.contains(&[y - 1, x - 1]) {
                count += 1;
            }
            if papers.contains(&[y - 1, *x]) {
                count += 1;
            }
            if papers.contains(&[y - 1, x + 1]) {
                count += 1;
            }
            if papers.contains(&[*y, x - 1]) {
                count += 1;
            }
            if papers.contains(&[*y, x + 1]) {
                count += 1;
            }
            if papers.contains(&[y + 1, x - 1]) {
                count += 1;
            }
            if papers.contains(&[y + 1, *x]) {
                count += 1;
            }
            if papers.contains(&[y + 1, x + 1]) {
                count += 1;
            }

            // println!("[{y}, {x}]: {count}");
            let yu: usize = (*y).try_into().unwrap();
            let xu: usize = (*x).try_into().unwrap();
            space[yu][xu] = count.to_string().chars().nth(0).unwrap();
            if count < 4 { 1 } else { 0 }
        })
        .sum();

    print_grid(space);
    println!("{}", total);
}

fn print_grid(grid: Vec<Vec<char>>) {
    for line in grid {
        for c in line {
            print!("{c}");
        }
        println!();
    }
}
