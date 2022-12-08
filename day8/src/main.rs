use std::fs;
use std::cmp;

/*

XXXXXTXXXXX
XXXXXTXXXXX
XXXXXTXXXXX
LLLLL_RRRRR
XXXXXBXXXXX
XXXXXBXXXXX

*/

#[inline]
fn top(grid: &Vec<Vec<&u8>>, tree: &u8, tree_x: usize, tree_y: usize) -> bool {
    for current_y in 0..tree_y {
        if grid[current_y][tree_x] >= tree {
            return false;
        }
    }
    true
}

#[inline]
fn top_score(grid: &Vec<Vec<&u8>>, tree: &u8, tree_x: usize, tree_y: usize) -> usize {
    let mut score: usize = 0;
    for current_y in (0..tree_y).rev() {
        score += 1;
        if grid[current_y][tree_x] >= tree {
            break;
        }
    }
    score
}

#[inline]
fn bot(grid: &Vec<Vec<&u8>>, tree: &u8, tree_x: usize, tree_y: usize) -> bool {
    for current_y in (tree_y + 1..grid.len()).rev() {
        if grid[current_y][tree_x] >= tree {
            return false;
        }
    }
    true
}
#[inline]
fn bot_score(grid: &Vec<Vec<&u8>>, tree: &u8, tree_x: usize, tree_y: usize) -> usize {
    let mut score: usize = 0;
    for current_y in tree_y + 1..grid.len() {
        score += 1;
        if grid[current_y][tree_x] >= tree {
            break;
        }
    }
    score
}

#[inline]
fn left(grid: &Vec<Vec<&u8>>, tree: &u8, tree_x: usize, tree_y: usize) -> bool {
    for current_x in 0..tree_x {
        if grid[tree_y][current_x] >= tree {
            return false;
        }
    }
    true
}

#[inline]
fn left_score(grid: &Vec<Vec<&u8>>, tree: &u8, tree_x: usize, tree_y: usize) -> usize {
    let mut score: usize = 0;
    for current_x in (0..tree_x).rev() {
        score += 1;
        if grid[tree_y][current_x] >= tree {
            break;
        }
    }
    score
}

#[inline]
fn right(grid: &Vec<Vec<&u8>>, tree: &u8, tree_x: usize, tree_y: usize) -> bool {
    for current_x in (tree_x + 1..grid[0].len()).rev() {
        if grid[tree_y][current_x] >= tree {
            return false;
        }
    }
    true
}

#[inline]
fn right_score(grid: &Vec<Vec<&u8>>, tree: &u8, tree_x: usize, tree_y: usize) -> usize {
    let mut score: usize = 0;
    for current_x in tree_x + 1..grid[0].len() {
        score += 1;
        if grid[tree_y][current_x] >= tree {
            break;
        }
    }
    score
}

fn is_visible(grid: &Vec<Vec<&u8>>, tree: &u8, tree_x: usize, tree_y: usize) -> bool {
    // borders are always visible
    if tree_y == 0 || tree_y == grid.len() - 1 || tree_x == 0 || tree_x == grid[0].len() - 1 {
        return true;
    }
    return top(grid, tree, tree_x, tree_y)
        || bot(grid, tree, tree_x, tree_y)
        || left(grid, tree, tree_x, tree_y)
        || right(grid, tree, tree_x, tree_y);
}

fn tree_score(grid: &Vec<Vec<&u8>>, tree: &u8, tree_x: usize, tree_y: usize) -> usize {
    return top_score(grid, tree, tree_x, tree_y)
        * bot_score(grid, tree, tree_x, tree_y)
        * left_score(grid, tree, tree_x, tree_y)
        * right_score(grid, tree, tree_x, tree_y);
}

fn main() {
    let contents = fs::read_to_string("input").expect("Cannot read the file");

    let grid = contents
        .lines()
        .map(|line| line.as_bytes().iter().collect::<Vec<&u8>>())
        .collect::<Vec<Vec<&u8>>>();
    //let mut tree_visible = grid.len() * 2 + grid[0].len() * 2 - 4;
    let mut tree_visible = 0;
    for (pos_y, row) in grid.iter().enumerate() {
        for (pos_x, tree) in row.iter().enumerate() {
            tree_visible += is_visible(&grid, tree, pos_x, pos_y) as usize;
        }
    }
    println!("Part1: {}", tree_visible);
    let mut best_spot = 0;
    for (pos_y, row) in grid.iter().enumerate() {
        for (pos_x, tree) in row.iter().enumerate() {
            best_spot = cmp::max(best_spot, tree_score(&grid, tree, pos_x, pos_y));
        }
    }
    println!("Part2: {}", best_spot);
}
