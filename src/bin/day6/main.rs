
use std::collections::HashSet;

use advent_of_code::read_lines;
use itertools::enumerate;

enum Direction {
    Up,
    Right,
    Down,
    Left
}

fn main() {
    let grid: Vec<String> = read_lines("src/bin/day6/input").unwrap().map(|x| x.unwrap()).collect();
    let n = grid.len();
    let m = grid[0].len();
    let mut direction = Direction::Up;

    let mut cur = get_start_pos(&grid);
    let mut part_one = 0;
    let mut visited = HashSet::<(usize, usize)>::new();

    while cur.0 < n && cur.1 < m {
        let pos = (cur.0,cur.1);
        if !visited.contains(&pos) {
            part_one += 1;
            visited.insert(pos);
        }
        match direction {
            Direction::Up => {
                if cur.0 == 0 { break; }
                let pos = (cur.0 - 1, cur.1);
                if get_pos(&grid, pos) == '#' {
                    direction = change_direction(direction);
                } else {
                    cur = pos;
                }
            }
            Direction::Right => {
                if cur.1 == m - 1 { break; }
                let pos = (cur.0, cur.1 + 1);
                if get_pos(&grid, pos) == '#' {
                    direction = change_direction(direction);
                } else {
                    cur = pos;
                }
            }
            Direction::Down => {
                if cur.0 == n - 1 { break; }
                let pos = (cur.0 + 1, cur.1);
                if get_pos(&grid, pos) == '#' {
                    direction = change_direction(direction);
                } else {
                    cur = pos;
                }
            }
            Direction::Left => {
                if cur.1 == 0 { break; }
                let pos = (cur.0, cur.1 - 1);
                if get_pos(&grid, pos) == '#' {
                    direction = change_direction(direction);
                } else {
                    cur = pos;
                }
            }
        };
    }

    println!("{:#?}", part_one);
}

fn change_direction(direction: Direction) -> Direction {
    match direction {
        Direction::Up => Direction::Right,
        Direction::Right => Direction::Down,
        Direction::Down => Direction::Left,
        Direction::Left => Direction::Up,
    }
}

fn get_pos(grid: &Vec<String>, pos: (usize, usize)) -> char {
    grid[pos.0].chars().nth(pos.1).unwrap()
}

fn get_start_pos(grid: &Vec<String>) -> (usize, usize) {
    for (i, row) in enumerate(grid) {
        for (j, val) in enumerate(row.chars()) {
            if val == '^' {
                return (i, j);
            }
        }
    }
    panic!("no start position found");
}