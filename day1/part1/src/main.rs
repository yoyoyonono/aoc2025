fn main() {
    let input = std::fs::read_to_string("input.txt")
        .unwrap();
    let input_lines = input.lines();

    let mut count = 0;
    let mut position = 50;
    for line in input_lines {
        let number: i32 = line.chars().skip(1).collect::<String>().parse().unwrap();
        if line.chars().nth(0).unwrap() == 'L' {
            position -= number;
        } else {
            position += number;
        }
        position = position % 100;
        println!("{}: {}", line, position);
        if position == 0 {
            count += 1;            
        }
    }
    println!("{}", count);    
}
