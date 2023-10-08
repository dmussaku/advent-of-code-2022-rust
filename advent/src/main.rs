mod days;

use days::{day1, day2, day3, day4, day6, day8};

fn main() {
    println!("Day1 part 1 result = {}", day1::main::run_part_1(day1::main::read_numbers_from_file("src/days/day1/input_files/file.txt")));
    println!("Day1 part 2 result = {:?}", day1::main::run_part_2(day1::main::read_numbers_from_file("src/days/day1/input_files/file.txt")));

    println!("Day2 part 1 result = {:?}", day2::main::run_part_1(day2::main::read_input_from_file("src/days/day2/input_files/file.txt")));
    println!("Day2 part 2 result = {:?}", day2::main::run_part_2(day2::main::read_input_from_file("src/days/day2/input_files/file.txt")));

    println!("Day3 part 1 result = {:?}", day3::main::run_part_1(day3::main::read_input_from_file_part1("src/days/day3/input_files/file.txt")));
    println!("Day3 part 2 result = {:?}", day3::main::run_part_2(day3::main::read_input_from_file_part2("src/days/day3/input_files/file.txt")));

    println!("Day4 part 1 result = {:?}", day4::main::run_part_1(day4::main::read_input_from_file("src/days/day4/input_files/file.txt")));
    println!("Day4 part 2 result = {:?}", day4::main::run_part_2(day4::main::read_input_from_file("src/days/day4/input_files/file.txt")));

    println!("Day6 part 1 result = {:?}", day6::main::run_part_1(day6::main::read_input_from_file("src/days/day6/input_files/file.txt")));
    println!("Day6 part 2 result = {:?}", day6::main::run_part_2(day6::main::read_input_from_file("src/days/day6/input_files/file.txt")));

    println!("Day8 part 1 result = {:?}", day8::main::run_part_1(&day8::main::read_input_from_file("src/days/day8/input_files/file.txt")));
}
