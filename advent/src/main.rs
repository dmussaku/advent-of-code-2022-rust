mod days;

use days::{day1};

fn main() {
    println!("Day1 part 1 result = {}", day1::day1::run_part_1(day1::day1::read_numbers_from_file("src/days/day1/input_files/file.txt")));
    println!("Day1 part 2 result = {:?}", day1::day1::run_part_2(day1::day1::read_numbers_from_file("src/days/day1/input_files/file.txt")));
}
