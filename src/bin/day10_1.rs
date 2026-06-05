use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashSet};
use std::fs;
use std::str::FromStr;

struct Machine {
    target_indicators: u64, //binary representation
    buttons: Vec<u64>,
    //joltages: Vec<usize>,
}

impl FromStr for Machine {
    type Err = String;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let (ind_slice, but_jol_slice) = line.split_once(' ').ok_or("wrong input")?;
        let (but_slice, _jlt_slice) = but_jol_slice.rsplit_once(' ').ok_or("wrong input")?;

        let pos0 = 1 << ind_slice.len() - 3;

        Ok(Self {
            target_indicators: ind_slice[1..ind_slice.len() - 1]
                .chars()
                .fold(0u64, |acc, c| (acc << 1) | (c == '#') as u64),

            buttons: but_slice
                .split(' ')
                .map(|b| {
                    b[1..b.len() - 1]
                        .split(',')
                        .fold(0u64, |acc, bit| acc + (pos0 >> bit.parse::<u64>().unwrap()))
                })
                .collect(),
        })
    }
}

fn main() {
    let file_path = "inputs/input10.txt";
    let contents = fs::read_to_string(file_path).expect("Error reading file");

    let machines: Vec<Machine> = contents
        .lines()
        .map(str::parse)
        .collect::<Result<_, _>>()
        .unwrap();

    //Dijkstra

    let mut result = 0;

    for m in machines {
        let mut locations = BinaryHeap::new(); // (lowest_distance, indicators)
        let mut visited = HashSet::new();
        locations.push(Reverse((0u64, 0u64)));

        'search: loop {
            let current = locations.pop().unwrap().0;
            visited.insert(current.1);
            for &button in &m.buttons {
                let next = current.1 ^ button;

                if next == m.target_indicators {
                    result += current.0 + 1;
                    break 'search;
                } else if !visited.contains(&next) {
                    locations.push(Reverse((current.0 + 1, next)));
                }
            }
        }
    }

    println!("{}", result);
}
