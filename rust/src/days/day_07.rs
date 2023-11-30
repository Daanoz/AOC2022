use std::{
    cell::RefCell,
    cmp::min,
    rc::{Rc, Weak},
};

use async_trait::async_trait;
use common::{Answer, Solution};

#[derive(Default)]
pub struct Puzzle {}

struct Dir {
    name: String,
    files: Vec<File>,
    parent: Option<Weak<RefCell<Dir>>>,
    dirs: Vec<Rc<RefCell<Dir>>>,
}
impl Dir {
    fn root() -> Self {
        Self::from_name("")
    }
    fn from_name(name: &str) -> Self {
        Self {
            name: String::from(name),
            parent: None,
            files: vec![],
            dirs: vec![],
        }
    }
    fn get_dir_size(&self) -> u64 {
        let files_sum = self.files.iter().fold(0, |acc, file| acc + file.size);
        let dirs_sum = self
            .dirs
            .iter()
            .fold(0, |acc, dir| acc + dir.borrow().get_dir_size());
        files_sum + dirs_sum
    }
}

struct File {
    size: u64,
}

fn add_dir_to_parent(parent: Rc<RefCell<Dir>>, mut child: Dir) {
    child.parent = Some(Rc::downgrade(&parent));
    let child_ref = Rc::new(RefCell::new(child));
    (*parent).borrow_mut().dirs.push(child_ref);
}

fn switch_directory(
    cwd: Rc<RefCell<Dir>>,
    root: Rc<RefCell<Dir>>,
    new_dir: &str,
) -> Rc<RefCell<Dir>> {
    {
        let current = (*cwd).borrow();
        match new_dir {
            ".." => {
                let new_parent = current.parent.clone();
                if let Some(new_parent) = new_parent {
                    return new_parent.upgrade().unwrap();
                }
            }
            "/" => return root,
            _ => {
                let new_parent = current.dirs.iter().find(|dir| dir.borrow().name == new_dir);
                if let Some(new_parent) = new_parent {
                    return new_parent.clone();
                }
            }
        }
    }
    cwd
}

fn read_dir_listing(cwd: Rc<RefCell<Dir>>, cmd: &str) {
    if cmd.starts_with("dir ") {
        let dir_name = cmd
            .split_once(' ')
            .expect("Dir should consist out of 2 parts")
            .1;
        let child_dir = Dir::from_name(dir_name);
        add_dir_to_parent(cwd, child_dir);
    } else {
        let (file_size, _) = cmd
            .split_once(' ')
            .expect("File should consist out of 2 bits");
        {
            (*cwd).borrow_mut().files.push(File {
                size: file_size.parse().expect("Invalid file_size"),
            });
        }
    }
}

fn read_terminal_output(root: Rc<RefCell<Dir>>, input: String) {
    let mut cwd = root.clone();
    input.split('\n').for_each(|cmd| {
        if cmd.starts_with('$') {
            let cmd = cmd
                .split_once(' ')
                .expect("Command should consist at least out of 2 parts")
                .1;
            if cmd.starts_with("cd") {
                let new_dir = cmd.split_once(' ').expect("Cd command missing directory").1;
                cwd = switch_directory(cwd.clone(), root.clone(), new_dir);
            }
        } else {
            read_dir_listing(cwd.clone(), cmd);
        }
    });
}

fn sum_all_dirs_with_size_of(size: u64, dir: Rc<RefCell<Dir>>) -> u64 {
    let mut sum = 0;
    let dir_size = dir.borrow().get_dir_size();
    if dir_size <= size {
        sum += dir_size
    }
    sum += dir.borrow().dirs.iter().fold(0, |acc, sub_dir| {
        acc + sum_all_dirs_with_size_of(size, sub_dir.clone())
    });
    sum
}

fn find_dir_with_closest_size_to(size: u64, dir: Rc<RefCell<Dir>>) -> u64 {
    let dir_size = dir.borrow().get_dir_size();
    if dir_size < size {
        return u64::MAX;
    }
    let smallest_size = dir
        .borrow()
        .dirs
        .iter()
        .map(|subdir| find_dir_with_closest_size_to(size, subdir.clone()))
        .min()
        .unwrap_or(u64::MAX);
    min(dir_size, smallest_size)
}

#[async_trait]
impl Solution for Puzzle {
    async fn solve_a(&mut self, input: String) -> Result<Answer, String> {
        let root = Rc::new(RefCell::new(Dir::root()));
        read_terminal_output(root.clone(), input);
        Ok(Answer::from(sum_all_dirs_with_size_of(100_000, root)))
    }

    async fn solve_b(&mut self, input: String) -> Result<Answer, String> {
        let root = Rc::new(RefCell::new(Dir::root()));
        read_terminal_output(root.clone(), input);
        let space_free = 70_000_000 - root.borrow().get_dir_size();
        let space_required = 30_000_000 - space_free;
        Ok(Answer::from(find_dir_with_closest_size_to(
            space_required,
            root,
        )))
    }
}

#[cfg(test)]
mod tests {
    use super::Puzzle;
    use common::{Answer, Solution};

    const TEST_INPUT: &str = "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k";

    #[tokio::test]
    async fn part_a() {
        let mut puzzle = Puzzle::default();
        assert_eq!(
            puzzle.solve_a(String::from(TEST_INPUT.trim())).await,
            Ok(Answer::from(95437))
        )
    }

    #[tokio::test]
    async fn part_b() {
        let mut puzzle = Puzzle::default();
        assert_eq!(
            puzzle.solve_b(String::from(TEST_INPUT.trim())).await,
            Ok(Answer::from(24933642))
        )
    }
}
