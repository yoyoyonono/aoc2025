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

    let mut currently_accessible_papers = accessible(&papers);
    let mut removed = 0;
    while currently_accessible_papers.len() > 0 {
        removed += currently_accessible_papers.len();
        for paper in &currently_accessible_papers {
            papers.remove(paper);
        }
        currently_accessible_papers = accessible(&papers);
    }
    println!("{removed}");
}

fn print_grid(grid: Vec<Vec<char>>) {
    for line in grid {
        for c in line {
            print!("{c}");
        }
        println!();
    }
}

fn accessible(papers: &HashSet<[isize; 2]>) -> Vec<[isize; 2]> {
    papers
        .iter()
        .filter(|[y, x]| {
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

            if count < 4 { true } else { false }
        })
        .map(|x| x.to_owned())
        .collect()
}
