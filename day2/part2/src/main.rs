fn main() {
    let input = std::fs::read_to_string("input.txt")
        .unwrap();
    let entries = input.split(',');
    let total: i64 = entries.map( |entry|  {
        let split: Vec<&str> = entry.split('-').collect();
        let beginning: i64= split[0].parse().unwrap();
        let end: i64 = split[1].parse().unwrap();
        let total: i64 = (beginning..end + 1).map(
            |i| {
            let i_string = i.to_string();
            let length = i_string.len();
            // println!("Current: {}, Length: {}, Max: {}", i, length, length / 2 + 1);
            for divisor in 2..length + 1 {
                // println!("Divisor: {}", divisor);
                if length % divisor != 0 {
                    continue;
                }
                let parts: Vec<&[u8]> = i_string.as_bytes().chunks(length / divisor).collect();
                let first_part = parts.first().unwrap();
                if parts.iter().all(|&x| &x == first_part) {
                    println!("Invalid: {}", i);
                    return i;
                }                
            }
            0
        }
        ).sum();
        total
    }
    ).sum();

    println!("{}", total);
    
}
