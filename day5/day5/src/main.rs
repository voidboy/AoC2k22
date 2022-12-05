use std::fs;

fn part1() {
    let contents = fs::read_to_string("input")
        .expect("Cannot read the file");

    let input = contents.lines().take(8)
       .map(|line| {
           line.as_bytes().chunks(4)
               .map(|chunk| chunk[1])
               .collect::<Vec<u8>>()
       }).collect::<Vec<Vec<u8>>>();
    let mut stacks: [Vec<u8>; 9] = [vec![],vec![],vec![],
        vec![],vec![],vec![],vec![],vec![],vec![]];
    for line in input.iter().rev() {
        for (i, crates) in line.iter().enumerate() {
            if *crates != b' ' { stacks[i].push(*crates); }
        }
    }
    let commands = contents.lines().skip(10)
        .map(|line| {
            let mut command = line.split(' ');
            (command.nth(1).unwrap().parse::<u8>().unwrap(),
             command.nth(1).unwrap().parse::<u8>().unwrap(),
             command.nth(1).unwrap().parse::<u8>().unwrap())
        }).collect::<Vec<(u8, u8, u8)>>();
    commands.iter().for_each(|command| {
        for _ in 0..command.0 {
            let top = stacks[command.1 as usize - 1]
                .pop().unwrap();
            stacks[command.2 as usize - 1].push(top);
        }
    });
  // top of the vector is right most element !
  stacks.iter().for_each(|stack|
    print!("{}", &(stack[stack.len() - 1] as char))
  );
}

fn part2() {
    let contents = fs::read_to_string("input")
        .expect("Cannot read the file");

    let input = contents.lines().take(8)
       .map(|line| {
           line.as_bytes().chunks(4)
               .map(|chunk| chunk[1])
               .collect::<Vec<u8>>()
       }).collect::<Vec<Vec<u8>>>();
    let mut stacks: [Vec<u8>; 9] = [vec![],vec![],vec![],
        vec![],vec![],vec![],vec![],vec![],vec![]];
    for line in input.iter().rev() {
        for (i, crates) in line.iter().enumerate() {
            if *crates != b' ' { stacks[i].push(*crates); }
        }
    }
    let commands = contents.lines().skip(10)
        .map(|line| {
            let mut command = line.split(' ');
            (command.nth(1).unwrap().parse::<u8>().unwrap(),
             command.nth(1).unwrap().parse::<u8>().unwrap(),
             command.nth(1).unwrap().parse::<u8>().unwrap())
        }).collect::<Vec<(u8, u8, u8)>>();
    commands.iter().for_each(|command| {
        let from = &mut stacks[command.1 as usize - 1];
        let slice = from.drain(from.len() - command.0 as usize..)
            .collect::<Vec<u8>>();
        stacks[command.2 as usize - 1].extend_from_slice(&slice);
    });
  // top of the vector is right most element !
  stacks.iter().for_each(|stack|
    print!("{}", &(stack[stack.len() - 1] as char))
  );
}

fn main() {
    print!("Part1: ");
    part1();
    print!("\nPart2: ");
    part2();
    println!("");
}
