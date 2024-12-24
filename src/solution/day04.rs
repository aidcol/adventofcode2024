use std::fs::File;
use std::io::{BufRead, BufReader};

fn parse_input(path: &str) -> Vec<Vec<char>> {
    let file = File::open(path).expect("Cannot open file");
    let reader = BufReader::new(file);
    
    let mut grid = Vec::new();
    for line in reader.lines() {
        let line = line.unwrap();
        let row: Vec<char> = line.chars().collect();
        grid.push(row);
    }

    grid
}

#[derive(Debug)]
enum Direction {
    Right,
    Left,
    Down,
    Up,
    DownRight,
    DownLeft,
    UpRight,
    UpLeft,
}

#[derive(Debug)]
struct Path {
    row: i32,
    col: i32,
    direction: Direction,
}

impl Path {
    fn next(&mut self) -> (i32, i32) {
        match self.direction {
            Direction::Right => self.col += 1,
            Direction::Left => self.col -= 1,
            Direction::Down => self.row += 1,
            Direction::Up => self.row -= 1,
            Direction::DownRight => {
                self.row += 1;
                self.col += 1;
            },
            Direction::DownLeft => {
                self.row += 1;
                self.col -= 1;
            },
            Direction::UpRight => {
                self.row -= 1;
                self.col += 1;
            },
            Direction::UpLeft => {
                self.row -= 1;
                self.col -= 1;
            },
        }

        (self.row, self.col)
    }
}

fn dfs(grid: &Vec<Vec<char>>, row: usize, col: usize) -> u32 {
    let mut count = 0;
    let mut stack = Vec::new();

    let row = row as i32;
    let col = col as i32;
    stack.push(Path{row, col, direction: Direction::Right});
    stack.push(Path{row, col, direction: Direction::Left});
    stack.push(Path{row, col, direction: Direction::Down});
    stack.push(Path{row, col, direction: Direction::Up});
    stack.push(Path{row, col, direction: Direction::DownRight});
    stack.push(Path{row, col, direction: Direction::DownLeft});
    stack.push(Path{row, col, direction: Direction::UpRight});
    stack.push(Path{row, col, direction: Direction::UpLeft});

    while !stack.is_empty() {
        let mut path = stack.pop().unwrap();
        let row = path.row as usize;
        let col = path.col as usize;

        let (new_row, new_col) = path.next();
        if new_row < 0 || new_row >= grid.len() as i32 {
            continue;
        }
        else if new_col < 0 || new_col >= grid[0].len() as i32 {
            continue;
        }
        
        let new_row = new_row as usize;
        let new_col = new_col as usize;
        match grid[row][col] {
            'X' => {
                if grid[new_row][new_col] == 'M' {
                    stack.push(path);
                }
            },
            'M' => {
                if grid[new_row][new_col] == 'A' {
                    stack.push(path);
                }
            },
            'A' => {
                if grid[new_row][new_col] == 'S' {
                    count += 1;
                }
            },
            _ => continue,
        }
    }

    count
}

fn search(grid: Vec<Vec<char>>) -> u32 {
    let mut count = 0;
    
    for row in 0..grid.len() {
        for col in 0..grid[0].len() {
            if grid[row][col] == 'X' {
                count += dfs(&grid, row, col);
            }
        }
    }

    count
}

pub struct Day04;

impl super::Solution for Day04 {
    fn get_part_one(&self) -> String {
        let path = "src/input/day04.txt";
        let grid = parse_input(path);
        let result = search(grid);
        result.to_string()
    }

    fn get_part_two(&self) -> String {
        "Not implemented yet".to_string()
    }
}

#[cfg(test)]
mod tests;
