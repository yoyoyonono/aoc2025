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
            if length % 2 != 0 {
                return 0;
            }
            if i_string[..length / 2] == i_string[length / 2..] {
                println!("{}", i);
                return i.into();
            }
            0
        }
        ).sum();
        total
    }
    ).sum();

    println!("{}", total);
    
}
