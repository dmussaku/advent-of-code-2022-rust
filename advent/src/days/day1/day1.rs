use std::fs;

pub fn read_numbers_from_file(path: &str) -> Vec<i32>{
    let contents = fs::read_to_string(path)
        .expect("Something went wrong with a file");
    let numbers: Vec<i32> = contents.lines()
        .map(|val| val.parse::<i32>().expect("parse error"))
        .collect();
    numbers
}

pub fn run_part_1(input_numbers: Vec<i32>) -> i32 {
    let mut count = 0;

    // println!("{} (N/A - no previous measurement)", input_numbers[0]);
    for i in 1..input_numbers.len(){
        if input_numbers[i] > input_numbers[i-1]{
            count += 1;
            // println!("{} (increased)", input_numbers[i]);
        }
        else{
            // println!("{} (decreased)", input_numbers[i]);
        }
    }
    count
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test_read_input_numbers() {
        let input_numbers = read_numbers_from_file(
            "src/days/day1/input_files/test_file.txt"
        );

        assert_eq!(vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263], input_numbers);
    }

    #[test]
    fn test_run_part_1(){
        assert_eq!(7, run_part_1(vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263]));
    }
}

