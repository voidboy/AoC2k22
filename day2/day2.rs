use std::fs;

/*
   A|X -> Rock 
   B|Y -> Paper
   C|Z -> Scissors

   lose -> 0 points
   draw -> 3 points
   win  -> 6 points
 */

enum Handsign {
    Rock     = 1,
    Paper    = 2,
    Scissors = 3
}

fn pick_handsign(letter: &str) -> Handsign {
    match letter {
        "A" | "X" => Handsign::Rock,
        "B" | "Y" => Handsign::Paper,
        "C" | "Z" => Handsign::Scissors,
        _ => panic!("unvalid handsign"),
    }
}

fn handgame(he: &str, my: &str) -> u32 {
    let p1 = pick_handsign(he);
    let p2 = pick_handsign(my);
    match (p1, p2) {
        (Handsign::Rock, Handsign::Rock)         => 3 + 1,
        (Handsign::Rock, Handsign::Paper)        => 6 + 2,
        (Handsign::Rock, Handsign::Scissors)     => 0 + 3,

        (Handsign::Paper, Handsign::Rock)        => 0 + 1,
        (Handsign::Paper, Handsign::Paper)       => 3 + 2,
        (Handsign::Paper, Handsign::Scissors)    => 6 + 3,

        (Handsign::Scissors, Handsign::Rock)     => 6 + 1,
        (Handsign::Scissors, Handsign::Paper)    => 0 + 2,
        (Handsign::Scissors, Handsign::Scissors) => 3 + 3,
    }
}

fn main() {
    let contents = fs::read_to_string("input")
        .expect("Cannot read the file");
    let mut score = 0;

    for line in contents.lines() {
        let mut mov = line.split_whitespace();
        let (opponent, should) = (
            mov.next().unwrap(),
            mov.next().unwrap(),
        );
        score += handgame(opponent, should);
    }
    println!("Your score is {}", score);
}
