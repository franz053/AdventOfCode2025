use std::fs;

fn check_field(map: &Vec<Vec<char>>, field: (usize, usize)) -> bool {
    if map[field.0][field.1] == '@' {
        let mut count = 0;
        for i in 0..=2 {
            for j in 0..=2 {
                if map[(field.0 + i)-1][(field.1 + j)-1] == '@' {
                    count += 1;
                }
            }
        }
        if count <= 4 { true } else { false } //4, since counting myself as well
    } else {
        false
    }
}

fn main() {
    let file_path = "inputs/input4.txt";
    let contents = fs::read_to_string(file_path).expect("Error reading file");

    let mut map: Vec<Vec<char>> = contents.lines().map(|l| l.chars().collect()).collect();
    //Add padding
    for row in &mut map {
        row.insert(0, '.');
        row.push('.')
    }
    map.insert(0, vec!['.'; map[0].len()]);
    map.push(vec!['.'; map[0].len()]);
    let mut result = 0;
    loop {
        let mut done = true;
        for i in 1..map.len() - 1 {
            for j in 1..map[i].len() - 1 {
                if check_field(&map, (i, j)) {
                    result += 1;
                    map[i][j] = '.';
                    if done {done = false};
                }
            }
        }
        if done {break;}
    }
    println!("{}", result);
}
