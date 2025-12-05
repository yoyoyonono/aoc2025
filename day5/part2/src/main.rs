fn main() {
    let binding = std::fs::read_to_string("input.txt").unwrap();
    let input: Vec<&str> = binding.split("\n\n").collect();

    let mut fresh_ranges: Vec<[i128; 2]> = input[0]
        .lines()
        .map(|line| {
            let x_split: Vec<&str> = line.split('-').collect();
            println!("{}, {}", x_split[0], x_split[1]);
            return [x_split[0].parse().unwrap(), x_split[1].parse().unwrap()];
        })
        .collect();
    fresh_ranges.sort();

    let mut super_fresh = reduce(fresh_ranges);

    let mut old_length = 0;
    while super_fresh.len() != old_length {
        old_length = super_fresh.len();
        super_fresh = reduce(super_fresh);
    }

    println!("{:?}", super_fresh);

    let total: i128 = super_fresh
        .into_iter()
        .map(|[low, high]| high - low + 1)
        .sum();

    println!("{}", total);
}

fn reduce(fresh_ranges: Vec<[i128; 2]>) -> Vec<[i128; 2]> {
    let mut super_fresh: Vec<[i128; 2]> = fresh_ranges.clone();

    let mut current_range = 0;
    while current_range < super_fresh.len() {
        for (index, other_range) in super_fresh.iter().enumerate().skip(current_range + 1) {
            // println!("{:?}", super_fresh);
            println!("{:?} vs {:?}", super_fresh[current_range], other_range);
            if can_combine(super_fresh[current_range], *other_range)
                || can_combine(*other_range, super_fresh[current_range])
            {
                super_fresh.push(combine(super_fresh[current_range], *other_range));
                super_fresh.remove(index);
                super_fresh.remove(current_range);
                break;
            }
        }
        current_range += 1;
    }

    super_fresh
}

fn can_combine(range_1: [i128; 2], range_2: [i128; 2]) -> bool {
    if range_1[0] <= range_2[0] && range_2[0] <= range_1[1] {
        return true;
    }
    if range_1[0] <= range_2[1] && range_2[1] <= range_1[1] {
        return true;
    }
    return false;
}

fn combine(range_1: [i128; 2], range_2: [i128; 2]) -> [i128; 2] {
    return [range_1[0].min(range_2[0]), range_1[1].max(range_2[1])];
}
