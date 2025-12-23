use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    let contents = fs::read_to_string(file_path).expect("Error reading file");
    let words = contents.split_whitespace(); //Iterator over words

    let mut current_value = 50;
    let mut result = 0;

    for word in words {
        let dir: bool = &word[..1] == "R"; // Positive or negative direction
        let num: i32 = word[1..].parse::<i32>().unwrap(); //amount to turn

        current_value += if dir { num } else { -num };
        current_value = ((current_value % 100) + 100) % 100; //wrap into range 0-99

        if current_value == 0 {
            result += 1;
        }
    }
    println!("{}", result);
}
