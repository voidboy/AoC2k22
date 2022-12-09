use std::collections::HashSet;
use std::fs;

/*

Part. I: 6339

.....    .....    .....
.TH.. -> .T.H. -> ..TH.
.....    .....    .....

...    ...    ...
.T.    .T.    ...
.H. -> ... -> .T.
...    .H.    .H.
...    ...    ...

.....    .....    .....
.....    ..H..    ..H..
..H.. -> ..... -> ..T..
.T...    .T...    .....
.....    .....    .....

.....    .....    .....
.....    .....    .....
..H.. -> ...H. -> ..TH.
.T...    .T...    .....
.....    .....    .....

*/

fn fix_up_down(knots: &mut Vec<(isize, isize)>, i: usize, visit: &mut HashSet<(isize, isize)>) {
    if knots[i - 1].1.abs_diff(knots[i].1) == 2 {
        if knots[i - 1].1 > knots[i].1 {
            knots[i].1 += 1;
        } else {
            knots[i].1 -= 1;
        }
        knots[i].0 = knots[i - 1].0;
        if i == knots.len() - 1 {
            visit.insert(knots[i]);
        }
    }
}

fn fix_left_right(knots: &mut Vec<(isize, isize)>, i: usize, visit: &mut HashSet<(isize, isize)>) {
    if knots[i - 1].0.abs_diff(knots[i].0) == 2 {
        if knots[i - 1].0 > knots[i].0 {
            knots[i].0 += 1;
        } else {
            knots[i].0 -= 1;
        }
        knots[i].1 = knots[i - 1].1;
        if i == knots.len() - 1 {
            visit.insert(knots[i]);
        }
    }
}

fn main() {
    let contents = fs::read_to_string("input").expect("Cannot read the input file");

    let mut knots: Vec<(isize, isize)> = vec![(0, 0); 2];
    let mut visit: HashSet<(isize, isize)> = HashSet::new();
    for line in contents.lines() {
        let mut motion = line.split_whitespace();
        let (direction, length) = (
            motion.next().unwrap(),
            motion.next().unwrap().parse::<isize>().unwrap(),
        );
        match direction {
            "U" => {
                for _ in 0..length {
                    knots[0].1 += 1;
                    for i in 1..knots.len() {
                        fix_up_down(&mut knots, i, &mut visit);
                    }
                }
            }
            "D" => {
                for _ in 0..length {
                    knots[0].1 -= 1;
                    for i in 1..knots.len() {
                        fix_up_down(&mut knots, i, &mut visit);
                    }
                }
            }
            "R" => {
                for _ in 0..length {
                    knots[0].0 += 1;
                    for i in 1..knots.len() {
                        fix_left_right(&mut knots, i, &mut visit);
                    }
                }
            }
            "L" => {
                for _ in 0..length {
                    knots[0].0 -= 1;
                    for i in 1..knots.len() {
                        fix_left_right(&mut knots, i, &mut visit);
                    }
                }
            }
            _ => {
                panic!("Invalid motion")
            }
        }
    }
    println!("Part. I: {}", visit.len());
}
