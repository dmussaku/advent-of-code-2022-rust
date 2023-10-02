use std::fs;
use std::collections::HashSet;


pub fn read_input_from_file(path: &str) -> Vec<(String, String)>{
    let contents = fs::read_to_string(path)
        .expect("Something went wrong with a file");
    let result = contents.lines()
        .map(|line| {
            let size = line.len();
            let middle_element_index: usize = size / 2;
            let first_compartment = line[..middle_element_index].to_string();
            let second_compartment = line[middle_element_index..].to_string();
            (first_compartment, second_compartment)
        }).collect();
    result
}

fn find_same_element(first_compartment: &str, second_compartment: &str) -> Option<char>{
    let first_compartment_items = first_compartment.chars().collect::<HashSet<char>>();
    let second_compartment_items = second_compartment.chars().collect::<HashSet<char>>();    
    let results = first_compartment_items.intersection(&second_compartment_items)
        .collect::<Vec<&char>>()
        .clone()
        .to_owned();
    if results.len() > 0{
        return Some(*results[0]);
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

pub fn run_part_1() -> usize {
    let input = read_input_from_file("./input_files/input.txt");
    let mut result = 0;

    for (first_compartment, second_compartment) in input{
        if let Some(element) = find_same_element(&first_compartment, &second_compartment){
            result += set_items_priority(&element) as usize;
        }
    }
    result
}


#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test_read_input_numbers() {
        let input= read_input_from_file(
            "src/days/day3/input_files/test_file.txt"
        );
        let expected_result = vec![("vJrwpWtwJgWr", "hcsFMMfFFhFp"), ("jqHRNqRjqzjGDLGL", "rsFMfFZSrLrFZsSL"), ("PmmdzqPrV", "vPwwTWBwg"), ("wMqvLMZHhHMvwLH", "jbvcjnnSBnvTQFn"), ("ttgJtRGJ", "QctTZtZT"), ("CrZsJsPPZsGz", "wwsLwLmpwMDw")]
            .iter()
            .map(|(first_compartment, second_compartment)| (first_compartment.to_string(), second_compartment.to_string()))
            .collect::<Vec<(String, String)>>();
        assert_eq!(input, expected_result);
    }

    #[test]
    fn test_find_same_element(){
        let result = find_same_element("vJrwpWtwJgWr", "hcsFMMfFFhFp");
        assert_eq!(result, Some('p'));
    }

    #[test]
    fn test_set_items_priority(){
        assert_eq!(set_items_priority(&'a'), 1);
        assert_eq!(set_items_priority(&'z'), 26);
        assert_eq!(set_items_priority(&'A'), 27);
        assert_eq!(set_items_priority(&'Z'), 52);
    }

}

