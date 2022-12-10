use std::fs;

/*
    Part. I : 15220
*/

const STOP_CYCLE: isize = 220;

fn main() {
    let contents = fs::read_to_string("input").expect("Cannot read the file");
    let mut lines = contents.lines();
    let mut current_instruction: isize = 0;
    let mut accumulator: isize = 1;
    let mut sum: isize = 0;
    let mut step: isize = 20;

    for cycle in 1..=STOP_CYCLE {
        if cycle == step {
            step += 40;
            sum += cycle * accumulator;
        }
        if current_instruction == 0 {
            let mut next_instruction = lines.next().unwrap().split_whitespace();
            match next_instruction.next().unwrap() {
                "addx" => {
                    current_instruction =
                        next_instruction.next().unwrap().parse::<isize>().unwrap();
                    continue;
                }
                "noop" => {}
                _ => {
                    panic!("Invalid instruction");
                }
            }
        }
        if current_instruction != 0 {
            accumulator += current_instruction;
            current_instruction = 0;
        }
    }
    println!("Part. I : {sum}");
}
