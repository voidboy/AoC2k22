use std::fs;
use std::{thread, time};

/*

    Part. I : 994
*/

type Point = (usize, usize);

struct Cave {
    map: Vec<Vec<u8>>,
    rock_paths: Vec<Vec<Point>>,
    min_max_x: Point,
    x_length: usize,
    min_max_y: Point,
    y_length: usize,
}

impl Cave {
    fn print(self: &Self) {
        for row in self.map.iter() {
            for c in row {
                match c {
                    b'.' => print!("."),
                    b'o' => print!("o"),
                    b'#' => print!("#"),
                    _ => panic!("Unvalid character"),
                }
            }
            println!();
        }
    }

    fn normalize(self: &mut Self) {
        for rock_path in &mut self.rock_paths {
            for point in rock_path.into_iter() {
                point.0 -= self.min_max_x.0;
            }
        }
    }

    fn fill(self: &mut Self) {
        for rock_path in &self.rock_paths {
            let mut last: Option<&Point> = None;
            for point in rock_path.iter() {
                if last.is_some() {
                    let from = last.unwrap();
                    // X
                    if from.0 != point.0 {
                        for x in from.0.min(point.0)..=from.0.max(point.0) {
                            self.map[from.1][x] = b'#';
                        }
                    }
                    // Y
                    if from.1 != point.1 {
                        for y in from.1.min(point.1)..=from.1.max(point.1) {
                            self.map[y][from.0] = b'#';
                        }
                    }
                }
                last = Some(point);
            }
        }
    }

    fn simulate(self: &mut Self) -> usize {
        let mut sand_counter: usize = 0;
        'main: loop {
            let mut sand: Point = (500 - self.min_max_x.0, 0);
            'inner: loop {
                if sand.1 == self.y_length - 1 || sand.0 == 0 || sand.0 == self.x_length - 1 {
                    break 'main;
                }
                if self.map[sand.1 + 1][sand.0] == b'.' {
                    self.map[sand.1][sand.0] = b'.';
                    sand.1 += 1;
                    self.map[sand.1][sand.0] = b'o';
                } else if self.map[sand.1 + 1][sand.0 - 1] == b'.' {
                    self.map[sand.1][sand.0] = b'.';
                    sand.1 += 1;
                    sand.0 -= 1;
                    self.map[sand.1][sand.0] = b'o';
                } else if self.map[sand.1 + 1][sand.0 + 1] == b'.' {
                    self.map[sand.1][sand.0] = b'.';
                    sand.1 += 1;
                    sand.0 += 1;
                    self.map[sand.1][sand.0] = b'o';
                } else {
                    sand_counter += 1;
                    break 'inner;
                }
            }
        }
        sand_counter
    }
}

fn main() {
    let contents = fs::read_to_string("my_input").expect("Cannot read the file");

    let rock_paths = contents
        .lines()
        .map(|line| {
            line.split("->")
                .map(|entry| {
                    let mut point = entry.trim().split(',');
                    (
                        point.next().unwrap().parse::<usize>().unwrap(),
                        point.next().unwrap().parse::<usize>().unwrap(),
                    )
                })
                .collect::<Vec<Point>>()
        })
        .collect::<Vec<Vec<Point>>>();
    let mut min_max_x: Point = (usize::MAX, 0);
    let mut min_max_y: Point = (0, 0);
    rock_paths.iter().flatten().for_each(|point| {
        if point.0 < min_max_x.0 {
            min_max_x.0 = point.0;
        }
        if point.0 > min_max_x.1 {
            min_max_x.1 = point.0;
        }
        // min. Y must be 0, sand starts @(500,0)
        if point.1 < min_max_y.0 {
            min_max_y.0 = point.1;
        }
        if point.1 > min_max_y.1 {
            min_max_y.1 = point.1;
        }
    });
    let x_length: usize = (min_max_x.1 - min_max_x.0) + 1;
    let y_length: usize = (min_max_y.1 - min_max_y.0) + 1;
    let mut cave = Cave {
        map: vec![vec![b'.'; x_length]; y_length],
        rock_paths,
        min_max_x,
        x_length,
        min_max_y,
        y_length,
    };
    cave.normalize();
    cave.fill();
    println!("Part I. : {}", cave.simulate());
}
