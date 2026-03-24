use std::cmp::max;
use std::fs;

fn main() {
    let file_path = "inputs/input5.txt";
    let contents = fs::read_to_string(file_path).expect("Error reading file");

    let mut result = 0;

    let (ranges_str, numbers_str) = contents.split_once("\r\n\r\n").unwrap();

    let mut ranges: Vec<(u64,u64)> = ranges_str.lines().map(|line| {
        let (l,r) = line.split_once('-').unwrap();
        return (l.parse().unwrap(),r.parse().unwrap());
    }).collect();

    ranges.sort_unstable();

    let mut merged: Vec<(u64,u64)> = Vec::new();

    let mut i = 0;
    while i < ranges.len() {
        let mut range = ranges[i];
        while range.1 + 1 >= ranges[i].0 {
            range.1 = max(range.1, ranges[i].1);
            i+=1;
            if i >= ranges.len() {break}
        }
        merged.push(range);
    }

    'outer: for number in numbers_str.lines().map(|line| line.parse::<u64>().unwrap()) {
        for range in &merged {
            if number >= range.0 && number <= range.1 {
                result+=1;
                continue 'outer;
            }
        }
    }

    println!("{}", result);
}
