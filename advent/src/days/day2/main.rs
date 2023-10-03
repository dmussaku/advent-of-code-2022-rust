use std::fs;

pub fn read_input_from_file(path: &str) -> Vec<(char, char)>{
    let contents = fs::read_to_string(path)
        .expect("Something went wrong with a file");
    
    let input = contents.lines()
        .map(|line| {
            let mut chars = line.chars();
            let first = chars.next().unwrap();
            let second = chars.last().unwrap();
            (first, second)
        })
        .collect::<Vec<(char, char)>>();
    input
}


fn play_rules_1(opponents_hand: char, my_hand: char) -> usize{
    // A,X: Rock, B,Y: Paper, C,Z: Scissors
    let hand_score: usize = match my_hand{
        'X' => 1,
        'Y' => 2,
        'Z' => 3,
        _ => panic!(),
    };
    let match_result: usize = match (opponents_hand, my_hand) {
        ('A', 'X') => 3,
        ('A', 'Y') => 6,
        ('A', 'Z') => 0,
        ('B', 'X') => 0,
        ('B', 'Y') => 3,
        ('B', 'Z') => 6,
        ('C', 'X') => 6,
        ('C', 'Y') => 0,
        ('C', 'Z') => 3,
        _ => panic!(),
    };
    hand_score + match_result
}

fn play_rules_2(opponents_hand: char, desired_outcome: char) -> usize{
    let win:usize = 6;
    let draw:usize = 3;
    let lose:usize = 0;

    let rock:usize = 1;
    let paper:usize = 2;
    let scissors:usize = 3;

    match (opponents_hand, desired_outcome){
        ('A', 'X') => scissors + lose, // A is rock, X is lose, so scissors + lose
        ('A', 'Y') => rock + draw, // A is rock, Y is draw, so rock + draw
        ('A', 'Z') => paper + win, // A is rock, Z is win, so paper + win
        ('B', 'X') => rock + lose, // B is paper, X is lose, so rock + lose
        ('B', 'Y') => paper + draw, // B is paper, Y is draw, so paper + draw
        ('B', 'Z') => scissors + win, // B is paper, Z is win, so scissors + win
        ('C', 'X') => paper + lose, // C is scissors, X is lose, so paper + lose
        ('C', 'Y') => scissors + draw, // C is scissors, Y is draw, so scissors + draw
        ('C', 'Z') => rock + win, // C is scissors, Z is win, so rock + win
        _ => panic!(),
    }
}

pub fn run_part_1(game_input: Vec<(char, char)>) -> usize {
    game_input.iter()
        .map(|(opponents_hand, my_hand)| play_rules_1(*opponents_hand, *my_hand))
        .sum()
}

pub fn run_part_2(game_input: Vec<(char, char)>) -> usize {
    game_input.iter()
        .map(|(opponents_hand, desired_outcome)| play_rules_2(*opponents_hand, *desired_outcome))
        .sum()
}


#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test_read_input_numbers() {
        let input= read_input_from_file(
            "src/days/day2/input_files/test_file.txt"
        );

        assert_eq!(vec![('A','Y'), ('B', 'X'), ('C', 'Z')], input);
    }

    #[test]
    fn test_run_part_1(){
        assert_eq!(run_part_1(vec![('A','Y'), ('B', 'X'), ('C', 'Z')]), 15);
    }

    #[test]
    fn test_run_part_2(){
        assert_eq!(run_part_2(vec![('A','Y'), ('B', 'X'), ('C', 'Z')]), 12);
    }
}

