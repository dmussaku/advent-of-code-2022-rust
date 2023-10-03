use std::fs;

pub fn read_input_from_file(path: &str) -> Vec<(u8, u8, u8, u8)>{
    let result: Vec<_> = fs::read_to_string(path)
        .expect("Something went wrong with a file")
        .lines()
        .map(|line| {
            let (left, right) = line.split_once(',').unwrap();
            let ((a, b), (c, d)) = (left.split_once('-').unwrap(), right.split_once('-').unwrap());
            (
                a.parse::<u8>().unwrap(),
                b.parse::<u8>().unwrap(),
                c.parse::<u8>().unwrap(),
                d.parse::<u8>().unwrap()
            )
        })
        .collect();
    result
}

pub fn run_part_1(input: Vec<(u8, u8, u8, u8)>) -> usize {
    let result = input
        .iter()
        .filter(|(a, b, c, d)| (a >= c && b <= d) || (a <= c && b >= d))
        .count() as usize;
    result
}


#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test_read_input_from_file() {
        let input= read_input_from_file(
            "src/days/day4/input_files/test_file.txt"
        );
        assert_eq!(input, vec![(2, 4, 6, 8), (2, 3, 4, 5), (5, 7, 7, 9), (2, 8, 3, 7), (6, 6, 4, 6), (2, 6, 4, 8)]);
    }

    #[test]
    fn test_run_part_1(){
        let result = run_part_1(vec![(2, 4, 6, 8), (2, 3, 4, 5), (5, 7, 7, 9), (2, 8, 3, 7), (6, 6, 4, 6), (2, 6, 4, 8)]);
        assert_eq!(result, 2);
    }
}

