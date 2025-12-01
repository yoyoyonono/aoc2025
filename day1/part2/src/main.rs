fn main() {
    let input = std::fs::read_to_string("input.txt")
        .unwrap();
    let input_lines = input.lines();

    let mut count = 0;
    let mut position = 50;
    let mut last_position = 50;
    for line in input_lines {
        last_position = position;
        println!("{}, {}", line, last_position);
        let mut number: i32 = line.chars().skip(1).collect::<String>().parse().unwrap();
        if number > 100 {
            count += number / 100;
            number %= 100;
        }
        if line.chars().nth(0).unwrap() == 'L' {
            position -= number;
        } else {
            position += number;
        }
        let position_mod = position.rem_euclid(100);
        println!("{}, {}", position, position_mod);
        if position != position_mod && last_position != 0{
            count += 1
        } else if position_mod == 0 {
            count += 1;            
        }
        println!("{}", count);
        position = position_mod;
        println!();
    }
    println!("{}", count);    
}
