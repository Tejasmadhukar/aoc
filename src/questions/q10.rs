use std::{collections::HashSet, isize, usize};

use crate::solution::Solution;

pub struct Q10 {}

impl Solution for Q10 {
    fn get_question_number(&self) -> u8 {
        10
    }
    fn solve_part_one(&self, path: Option<&str>) -> i32 {
        let mut input = self.get_input();
        match path {
            Some(path) => input = self.get_custom_input(path),
            None => {}
        }

        let grid: Vec<Vec<u8>> = input
            .lines()
            .map(|line| {
                line.chars()
                    .map(|x| x.to_digit(10).unwrap() as u8)
                    .collect()
            })
            .collect();

        let mut ans = 0;

        for i in 0..grid.len() {
            for j in 0..grid.len() {
                if grid[i][j] != 0 {
                    continue;
                }

                let mut unique_final: HashSet<(usize, usize)> = HashSet::new();
                solve_one(&grid, i, j, &mut unique_final);
                ans += unique_final.len();
            }
        }

        ans as i32
    }
}

fn solve_one(grid: &Vec<Vec<u8>>, x: usize, y: usize, seen: &mut HashSet<(usize, usize)>) {
    let dirs: Vec<(isize, isize)> = vec![(-1, 0), (0, 1), (1, 0), (0, -1)];

    if grid[x][y] == 9 {
        seen.insert((x, y));
    }

    for dir in dirs {
        let nx = x as isize + dir.0;
        let ny = y as isize + dir.1;

        if (0..grid[0].len()).contains(&(nx as usize)) && (0..grid.len()).contains(&(ny as usize)) {
            if grid[nx as usize][ny as usize] == grid[x][y] + 1 {
                solve_one(grid, nx as usize, ny as usize, seen);
            }
        }
    }
}
