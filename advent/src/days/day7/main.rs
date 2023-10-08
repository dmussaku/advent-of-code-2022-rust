use std::borrow::BorrowMut;
use std::fs;
use std::{cell::RefCell, rc::Rc};

#[derive(Debug, Clone)]
struct Directory{
    name: String,
    files: Vec<File>,
    directories: Vec<DirectoryRef>,
    parent: Option<DirectoryRef>,
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


fn create_directory_tree(input: Vec<String>) -> Directory{
    let mut root = Directory{
        name: "/".to_string(),
        files: Vec::new(),
        directories: Vec::new(),
        parent: None,
    };
    let mut current_directory = Rc::new(RefCell::new(root.clone()));

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
                    current_directory = current_directory.borrow().parent.clone().unwrap();
                }
                else{ // eg $ cd a
                    // If the command is to change directory to a new directory
                    // we create a new directory and make it's parent the current directory
                    // the we change the current directory to this new directory created
                    current_directory = current_directory.borrow().find_directory(command[2]).unwrap();
                }
            }
            else if command[1] == "ls"{
                continue
            }
        }
        else {
            let command = line.split_whitespace().collect::<Vec<&str>>();
            if command[0] == "dir"{
                let mut current_directory = current_directory.borrow();
                if !current_directory.is_directory_exist(command[1]){
                    let new_directory = Directory{
                        name: command[1].to_string(),
                        files: Vec::new(),
                        directories: Vec::new(),
                        parent: Some(Rc::new(RefCell::new(current_directory.clone()))),
                    };
                    // push new_directory into current_directory.directories
                    current_directory.borrow_mut().directories.push(Rc::new(RefCell::new(new_directory)));
                }
            }
            else {
                let current_directory = current_directory.borrow();
                if !current_directory.is_file_exists(command[1]){
                    let new_file = File{
                        name: command[1].to_string(),
                        size: command[0].parse::<usize>().unwrap(),
                    };
                    current_directory.files.push(new_file);
                }
            }
        }
    }
    root
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
        let root = create_directory_tree(input.iter().map(|line| line.to_string()).collect::<Vec<String>>());
        assert_eq!(root.name, "/");
        assert_eq!(root.files.len(), 2);
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
