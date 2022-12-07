use std::{collections::HashMap};

struct File {
    name: String,
    size: usize,
}

struct Dir {
    files: Vec<File>
}

fn main() {
    let lines = include_str!("../../../input07").lines().collect::<Vec<&str>>();

    let tree = run(&lines);

    println!("Part1: {}", tree.keys()
        .map(|p| size_of_dir(p, &tree))
        .filter(|s| s <= &100000)
        .sum::<usize>());

    // P2
    let used_space = size_of_dir(&"".to_string(), &tree);
    let unused = 70000000 - used_space;
    let needed = 30000000 - unused;
    let mut path_sizes = tree.keys()
        .map(|path| (size_of_dir(path, &tree), path))
        .collect::<Vec<(usize, &String)>>();
    path_sizes.sort_by_key(|p| p.0);
    let big_enough_small_dir = path_sizes.iter().find(|p| p.0 >= needed).expect("Must have an answer");
    println!("Part2: Removing {} frees up {} and needed {}", big_enough_small_dir.1, big_enough_small_dir.0, needed);
}

fn size_of_dir(path: &String, tree: &HashMap<String, Dir>) -> usize {
    tree.iter()
        .filter(|(p, _)| p.starts_with(path))
        .map(|(_, d)| d.files.iter().map(|f| f.size).sum::<usize>())
        .sum()
}

fn run(instrs: &Vec<&str>) -> HashMap<String, Dir> {
    let mut res = HashMap::new();

    let mut current_path: Vec<String> = Vec::new();

    for i in 0..instrs.len() {
        let instr = instrs[i];
        if instr.starts_with("$ cd ") {
            let path = instr.split(" ").nth(2).expect("cd must have argument");
            match path {
                "/" => current_path.clear(),
                ".." => { current_path.pop(); },
                other => current_path.push(other.to_string()),
            }

            let cur_path = current_path.join("/");
            if !res.contains_key(&cur_path) {
                res.insert(cur_path, Dir { files: Vec::new() });    // Insert placeholder path
            }
        } else if instr == "$ ls" {
            let cur_path = current_path.join("/");
            let mut dir = Dir {
                files: Vec::new(),
            };

            // Parse cmd output
            for u in (i+1)..instrs.len() {
                let cur_line = instrs[u];
                if cur_line.starts_with("$") {
                    break;
                } else if cur_line.starts_with("dir") {
                    // Ignore directories
                } else {
                    // Should be file
                    let (size, name) = cur_line.split_once(" ").expect("should be file line");
                    dir.files.push(File {
                        name: name.to_string(),
                        size: size.parse().expect("Size must be parseable")
                    });
                }
            }

            res.insert(cur_path, dir);
        } else if instr.starts_with("$"){
            panic!("Unhandled instr: {}", instr);
        }
    }

    res
}