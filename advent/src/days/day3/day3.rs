use std::fs;
use std::collections::HashSet;


pub fn read_file_lines(path: &str) -> Vec<String> {
    fs::read_to_string(path)
        .expect("Something went wrong with a file")
        .lines()
        .map(|line| line.to_string())
        .collect()
}

pub fn read_input_from_file_part1(path: &str) -> Vec<(String, String)> {
    read_file_lines(path)
        .into_iter()
        .map(|line| {
            let size = line.len();
            let middle_element_index: usize = size / 2;
            let first_compartment = line[..middle_element_index].to_string();
            let second_compartment = line[middle_element_index..].to_string();
            (first_compartment, second_compartment)
        })
        .collect()
}

pub fn read_input_from_file_part2(path: &str) -> Vec<Vec<String>> {
    read_file_lines(path)
        .chunks(3)
        .map(|chunk| chunk.to_vec())
        .collect()
}

fn recursive_same_element(rucksacks: Vec<String>) -> Option<HashSet<char>>{
    if rucksacks.len() < 2{
        return None
    }
    else if rucksacks.len() == 2{
        let first: HashSet<char> = rucksacks[0].clone().chars().collect();
        let second: HashSet<char> = rucksacks[1].clone().chars().collect();
        return Some(first.intersection(&second).cloned().collect::<HashSet<char>>());
    }
    else{
        let first: HashSet<char> = rucksacks[0].clone().chars().collect();
        let rest_set = recursive_same_element(rucksacks[1..].to_vec());
        if let Some(rest) = rest_set{
            return Some(first.intersection(&rest).cloned().collect::<HashSet<char>>());
        }
    }
    None
}

fn set_items_priority(item: &char) -> u8 {
    let char_val = *item as u8;
    if char_val >= b'a' {
        char_val - b'a' + 1
    } else {
        char_val - b'A' + 27
    }
}

pub fn run_part_1(input: Vec<(String, String)>) -> usize {
    let mut result = 0;

    for (first_compartment, second_compartment) in input{
        let same_elements = recursive_same_element(vec![first_compartment, second_compartment]);
        if let Some(elements) = same_elements{
            for element in elements{
                result += set_items_priority(&element) as usize;
            }
        }
    }
    result
}

pub fn run_part_2(input: Vec<Vec<String>>) -> usize {
    let mut result = 0;

    for chunk in input{
        let same_elements = recursive_same_element(chunk);
        if let Some(elements) = same_elements{
            for element in elements{
                result += set_items_priority(&element) as usize;
            }
        }
    }
    result
}


#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test_read_input_from_file_part1() {
        let input= read_input_from_file_part1(
            "src/days/day3/input_files/test_file.txt"
        );
        let expected_result = vec![("vJrwpWtwJgWr", "hcsFMMfFFhFp"), ("jqHRNqRjqzjGDLGL", "rsFMfFZSrLrFZsSL"), ("PmmdzqPrV", "vPwwTWBwg"), ("wMqvLMZHhHMvwLH", "jbvcjnnSBnvTQFn"), ("ttgJtRGJ", "QctTZtZT"), ("CrZsJsPPZsGz", "wwsLwLmpwMDw")]
            .iter()
            .map(|(first_compartment, second_compartment)| (first_compartment.to_string(), second_compartment.to_string()))
            .collect::<Vec<(String, String)>>();
        assert_eq!(input, expected_result);
    }

    #[test]
    fn test_read_input_from_file_part2() {
        let input = read_input_from_file_part2(
            "src/days/day3/input_files/test_file.txt"
        );
        let expected_result = [["vJrwpWtwJgWrhcsFMMfFFhFp", "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL", "PmmdzqPrVvPwwTWBwg"], ["wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn", "ttgJtRGJQctTZtZT", "CrZsJsPPZsGzwwsLwLmpwMDw"]]
            .iter()
            .map(|chunk| chunk.iter().map(|line| line.to_string()).collect::<Vec<String>>())
            .collect::<Vec<Vec<String>>>();
        assert_eq!(input, expected_result)
    }

    #[test]
    fn test_recursive_same_element_2_strings(){
        let result = recursive_same_element(vec!["vJrwpWtwJgWr".to_string(), "hcsFMMjfFFhFp".to_string()]);
        assert_eq!(result, Some(['p'].iter().cloned().collect::<HashSet<char>>()));
    }

    #[test]
    fn test_recursive_same_elemt_3_strings(){
        let result = recursive_same_element(vec!["vJrwpWtwJgWrjp".to_string(), "hcsFMMjfFFhFp".to_string(), "jqHRNqRjpqzjGDLGL".to_string()]);
        assert_eq!(result, Some(['j', 'p'].iter().cloned().collect::<HashSet<char>>()));
    }

    #[test]
    fn test_set_items_priority(){
        assert_eq!(set_items_priority(&'a'), 1);
        assert_eq!(set_items_priority(&'z'), 26);
        assert_eq!(set_items_priority(&'A'), 27);
        assert_eq!(set_items_priority(&'Z'), 52);
    }

    #[test]
    fn test_run_part_1(){
        let input = read_input_from_file_part1("src/days/day3/input_files/test_file.txt");
        assert_eq!(run_part_1(input), 157);
    }

    #[test]
    fn test_run_part_2(){
        let input = read_input_from_file_part2("src/days/day3/input_files/test_file.txt");
        assert_eq!(run_part_2(input), 70);
    }

}

