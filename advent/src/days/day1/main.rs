use std::fs;

pub fn read_numbers_from_file(path: &str) -> Vec<i32>{
    let contents = fs::read_to_string(path)
        .expect("Something went wrong with a file");
    
    let mut unsorted = contents.split("\n\n").
        map(|s| s.lines().map(|x| x.parse::<i32>().unwrap()).sum::<i32>())
        .collect::<Vec<i32>>();
    unsorted.sort();
    unsorted.reverse();
    let result = unsorted.clone();
    result
}

pub fn run_part_1(input_numbers: Vec<i32>) -> i32 {
    // println!("input_numbers: {:?}", input_numbers);
    input_numbers[0]
}

pub fn run_part_2(input_numbers: Vec<i32>) -> i32{
    // println!("input_numbers: {:?}", input_numbers);
    let result = input_numbers[0..3].iter().sum();
    result
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test_read_input_numbers() {
        let input_numbers = read_numbers_from_file(
            "src/days/day1/input_files/test_file.txt"
        );

        assert_eq!(vec![24000, 11000, 10000, 6000, 4000], input_numbers);
    }

    #[test]
    fn test_run_part_1(){
        assert_eq!(run_part_1(vec![24000, 11000, 10000, 6000, 4000]), 24000);
    }

    #[test]
    fn test_run_part_2(){
        assert_eq!(run_part_2(vec![24000, 11000, 10000, 6000, 4000]), 45000);
    }
}

