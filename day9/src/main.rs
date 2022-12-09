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

type Pos = (isize, isize);

fn fix_knots(knots: &mut Vec<Pos>, visit: &mut HashSet<Pos>) {
    for i in 1..knots.len() {
        if knots[i - 1].1.abs_diff(knots[i].1) == 2 {
            if knots[i - 1].1 > knots[i].1 {
                // UP
                knots[i].1 += 1;
            } else {
                // DOWN
                knots[i].1 -= 1;
            }
            if knots[i - 1].0 > knots[i].0 {
                knots[i].0 += 1;
            }
            else if knots[i - 1].0 < knots[i].0 {
                knots[i].0 -= 1;
            }
        }
        if knots[i - 1].0.abs_diff(knots[i].0) == 2 {
            if knots[i - 1].0 > knots[i].0 {
                // RIGHT
                knots[i].0 += 1;
            } else {
                // LEFT
                knots[i].0 -= 1;
            }
            if knots[i - 1].1 > knots[i].1 {
                knots[i].1 += 1;
            }
            else if knots[i - 1].1 < knots[i].1 {
                knots[i].1 -= 1;
            }
        }
        if i == knots.len() - 1 {
            visit.insert(knots[i]);
        }
    }
}

fn ropes_simulation(nbr_of_knots: usize) -> usize {
    let contents = fs::read_to_string("input").expect("Cannot read the input file");

    let mut knots: Vec<Pos> = vec![(0, 0); nbr_of_knots];
    let mut visit: HashSet<Pos> = HashSet::from([(0, 0)]);
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
                    fix_knots(&mut knots, &mut visit);
                }
            }
            "D" => {
                for _ in 0..length {
                    knots[0].1 -= 1;
                    fix_knots(&mut knots, &mut visit);
                }
            }
            "R" => {
                for _ in 0..length {
                    knots[0].0 += 1;
                    fix_knots(&mut knots, &mut visit);
                }
            }
            "L" => {
                for _ in 0..length {
                    knots[0].0 -= 1;
                    fix_knots(&mut knots, &mut visit);
                }
            }
            _ => {
                panic!("Invalid motion")
            }
        }
    }
    visit.len()
}

fn main() {
    println!("Part. I:  {}", ropes_simulation(2));
    println!("Part. II: {}", ropes_simulation(10));
}
