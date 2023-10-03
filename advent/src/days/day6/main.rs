use std::fs;

pub fn read_input_from_file(path: &str) -> String {
    fs::read_to_string(path)
        .expect("Something went wrong with a file")
        .lines()
        .next()
        .unwrap()
        .to_string()
}

fn find_solution(input: String, expected_position: usize) -> usize{
    let mut characters: Vec<char> = Vec::new();
    for (i, letter) in input.chars().enumerate(){
        if characters.len() == expected_position{
            return i
        }
        else {
            if !characters.contains(&letter){
                characters.push(letter);
            } else {
                let char_position = characters.iter().position(|&x| x == letter).unwrap();
                characters = characters[char_position+1..].to_vec();
                characters.push(letter);
            }
        }
    }
    0
}

pub fn run_part_1(input: String) -> usize{
    find_solution(input, 4)
}

pub fn run_part_2(input: String) -> usize{
    find_solution(input, 14)
}

// pub fn run_part_2() -> usize {
//     0
// }


#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test_read_input_from_file(){
        let result = read_input_from_file("src/days/day6/input_files/test_file.txt");
        assert_eq!(result, "mjqjpqmgbljsphdztnvjfqwrcgsmlb")
    }

    #[test]
    fn test_run_part_1(){
        assert_eq!(run_part_1("mjqjpqmgbljsphdztnvjfqwrcgsmlb".to_string()), 7);
        assert_eq!(run_part_1("bvwbjplbgvbhsrlpgdmjqwftvncz".to_string()), 5);
        assert_eq!(run_part_1("nppdvjthqldpwncqszvftbrmjlhg".to_string()), 6);
        assert_eq!(run_part_1("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg".to_string()), 10);
        assert_eq!(run_part_1("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw".to_string()), 11);
    }

    #[test]
    fn test_run_part_2(){
        assert_eq!(run_part_2("mjqjpqmgbljsphdztnvjfqwrcgsmlb".to_string()), 19);
        assert_eq!(run_part_2("bvwbjplbgvbhsrlpgdmjqwftvncz".to_string()), 23);
        assert_eq!(run_part_2("nppdvjthqldpwncqszvftbrmjlhg".to_string()), 23);
        assert_eq!(run_part_2("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg".to_string()), 29);
        assert_eq!(run_part_2("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw".to_string()), 26);
    }

}
