use std::fs;

fn main() {
    let file_path = "inputs/input6.txt";
    let contents = fs::read_to_string(file_path).expect("Error reading file");

    let mut result:u64 = 0;

    let lines = contents
        .lines()
        .map(|l| l.split_whitespace().collect::<Vec<&str>>())
        .collect::<Vec<Vec<&str>>>();

    for task in 0..lines[0].len() {
        let operator = lines.last().unwrap()[task].chars().next().unwrap();
        let mut temp:u64 = lines.first().unwrap()[task].parse::<u64>().unwrap();
        for row in 1..lines.len()-1 {
            let number:u64 = lines[row][task].parse::<u64>().unwrap();
            match operator {
                '+' => temp += number,
                '*' => temp *= number,
                _ => panic!("Unknown operator: {}", operator)
            }
        }
        result += temp;
    }

    println!("{}", result);
}
