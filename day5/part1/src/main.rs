fn main() {
    let binding = std::fs::read_to_string("input.txt").unwrap();
    let input: Vec<&str> = binding.split("\n\n").collect();

    let fresh_ranges: Vec<[i128; 2]> = input[0]
        .lines()
        .map(|line| {
            let x_split: Vec<&str> = line.split('-').collect();
            println!("{}, {}", x_split[0], x_split[1]);
            return [x_split[0].parse().unwrap(), x_split[1].parse().unwrap()];
        })
        .collect();

    let available_ids: Vec<i128> = input[1].lines().map(|x| x.parse().unwrap()).collect();

    let total: usize = available_ids
        .iter()
        .filter(|id| {
            for range in &fresh_ranges {
                if range[0] <= **id && **id <= range[1] {
                    return true;
                }
            }
            false
        })
        .count();

    println!("{total}");
}
