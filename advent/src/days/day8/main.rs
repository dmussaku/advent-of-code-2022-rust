use std::fs;

pub fn read_input_from_file(path: &str) -> Vec<Vec<u8>>{
    let result: Vec<Vec<u8>> = fs::read_to_string(path)
        .expect("Something went wrong with a file")
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as u8)
                .collect::<Vec<_>>()})
        .collect();
    result
}

fn is_tree_visible(map: &Vec<Vec<u8>>, row: usize, col: usize) -> bool{
    if row == 0 || col == 0 || row == map.len() - 1 || col == map[0].len() - 1 {
        return true;
    }
    let mut is_visible_from_top = true;
    let mut is_visible_from_bottom = true;
    let mut is_visible_from_left = true;
    let mut is_visible_from_right = true;
    
    //first iterate to the top
    for i in 0..row {
        if map[i][col] >= map[row][col] {
            is_visible_from_top = false;
            break;
        }
    }
    //next iterate to the bottom
    for i in row+1..map.len() {
        if map[i][col] >= map[row][col] {
            is_visible_from_bottom = false;
            break;
        }
    }
    //next iterate to the left
    for i in 0..col {
        if map[row][i] >= map[row][col] {
            is_visible_from_left = false;
            break;
        }
    }

    //next iterate to the right
    for i in col+1..map[row].len() {
        if map[row][i] >= map[row][col] {
            is_visible_from_right = false;
            break;
        }
    }

    is_visible_from_top || is_visible_from_bottom || is_visible_from_left || is_visible_from_right
}


fn scenic_score(map: &Vec<Vec<u8>>, row: usize, col: usize) -> usize{
    if row == 0 || col == 0 || row == map.len() - 1 || col == map[0].len() - 1 {
        return 0;
    }
    let mut visible_trees_from_top: usize = 0;
    let mut visible_trees_from_bottom: usize = 0;
    let mut visible_trees_from_left: usize = 0;
    let mut visible_trees_from_right: usize = 0;

    //first iterate to the top
    for i in (0..row).rev() {
        if map[i][col] >= map[row][col] {
            visible_trees_from_top += 1;
            break;
        }
        visible_trees_from_top += 1;
    }
    //next iterate to the bottom
    for i in row+1..map.len() {
        if map[i][col] >= map[row][col] {
            visible_trees_from_bottom += 1;
            break;
        }
        visible_trees_from_bottom += 1;
    }

    //next iterate to the left
    for i in (0..col).rev() {
        if map[row][i] >= map[row][col] {
            visible_trees_from_left += 1;
            break;
        }
        visible_trees_from_left += 1;
    }

    //next iterate to the right
    for i in col+1..map[row].len() {
        if map[row][i] >= map[row][col] {
            visible_trees_from_right += 1;
            break;
        }
        visible_trees_from_right += 1;
    }
    // println!("Visible trees for position [{}, {}] from top: {}, left: {},  right: {}, down: {}", row, col, visible_trees_from_top, visible_trees_from_left, visible_trees_from_right, visible_trees_from_bottom);
    visible_trees_from_top * visible_trees_from_bottom * visible_trees_from_left * visible_trees_from_right
}

pub fn run_part_1(input: &Vec<Vec<u8>>) -> usize{
    let mut result = 0;
    let mut resulting_vector: Vec<Vec<u8>> = vec![vec![0; input[0].len()]; input.len()];
    for row in 0..input.len() {
        for col in 0..input[0].len() {
            if is_tree_visible(&input, row, col) {
                result += 1;
                resulting_vector[row][col] = 1;
            }
        }
    }
    // for row in 0..input.len() {
    //     println!("Resulting vector: {:?}", resulting_vector[row]);
    // }
    result
}

pub fn run_part_2(input: &Vec<Vec<u8>>) -> usize {
    let mut max = 0;
    for row in 0..input.len() {
        for col in 0..input[0].len() {
            let score = scenic_score(&input, row, col);
            if score > max {
                max = score;
            }
        }
    }
    max
}


#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test_read_input_from_file() {
        let input= read_input_from_file(
            "src/days/day8/input_files/test_file.txt"
        );
        assert_eq!(input, vec![
            vec![3,0,3,7,3],
            vec![2,5,5,1,2],
            vec![6,5,3,3,2],
            vec![3,3,5,4,9],
            vec![3,5,3,9,0],
        ]);
    }

    #[test]
    fn test_is_tree_visible(){
        let input= read_input_from_file(
            "src/days/day8/input_files/test_file.txt"
        );

        assert_eq!(is_tree_visible(&input, 0, 0), true);
        assert_eq!(is_tree_visible(&input, 4, 0), true);
        assert_eq!(is_tree_visible(&input, 0, 4), true);
        assert_eq!(is_tree_visible(&input, 4, 4), true);

        assert_eq!(is_tree_visible(&input, 1, 1), true);
        assert_eq!(is_tree_visible(&input, 1, 2), true);
        assert_eq!(is_tree_visible(&input, 1, 3), false);
        assert_eq!(is_tree_visible(&input, 3, 3), false);
        assert_eq!(is_tree_visible(&input, 2, 2), false);
        assert_eq!(is_tree_visible(&input, 3, 1), false);
    }

    #[test]
    fn test_scenic_score(){
        let input= read_input_from_file(
            "src/days/day8/input_files/test_file.txt"
        );

        assert_eq!(scenic_score(&input, 0, 0), 0);
        assert_eq!(scenic_score(&input, 4, 0), 0);
        assert_eq!(scenic_score(&input, 0, 4), 0);
        assert_eq!(scenic_score(&input, 4, 4), 0);

        assert_eq!(scenic_score(&input, 1, 2), 4);
        assert_eq!(scenic_score(&input, 3, 2), 8);
        assert_eq!(scenic_score(&input, 2, 1), 6);
    }

    #[test]
    fn test_run_part_1(){
        let input= read_input_from_file(
            "src/days/day8/input_files/test_file.txt"
        );
        assert_eq!(run_part_1(&input), 21);
    }

    #[test]
    fn test_run_part_2(){
        let input= read_input_from_file(
            "src/days/day8/input_files/test_file.txt"
        );
        assert_eq!(run_part_2(&input), 8);
    }
}
