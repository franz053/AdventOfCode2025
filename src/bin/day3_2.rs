use std::fs;

fn main() {
    let file_path = "inputs/input3.txt";

    let contents = fs::read_to_string(file_path).expect("Error reading file");
    let lines = contents.split_whitespace();

    let mut result: u64 = 0;

    for line in lines {
        let mut last_pos_left = 0; //where was the last digit found, don't check before that
        for power_space in (0..=11).rev() {
            //power_space records both power of current digit and space needed for following digits
            for digit in (0..=9).rev() {
                let digit_char = char::from_digit(digit, 10).unwrap();
                let pos = line[last_pos_left..line.len() - power_space].find(digit_char);
                if pos.is_some() {
                    let add_char = line.chars().nth(pos.unwrap() + last_pos_left).unwrap();
                    result +=
                        add_char.to_digit(10).unwrap() as u64 * 10_u64.pow(power_space as u32);
                    last_pos_left = last_pos_left + pos.unwrap() + 1;
                    break;
                }
            }
        }
    }
    println!("{}", result);
}
