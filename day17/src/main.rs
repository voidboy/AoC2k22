use std::fs;

/* 

S###

.#.
###
S#.

..#
..#
S##

#
#
#
S

##
S#

The rocks fall in the order shown above

The tall, vertical chamber is exactly seven units wide.
Each rock appears so that its left edge is two units away
from the left wall and its bottom edge is three units
above the highest rock in the room (or the floor, if there isn't one).

*/

#[derive(Debug, Copy, Clone, PartialEq)]
enum RockShape {
    HBar,
    Cros,
    LLLL,
    VBar,
    Cube,
}

impl RockShape {
    fn next(self: &Self) -> Self {
        match self {
            RockShape::HBar => RockShape::Cros,
            RockShape::Cros => RockShape::LLLL,
            RockShape::LLLL => RockShape::VBar,
            RockShape::VBar => RockShape::Cube,
            RockShape::Cube => RockShape::HBar,
        }
    }

    fn which_width(s: RockShape) -> usize {
        match s {
            RockShape::HBar => 4,
            RockShape::Cros => 3,
            RockShape::LLLL => 3,
            RockShape::VBar => 1,
            RockShape::Cube => 2,
        }
    }

    fn which_height(s: RockShape) -> usize {
        match s {
            RockShape::HBar => 1,
            RockShape::Cros => 3,
            RockShape::LLLL => 3,
            RockShape::VBar => 4,
            RockShape::Cube => 2,
        }
    }

    fn which_marks(s: RockShape) -> usize {
        match s {
            RockShape::HBar => 4,
            RockShape::Cros => 5,
            RockShape::LLLL => 5,
            RockShape::VBar => 4,
            RockShape::Cube => 4,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Copy, Clone, PartialEq)]
struct Position {
    x: usize,
    y: usize,
}

#[derive(Debug, PartialEq)]
struct Rock {
    pos: Position,
    shape: RockShape,
    width: usize,
    height: usize,
    marks: Vec<Position>
}

impl Rock {

    fn new(yhighest_rock: usize, s: RockShape) -> Rock {
        let mut new_rock: Self = Self {
            pos: Position { 
                x: 3, 
                y: yhighest_rock + 4
            },
            shape: s,
            width: RockShape::which_width(s),
            height: RockShape::which_height(s),
            marks: Vec::with_capacity(RockShape::which_marks(s))
        };
        for _ in 0..RockShape::which_marks(s) {
            new_rock.marks.push(Position { x: 0, y: 0});
        }
        new_rock.update_marks();
        new_rock
    }

    fn update(self: &mut Self, d: &Direction) {
        match d {
            Direction::Up    => self.pos.y += 1,
            Direction::Down  => {
                if self.pos.y > 0 {
                    self.pos.y -= 1;
                    self.update_marks();
                }
            },
            Direction::Left  => {
                if self.pos.x > 1 {
                    self.pos.x -= 1;
                    self.update_marks();
                }
            }
            Direction::Right => {
                // are you sure about that ?
                if self.pos.x + (self.width - 1) < 7 {
                    self.pos.x += 1;
                    self.update_marks();
                }
            }
        }
    }

    fn update_marks(self: &mut Self) {
        match self.shape {
            RockShape::HBar => {
                self.marks[0] = Position { x: self.pos.x + 0, y: self.pos.y + 0};
                self.marks[1] = Position { x: self.pos.x + 1, y: self.pos.y + 0};
                self.marks[2] = Position { x: self.pos.x + 2, y: self.pos.y + 0};
                self.marks[3] = Position { x: self.pos.x + 3, y: self.pos.y + 0};
            }
            RockShape::Cros => {
                self.marks[0] = Position { x: self.pos.x + 1, y: self.pos.y + 0}; 
                self.marks[1] = Position { x: self.pos.x + 1, y: self.pos.y + 1};
                self.marks[2] = Position { x: self.pos.x + 1, y: self.pos.y + 2};
                self.marks[3] = Position { x: self.pos.x + 0, y: self.pos.y + 1};
                self.marks[4] = Position { x: self.pos.x + 2, y: self.pos.y + 1};
            }
            RockShape::LLLL => {
                self.marks[0] = Position { x: self.pos.x + 0, y: self.pos.y + 0};
                self.marks[1] = Position { x: self.pos.x + 1, y: self.pos.y + 0};
                self.marks[2] = Position { x: self.pos.x + 2, y: self.pos.y + 0};
                self.marks[3] = Position { x: self.pos.x + 2, y: self.pos.y + 1};
                self.marks[4] = Position { x: self.pos.x + 2, y: self.pos.y + 2};
            }
            RockShape::VBar => {
                self.marks[0] = Position { x: self.pos.x + 0, y: self.pos.y + 0};
                self.marks[1] = Position { x: self.pos.x + 0, y: self.pos.y + 1};
                self.marks[2] = Position { x: self.pos.x + 0, y: self.pos.y + 2};
                self.marks[3] = Position { x: self.pos.x + 0, y: self.pos.y + 3};
            }
            RockShape::Cube => {
                self.marks[0] = Position { x: self.pos.x + 0, y: self.pos.y + 0};
                self.marks[1] = Position { x: self.pos.x + 1, y: self.pos.y + 0};
                self.marks[2] = Position { x: self.pos.x + 0, y: self.pos.y + 1};
                self.marks[3] = Position { x: self.pos.x + 1, y: self.pos.y + 1};
            }
        }
    }


    fn collide(self: &Self, other: &Rock) -> bool {
        for lpos in &self.marks {
            for rpos in &other.marks {
                if lpos == rpos {
                    return true;
                }
            }
        }
        false
    }
}

fn attempt_update(new_rock: &mut Rock, d: &Direction, chamber: &Vec<Rock>) -> bool {
    let last_position = new_rock.pos;
    new_rock.update(d);
    for rock in chamber {
        if new_rock.collide(rock) {
            new_rock.pos = last_position;
            new_rock.update_marks();
            return false;
        }
    }
    if d == &Direction::Down && new_rock.pos.y == 0 {
        new_rock.pos = last_position;
        new_rock.update_marks();
        false
    } else {
        true 
    }
}

fn tetris(tower_tall: usize) -> usize {
    let contents = fs::read_to_string("my_input")
        .expect("Cannot read the file");
    let mut jet_pattern = contents.trim().as_bytes().iter().cycle();
    let mut chamber: Vec<Rock> = vec![];
    let mut current_shape = RockShape::HBar;
    let mut yhighest_rock: usize = usize::MIN;
    while chamber.len() != tower_tall {
        let mut new_rock = Rock::new(yhighest_rock, current_shape);
        'outer: loop {
            // being pushed by a jet of hot gas
            match jet_pattern.next().unwrap() {
                b'<' => attempt_update(&mut new_rock, &Direction::Left,  &chamber),
                b'>' => attempt_update(&mut new_rock, &Direction::Right, &chamber),
                c    => panic!("Invalid input file: {}", *c as char)
            };
            // falling one unit down
            if !attempt_update(&mut new_rock, &Direction::Down, &chamber) {
                break 'outer;
            }
        }
        if new_rock.pos.y + (new_rock.height - 1) > yhighest_rock {
            yhighest_rock = new_rock.pos.y + (new_rock.height - 1);
        }
        current_shape = current_shape.next();
        chamber.push(new_rock);
    }
    yhighest_rock
}
/*

    Part. I: 3209
*/

fn main() {
    assert_eq!(tetris(2022), 3209);
    //println!("Part. II : {}", tetris(1_000_000_000_000));
}
