use std::fs;

fn main() {
    let file_path = "inputs/input2.txt";

    let contents = fs::read_to_string(file_path).expect("Error reading file");
    let ranges = contents.split(',');

    let mut result:i64 = 0;

    for range in ranges {
        let (left, right) = range.split_once('-').unwrap();

        for num in left.parse::<i64>().unwrap()..right.parse::<i64>().unwrap()+1 {
            let str = num.to_string();
            let (left, right) = str.split_at(str.len() / 2);
            if left == right {
                result += num;
            }
        }
    }
    println!("{}", result);
}