use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;

fn main() {
    let file_path = "inputs/input7.txt";
    let contents = fs::read_to_string(file_path).expect("Error reading file");

    let mut result:usize = 1;

    //stores beam locations
    let mut beams:HashMap<usize,usize> = HashMap::new();
    beams.insert(contents.find('S').unwrap(),1); // add centre beam

    // every other line
    for line in contents.lines().skip(2).step_by(2) {
        let splitters: HashSet<usize> = line.match_indices('^').map(|(i,_)| i).collect();
        let beam_keys: HashSet<usize> = beams.keys().copied().collect();
        let matches: Vec<usize> = splitters.intersection(&beam_keys).copied().collect();
        for matched in matches {
            let match_value = beams.remove(&matched).unwrap();
            beams.entry(matched+1).and_modify(|x| *x += match_value).or_insert(match_value);
            beams.entry(matched-1).and_modify(|x| *x += match_value).or_insert(match_value);
            result+=match_value;
        }
    }

    println!("{}", result);
}
