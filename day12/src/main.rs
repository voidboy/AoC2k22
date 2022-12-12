use std::collections::HashMap;
use std::fs;

fn get_map_info(map: &mut[Vec<char>]) -> Option<((usize, usize), (usize, usize))> {
    let mut start = None;
    let mut end = None;
    'main: for (y, line) in map.iter_mut().enumerate() {
        for (x, character) in line.iter_mut().enumerate() {
            if *character == 'S' {
                *character = 'a';
                start = Some((x, y));
                if end.is_some() {
                    break 'main;
                }
            }
            if *character == 'E' {
                *character = 'z';
                end = Some((x, y));
                if start.is_some() {
                    break 'main;
                }
            }
        }
    }
    if let (Some(s), Some(e)) = (start, end) {
        Some((s, e))
    } else {
        None
    }
}

fn neighbors(map: &Vec<Vec<char>>, me: (usize, usize)) -> Vec<(usize, usize)> {
    let mut found = vec![];
    let me_character = map[me.1][me.0];
    // left
    if me.0 > 0 && map[me.1][me.0 - 1] as u8 <= me_character as u8 + 1 {
        found.push((me.0 - 1, me.1));
    }
    // right
    if me.0 < map[0].len() - 1 && map[me.1][me.0 + 1] as u8 <= me_character as u8 + 1 {
        found.push((me.0 + 1, me.1));
    }
    // top
    if me.1 > 0 && map[me.1 - 1][me.0] as u8 <= me_character as u8 + 1 {
        found.push((me.0, me.1 - 1));
    }
    // bot
    if me.1 < map.len() - 1 && map[me.1 + 1][me.0] as u8 <= me_character as u8 + 1 {
        found.push((me.0, me.1 + 1));
    }
    found
}

fn main() {
    let contents = fs::read_to_string("input").expect("Cannot read the file");

    let mut map = contents
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    let (start, end) = get_map_info(&mut map).unwrap();
    println!("Start @ {:?}", start);
    println!("  End @ {:?}", end);
    let mut frontier = Vec::<(usize, usize)>::new();
    frontier.push(start);
    let mut cames_from = HashMap::<(usize, usize), Option<(usize, usize)>>::new();
    let mut index = 0;
    cames_from.insert(start, None);
    while !frontier.is_empty() {
        index += 1;
        let current = frontier.pop().unwrap();
        if current == end {
            break ;
        }
        let neighbors = neighbors(&map, current);
        for neighbor in neighbors {
            if cames_from.get(&neighbor).is_none() {
                frontier.insert(0, neighbor);
                cames_from.insert(neighbor, Some(current));
            }
        }
        if index > 1_000_000 { panic!("thanks cod3monks"); }
    }
    let mut from = cames_from.get(&end);
    let mut count = 0;
    while from.unwrap().is_some() {
        from = cames_from.get(&from.unwrap().unwrap());
        count += 1;
    }
    println!("Part I. : {}", count);
}
