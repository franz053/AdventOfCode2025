use std::fs;

fn main() {
    let file_path = "inputs/input3.txt";

    let contents = fs::read_to_string(file_path).expect("Error reading file");
    let lines = contents.split_whitespace();

    let mut result = 0;

    for line in lines {
        let mut pos_left = 0;
        for digit in (0..=9).rev() {
            let digit_char = char::from_digit(digit, 10).unwrap();
            let pos = line[..line.len() - 1].find(digit_char); //exclude last for first digit
            if pos.is_some() {
                pos_left = pos.unwrap();
                let add_char = line.chars().nth(pos.unwrap()).unwrap();
                result += add_char.to_digit(10).unwrap() * 10;
                break;
            }
        }
        for digit in (0..=9).rev() {
            let digit_char = char::from_digit(digit, 10).unwrap();
            let pos = line[pos_left + 1..].find(digit_char); //search right of first digit
            if pos.is_some() {
                let add_char = line.chars().nth(pos.unwrap() + pos_left + 1).unwrap();
                result += add_char.to_digit(10).unwrap();
                break;
            }
        }
    }
    println!("{}", result);
}
