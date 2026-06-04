use std::fs;

fn calc_area(p1: (isize, isize), p2: (isize, isize)) -> usize {
    let h = p1.0.abs_diff(p2.0)+1;
    let w = p1.1.abs_diff(p2.1)+1;
    h * w
}

fn main() {
    let file_path = "inputs/input9.txt";
    let contents = fs::read_to_string(file_path).expect("Error reading file");

    //get all coordinates into a vector
    let red_tiles: Vec<(isize, isize)> = contents
        .lines()
        .map(|line| {
            let (l, r) = line.split_once(',').unwrap();
            (l.parse().unwrap(), r.parse().unwrap())
        })
        .collect();

    let mut result = 1;


    //loop over every pair and find largest area (probably not the most efficient way)
    for t1 in 0..red_tiles.len() {
        for t2 in (t1 + 1)..red_tiles.len() {
            let area = calc_area(red_tiles[t1], red_tiles[t2]);
            if area > result {
                result = area;
            }
        }
    }

    println!("{}", result);
}
