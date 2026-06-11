use std::collections::HashMap;
use std::fs;

// Recursive solution
fn calc_paths(name: &str, map: &HashMap<&str, Vec<&str>>) -> usize {
    match name {
        "out" => 1,
        _ => map[name].iter().map(|dev| calc_paths(dev, map)).sum(),
    }
}
fn main() {
    let file_path = "inputs/input11.txt";
    let contents = fs::read_to_string(file_path).expect("Error reading file");

    let devices: HashMap<&str, Vec<&str>> = contents
        .lines()
        .map(|l| {
            let (l, r) = l.split_once(": ").unwrap();
            (l, r.split(" ").collect())
        })
        .collect();

    let result = calc_paths("you", &devices);

    println!("{}", result);
}
