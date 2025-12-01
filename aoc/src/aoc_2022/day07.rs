use std::{collections::HashMap, str::FromStr};

use challenges_input::Input;
use itertools::Itertools;

pub const TRIM: bool = true;

pub fn run(input: Input) -> String {
    aoc_helpers::run(input, part_a, part_b)
}

// struct Directory {
//     files: Vec<File>,
//     directories: Vec<Directory>,
//     path: Vec<String>,
// }
//
// struct File {
//     name: String,
//     size: u64,
// }
//
enum LsEntry {
    Dir { name: String },
    File { name: String, size: u64 },
}
impl FromStr for LsEntry {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.split_whitespace().collect_vec();
        if let Ok(v) = u64::from_str(parts[0]) {
            Ok(Self::File {
                name: parts[1].to_string(),
                size: v,
            })
        } else if parts[0] == "dir" {
            Ok(Self::Dir {
                name: parts[1].to_string(),
            })
        } else {
            Err(())
        }
    }
}

enum Command {
    Cd(String),
    Ls,
}
impl FromStr for Command {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let Some(s) = s.strip_prefix("$ ") else {
            return Err(());
        };
        let args = s.split_whitespace().collect_vec();
        match args[0] {
            "ls" => Ok(Self::Ls),
            "cd" => Ok(Self::Cd(args[1].to_string())),
            _ => Err(()),
        }
    }
}

fn parse_path(path: &str, cwd: &[String]) -> Vec<String> {
    if path == ".." {
        let mut dest = cwd.to_vec();
        dest.pop();
        return dest;
    }
    let (mut proto_path, path) = match path.strip_prefix("/") {
        Some(v) => (vec![], v),
        None => (cwd.to_vec(), path),
    };
    let parts = path.split("/");
    for p in parts {
        proto_path.push(p.to_string());
    }
    proto_path
}

fn part_a(mut input: Input) -> u64 {
    let mut directories: HashMap<Vec<String>, u64> = HashMap::new();
    let mut cwd = vec![];
    while let Some(cmd) = input.next::<Command>() {
        match cmd {
            Command::Cd(dir) => cwd = parse_path(&dir, &cwd),
            Command::Ls => {
                while let Some(ent) = input.next::<LsEntry>() {
                    match ent {
                        LsEntry::Dir { .. } => (),
                        LsEntry::File { size, name } => {
                            let path = parse_path(&name, &cwd);
                            for i in 0..path.len() {
                                let dir = path[..=i].to_vec();
                                *directories.entry(dir).or_insert(0) += size;
                            }
                        }
                    }
                }
            }
        }
    }
    directories.values().filter(|i| **i <= 100000).sum()
    // let fs = build_fs(input);
    // let mut directory_sizes = vec![];
    // get_dir_size(&mut directory_sizes, &fs);
    // directory_sizes.into_iter().filter(|i| *i <= 100000).sum()
}

fn part_b(input: Input) -> u64 {
    // let fs = build_fs(input);
    // let mut directory_sizes = vec![];
    // get_dir_size(&mut directory_sizes, &fs);
    // let max = *directory_sizes.iter().max().unwrap();
    // directory_sizes
    //     .into_iter()
    //     .filter(|i| *i >= 30000000 + max - 70000000)
    //     .min()
    //     .unwrap()
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc_helpers::mk_test_input;

    #[test]
    fn test_part_a() {
        let input = mk_test_input!(
            "$ cd /
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
7214296 k"
        );
        assert_eq!(part_a(input), 95437);
    }

    #[test]
    fn test_part_b() {
        let input = mk_test_input!(
            "$ cd /
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
7214296 k"
        );
        assert_eq!(part_b(input), 24933642);
    }
}
