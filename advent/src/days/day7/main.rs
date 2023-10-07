use std::borrow::BorrowMut;
use std::fs;
use std::{cell::RefCell, rc::Rc};

#[derive(Debug, Clone)]
struct Directory{
    name: String,
    files: Vec<File>,
    directories: Vec<DirectoryRef>,
    parent_position: Option<usize>,
}

impl Directory {
    fn find_directory(&self, target_name: &str) -> Option<DirectoryRef>{
        for directory in &self.directories {
            if directory.borrow().name == target_name {
                return Some(directory.clone());
            }
        }
        None
    }

    fn is_directory_exist(&self, target_name: &str) -> bool {
        for directory in &self.directories {
            if directory.borrow().name == target_name {
                return true;
            }
        }
        false
    }

    fn is_file_exists(&self, target_name: &str) -> bool {
        for file in &self.files {
            if file.name == target_name {
                return true;
            }
        }
        false
    }
}

type DirectoryRef = Rc<RefCell<Directory>>;

#[derive(Debug, Clone)]
struct File{
    name: String,
    size: usize,
}


pub fn read_input_from_file(path: &str) -> Vec<String>{
    println!("Reading input from file: {}", path);
    let contents = fs::read_to_string(path)
        .expect("Something went wrong with a file")
        .lines()
        .map(|line| line.to_string())
        .collect::<Vec<String>>();
    contents
}


fn create_directory_tree(input: Vec<String>) -> Vec<DirectoryRef>{
    let mut root = Directory{
        name: "/".to_string(),
        files: Vec::new(),
        directories: Vec::new(),
        parent_position: None,
    };
    let mut directory_tree: Vec<DirectoryRef> = Vec::new();
    directory_tree.push(Rc::new(RefCell::new(root.clone())));
    let mut current_directory_position: usize = 0;

    for line in input.iter().skip(1){
        println!("Line: {}", line);
        if line.starts_with('$'){
            let command = line.split_whitespace().collect::<Vec<&str>>();
            // println!("{:?}", command);
            if command[1] == "cd"{
                if command[2] == ".."{  // eg $ cd ..
                    // If the command is to change directory to the parent directory
                    // we change the current directory to the parent directory
                    // if the current directory is the root directory, we do nothing
                    // current_directory = &directory_tree[current_directory.borrow().parent_position.unwrap()]
                    current_directory_position = directory_tree[current_directory_position]
                        .borrow().parent_position.unwrap();
                }
                else{ // eg $ cd a
                    // If the command is to change directory to a new directory
                    // we create a new directory and make it's parent the current directory
                    // the we change the current directory to this new directory created
                    let current_directory = directory_tree[current_directory_position]
                        .borrow()
                        .find_directory(command[2]);
                    if current_directory.is_some(){
                        current_directory_position = directory_tree.iter().position(|directory| directory.borrow().name == command[2]).unwrap();
                    }
                }
            }
            else if command[1] == "ls"{
                continue
            }
        }
        else {
            let command = line.split_whitespace().collect::<Vec<&str>>();
            if command[0] == "dir"{
                let current_directory = directory_tree[current_directory_position].borrow_mut();
                if !current_directory.is_directory_exist(command[1]){
                    let new_directory = Directory{
                        name: command[1].to_string(),
                        files: Vec::new(),
                        directories: Vec::new(),
                        parent_position: Some(current_directory_position.clone()),
                    };
                    directory_tree[current_directory_position].borrow_mut().directories.push(Rc::new(RefCell::new(new_directory.clone())));
                    directory_tree.borrow_mut().push(Rc::new(RefCell::new(new_directory.clone())));
                }
            }
            else {
                let current_directory = directory_tree[current_directory_position].borrow_mut();
                if !current_directory.is_file_exists(command[1]){
                    let new_file = File{
                        name: command[1].to_string(),
                        size: command[0].parse::<usize>().unwrap(),
                    };
                    directory_tree[current_directory_position].borrow_mut().files.push(new_file.clone());
                }
            }
        }
    }
    directory_tree
}

// pub fn run_part_1(input: String) -> usize{
//     0
// }

// pub fn run_part_2(input: String) -> usize{
//     0
// }


#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test_read_input_from_file(){
        let results = read_input_from_file("src/days/day7/input_files/test_file.txt");
        assert_eq!(results, vec!["$ cd /", "$ ls", "dir a", "14848514 b.txt", "8504156 c.dat", "dir d", "$ cd a", "$ ls", "dir e", "29116 f", "2557 g", "62596 h.lst", "$ cd e", "$ ls", "584 i", "$ cd ..", "$ cd ..", "$ cd d", "$ ls", "4060174 j", "8033020 d.log", "5626152 d.ext", "7214296 k"])
    }

    #[test]
    fn test_create_directory_tree(){
        let input = vec!["$ cd /", "$ ls", "dir a", "14848514 b.txt", "8504156 c.dat", "dir d", "$ cd a", "$ ls", "dir e", "29116 f", "2557 g", "62596 h.lst", "$ cd e", "$ ls", "584 i", "$ cd ..", "$ cd ..", "$ cd d", "$ ls", "4060174 j", "8033020 d.log", "5626152 d.ext", "7214296 k"];
        let results = create_directory_tree(input.iter().map(|line| line.to_string()).collect::<Vec<String>>());
        // println!("{:?}", results);
        for result in results.iter(){
            println!("{:?}", result.borrow());
        }
        assert_eq!(results[0].borrow().name, "/");
        assert_eq!(results[0].borrow().files.len(), 2);
    }

    // #[test]
    // fn test_run_part_1(){
    //     assert_eq!(0,0)
    // }

    // #[test]
    // fn test_run_part_2(){
    //     assert_eq!(0,0)
    // }

}
