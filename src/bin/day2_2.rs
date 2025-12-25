use std::fs;

fn main() {
    let file_path = "inputs/input2.txt";

    let contents = fs::read_to_string(file_path).expect("Error reading file");
    let ranges = contents.split(',');

    let mut result: i64 = 0;

    for range in ranges {
        let (left, right) = range.split_once('-').unwrap();

        for num in left.parse::<i64>().unwrap()..right.parse::<i64>().unwrap() + 1 {
            let num_string = num.to_string();

            'currentNum:
            for slice_size in 1..(num_string.len() / 2) + 1 {

                let mut index = 0;
                if num_string.len()%slice_size != 0 {
                    continue 'currentNum;
                }
                while num_string[index..index + slice_size]
                    == num_string[index + slice_size..index + slice_size * 2]
                {
                    index += slice_size;
                    if(index+slice_size) >= num_string.len(){
                        result += num;
                        break 'currentNum;
                    }
                }
            }
        }
    }
    println!("{}", result);
}
