use std::fs;
fn main() {
    let file_path = "inputs/input1.txt";

    let contents = fs::read_to_string(file_path).expect("Error reading file");
    let words = contents.split_whitespace(); //Iterator over words

    let mut current_value = 50;
    let mut result = 0;

    for word in words {
        let dir: bool = &word[..1] == "R"; // Positive or negative direction
        let num: i32 = word[1..].parse::<i32>().unwrap(); //amount to turn

        current_value += if dir { num } else { -num };

        if current_value == 0 {
            result += 1;
        } else if current_value >= 100 {
            result += current_value / 100;
        } else if current_value < 0 {
            if current_value + num != 0 {
                //check if value was 0 before, and don't count it twice if it is
                result += 1
            }
            result += current_value / -100;
        }
        current_value = ((current_value % 100) + 100) % 100; //wrap into range 0-99
    }
    println!("{}", result);
}
