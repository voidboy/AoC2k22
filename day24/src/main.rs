use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;

type Point = (usize, usize);

#[derive(Clone, Debug)]
enum Direction {
    North,
    South,
    East,
    West,
    Wait,
}

#[derive(Clone, Debug)]
struct Blizzard {
    pos: Point,
    dir: Direction,
}

fn parse_blizzard(map: &Vec<Vec<char>>, blizzard: &mut Vec<Blizzard>) {
    for y in 1..map.len() - 1 {
        for x in 1..map[0].len() - 1 {
            match map[y][x] {
                '^' => blizzard.push(Blizzard {
                    pos: (x, y),
                    dir: Direction::North,
                }),
                'v' => blizzard.push(Blizzard {
                    pos: (x, y),
                    dir: Direction::South,
                }),
                '>' => blizzard.push(Blizzard {
                    pos: (x, y),
                    dir: Direction::East,
                }),
                '<' => blizzard.push(Blizzard {
                    pos: (x, y),
                    dir: Direction::West,
                }),
                _ => {}
            }
        }
    }
}

#[derive(Clone, Debug)]
struct Expedition {
    player_position: Point,
    end_position: Point,
    map: Vec<Vec<char>>,
    tried_path: HashMap<String, Point>,
    this_blizzard: Vec<Blizzard>,
    time_blizzard: Vec<HashSet<Point>>,
    found: bool,
}

impl Expedition {
    fn simulate(self: &mut Self, simulation_time: usize) {
        for _ in 0..simulation_time {
            self.play_blizzard();
            let mut this: HashSet<Point> = HashSet::new();
            self.this_blizzard.iter().for_each(|b| {
                this.insert(b.pos);
            });
            self.time_blizzard.push(this);
        }
    }

    fn print_map(self: &Self) {
        for line in self.map.iter() {
            println!("{}", line.iter().collect::<String>());
        }
        println!("================");
    }

    fn play_blizzard(self: &mut Self) {
        for blizzard in self.this_blizzard.iter_mut() {
            let (x, y) = blizzard.pos;
            match blizzard.dir {
                Direction::North => {
                    if y > 1 {
                        blizzard.pos = (x, y - 1);
                    } else {
                        blizzard.pos = (x, self.map.len() - 2);
                    }
                }
                Direction::South => {
                    if y < self.map.len() - 2 {
                        blizzard.pos = (x, y + 1);
                    } else {
                        blizzard.pos = (x, 1);
                    }
                }
                Direction::East => {
                    if x < self.map[0].len() - 2 {
                        blizzard.pos = (x + 1, y);
                    } else {
                        blizzard.pos = (1, y);
                    }
                }
                Direction::West => {
                    if x > 1 {
                        blizzard.pos = (x - 1, y);
                    } else {
                        blizzard.pos = (self.map[0].len() - 2, y);
                    }
                }
                Direction::Wait => panic!("Blizzard cannot wait"),
            }
        }
    }

    fn update_map(self: &mut Self) {
        // reset the all map
        for y in 1..self.map.len() - 1 {
            for x in 1..self.map[0].len() - 1 {
                self.map[y][x] = '.';
            }
        }
        for blizzard in &self.this_blizzard {
            let (x, y) = blizzard.pos;
            match self.map[y][x] {
                '.' => match blizzard.dir {
                    Direction::North => self.map[y][x] = '^',
                    Direction::South => self.map[y][x] = 'v',
                    Direction::East => self.map[y][x] = '>',
                    Direction::West => self.map[y][x] = '<',
                    Direction::Wait => panic!("Blizzard cannot wait"),
                },
                '2'..='3' => self.map[y][x] = (self.map[y][x] as u8 + 1) as char,
                '^' | 'v' | '<' | '>' => self.map[y][x] = '2',
                _ => panic!("Invalid map"),
            }
        }
        self.map[self.player_position.1][self.player_position.0] = 'E';
    }

    fn find_path(self: &mut Self, player_position: Point, depth: usize, dirs: String) -> bool {
        let (mut x, mut y) = player_position;
        match dirs.chars().last().unwrap() {
            'N' => y -= 1,
            'S' => y += 1,
            'E' => x += 1,
            'W' => x -= 1,
            '_' => {}
            _ => panic!("Invalid direction"),
        }
        if (x, y) == self.end_position {
            return true;
        } else if (player_position == self.player_position)
            || (x > 0
                && x < self.map[0].len() - 1
                && y > 0
                && y < self.map.len() - 1
                && !self.time_blizzard[depth].contains(&(x, y)))
        {
            self.tried_path.insert(dirs.clone(), (x, y));
        }
        false
    }
}

fn find_dot(row: &Vec<char>) -> Option<usize> {
    for (i, byte) in row.iter().enumerate() {
        match byte {
            '.' => return Some(i),
            _ => {}
        }
    }
    None
}

fn main() {
    let contents = fs::read_to_string("my_input").expect("Cannot read the file");

    let map = contents
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    let player_position: Point = if let Some(start) = find_dot(&map[0]) {
        (start, 0)
    } else {
        panic!("start position is missing")
    };
    let end_position: Point = if let Some(end) = find_dot(&map[map.len() - 1]) {
        (end, map.len() - 1)
    } else {
        panic!("end position is missing")
    };
    println!("Start @{:?} End @{:?}", player_position, end_position);
    let mut this_blizzard: Vec<Blizzard> = vec![];
    parse_blizzard(&map, &mut this_blizzard);
    let mut expedition: Expedition = Expedition {
        map,
        tried_path: HashMap::new(),
        this_blizzard,
        player_position,
        end_position,
        time_blizzard: vec![],
        found: false,
    };
    let mut depth: usize = 1;
    'main: loop {
        if depth >= expedition.time_blizzard.len() {
            expedition.simulate(10);
        }
        if depth > 1 {
            let mut to_test: Vec<(String, Point)> = vec![];
            for (key, value) in expedition.tried_path.iter() {
                to_test.push((key.clone(), *value));
            }
            if depth == 10 {
                println!("{:?}", expedition.tried_path);
                break 'main;
            }
            expedition.tried_path.clear();
            for (key, value) in to_test.iter() {
                if expedition.find_path(*value, depth, key.to_owned() + "N")
                    || expedition.find_path(*value, depth, key.to_owned() + "S")
                    || expedition.find_path(*value, depth, key.to_owned() + "E")
                    || expedition.find_path(*value, depth, key.to_owned() + "W")
                    || expedition.find_path(*value, depth, key.to_owned() + "_")
                {
                    println!("FOUND path with {} moves", depth + 1);
                    break 'main;
                }
            }
        } else {
            expedition.find_path(player_position, 0, String::from("S"));
        }
        depth += 1;
    }
}
