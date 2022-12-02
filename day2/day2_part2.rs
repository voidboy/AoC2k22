use std::fs;

/* Thanks Mr. russriguez, https://github.com/RussellWaite */

fn compute_score(contents: &String, choice: bool) -> u32 {
    contents.lines().map(|line|{
        match line {
            "A X" => (4,3), 
            "A Y" => (8,4),
            "A Z" => (3,8),
            "B X" => (1,1),
            "B Y" => (5,5),
            "B Z" => (9,9),
            "C X" => (7,2),
            "C Y" => (2,6),
            "C Z" => (6,7),
            _ => panic!("invalid line !"),
        }
    }).map(|(a, b)| if !choice { a } else { b } ).sum()
}

fn main() {
    let contents = fs::read_to_string("input")
        .expect("Cannot read the file");
    println!("Part. 1 is {}", compute_score(&contents, false));
    println!("Part. 2 is {}", compute_score(&contents, true));
}
