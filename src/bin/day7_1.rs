use std::collections::HashSet;
use std::fs;

fn main() {
    let file_path = "inputs/input7.txt";
    let contents = fs::read_to_string(file_path).expect("Error reading file");

    let mut result:usize = 0;

    //stores beam locations
    let mut beams = HashSet::new();
    beams.insert(contents.find('S').unwrap()); // add centre beam

    // every other line
    for line in contents.lines().skip(2).step_by(2) {
        let splitters: HashSet<usize> = line.match_indices('^').map(|(i,_)| i).collect();
        let matches: Vec<usize> = splitters.intersection(&beams).copied().collect();
        for matched in matches {
            result+=1;
            beams.remove(&matched);
            beams.insert(matched+1);
            beams.insert(matched-1);
        }
    }

    println!("{}", result);
}
