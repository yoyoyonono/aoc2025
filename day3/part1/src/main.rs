fn main() {
    let input: Vec<Vec<u32>> = std::fs::read_to_string("input.txt")
        .unwrap()
        .lines()
        .map(|x| x.chars().map(|z| z.to_digit(10).unwrap()).collect())
        .collect();

    let total: u32 = input
        .iter()
        .map(|line| {
            let mut maximum = 0;
            for (i, a) in line.iter().enumerate() {
                for b in line[i + 1..].iter() {
                    if a * 10 + b > maximum {
                        maximum = a * 10 + b;
                    }
                }
            }
            println!("{}", maximum);
            maximum
        })
        .sum();

    println!("{}", total);
}
