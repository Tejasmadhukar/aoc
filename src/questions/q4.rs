use std::usize;

use crate::solution::Solution;

pub struct Q4 {}

const WORD: &str = "XMAS";

impl Solution for Q4 {
    fn get_question_number(&self) -> u8 {
        4
    }
    fn solve_part_one(&self, path: Option<&str>) -> i32 {
        let mut input = self.get_input();
        match path {
            Some(path) => input = self.get_custom_input(path),
            None => {}
        }

        let mut grid: Vec<Vec<char>> = Vec::new();

        for line in input.lines() {
            let mut row: Vec<char> = Vec::new();
            for c in line.chars() {
                row.push(c);
            }
            grid.push(row);
        }

        let mut ans = 0;

        for row in 0..grid.len() {
            for col in 0..grid[row].len() {
                ans += search_coordinate(&grid, row.try_into().unwrap(), col.try_into().unwrap());
            }
        }

        ans
    }
    fn solve_part_two(&self, _path: Option<&str>) -> i32 {
        let mut input = self.get_input();
        match _path {
            Some(path) => input = self.get_custom_input(path),
            None => {}
        }

        let mut grid: Vec<Vec<char>> = Vec::new();

        for line in input.lines() {
            let mut row: Vec<char> = Vec::new();
            for c in line.chars() {
                row.push(c);
            }
            grid.push(row);
        }

        let mut ans = 0;

        for row in 0..grid.len() {
            for col in 0..grid[row].len() {
                ans += search_x_mas(&grid, row.try_into().unwrap(), col.try_into().unwrap());
            }
        }

        ans
    }
}

fn search_x_mas(grid: &Vec<Vec<char>>, r: i32, c: i32) -> i32 {
    if grid[r as usize][c as usize] != 'A' {
        return 0;
    }

    let directions: Vec<(i32, i32)> = vec![(-1, 1), (1, 1), (1, -1), (-1, -1)];
    let mut points: Vec<char> = Vec::new();

    for dir in directions {
        let cx = r + dir.0;
        let cy = r + dir.1;

        if !(0..grid.len()).contains(&(cx as usize)) || !(0..grid[0].len()).contains(&(cy as usize))
        {
            return 0;
        }

        let point = grid[cx as usize][cy as usize];

        if point != 'M' && point != 'S' {
            return 0;
        }

        points.push(point);
    }

    if points[0] != points[2] && points[1] != points[3] {
        return 1;
    }

    0
}

fn search_coordinate(grid: &Vec<Vec<char>>, r: i32, c: i32) -> i32 {
    let mut count = 0;
    let directions: Vec<(i32, i32)> = vec![
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];

    if grid[r as usize][c as usize] != WORD.chars().nth(0).unwrap() {
        return 0;
    }

    for dir in directions {
        let mut cx = r + dir.0;
        let mut cy = c + dir.1;

        let mut k = 1;
        while k < WORD.len() {
            if !(0..grid.len()).contains(&(cx as usize))
                || !(0..grid[0].len()).contains(&(cy as usize))
            {
                break;
            }

            if grid[cx as usize][cy as usize] != WORD.chars().nth(k).unwrap() {
                break;
            }

            cx += dir.0;
            cy += dir.1;
            k += 1;
        }

        if k == WORD.len() {
            count += 1;
        }
    }

    count
}
