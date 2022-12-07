use std::collections::HashMap;
use std::fs;
use std::cmp;

fn main() {
    let contents = fs::read_to_string("input").expect("Cannot read the file");

    let mut cwd: Vec<&str> = vec![];
    let mut tree: HashMap<String, u32> = HashMap::new();
    contents.lines().for_each(|line| {
        let mut command = line.split(' ');
        match command.next().unwrap() {
            // $ cd /
            // $ ls
            "$" => match command.next().unwrap() {
                "cd" => match command.next().unwrap() {
                    ".." => {
                        cwd.pop();
                    }
                    path => {
                        cwd.push(path);
                        tree.insert(cwd.join(""), 0);
                    }
                },
                _ => {}
            },
            // dir hzjzlwv
            "dir" => {}
            // 69990 hdf.fjn
            n => {
                let file_size = n.parse::<u32>().unwrap();
                for level in (0..=cwd.len()).rev() {
                    let key = &cwd[..level].join("");
                    if let Some(size) = tree.get_mut(key) {
                        *size += file_size;
                    }
                }
            }
        }
    });
    let mut part1: u32 = 0;
    for size in tree.values() {
        if size <= &100000 {
            part1 += size;
        }
    }
    println!("part1: {part1}");
    let to_free = 30000000 - (70000000 - tree.get("/").unwrap());
    let mut part2 = u32::MAX;
    for size in tree.values() {
        if *size >= to_free {
            part2 = cmp::min(*size, part2);
        }
    }
    println!("part2: {part2}");
}
