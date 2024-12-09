use std::{collections::HashSet, usize};

use crate::solution::Solution;

pub struct Q6 {}

#[derive(Copy, Clone, Debug)]
enum Direction {
    Up = 0,
    Right = 1,
    Down = 2,
    Left = 3,
}

impl Direction {
    fn next(&self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }
}

#[derive(Debug)]
struct Game {
    obstacles: HashSet<(i32, i32)>,
    guard_position: (i32, i32),
    guard_direction: Direction,
    game_limits: (i32, i32),
}

impl Game {
    fn check_out_of_bounds(&self) -> bool {
        if self.guard_position.0 < 0 || self.guard_position.0 > self.game_limits.0 {
            return true;
        }

        if self.guard_position.1 < 0 || self.guard_position.1 > self.game_limits.1 {
            return true;
        }

        false
    }
    fn change_direction(&mut self) {
        self.guard_direction = self.guard_direction.next();
    }
    fn is_obstacle_ahead(&self) -> bool {
        let directions: Vec<(i32, i32)> = vec![(0, -1), (1, 0), (0, 1), (-1, 0)];

        let c = directions.get(self.guard_direction as usize).unwrap();
        let dir_to_check = (self.guard_position.0 + c.0, self.guard_position.1 + c.1);

        if self.obstacles.contains(&dir_to_check) {
            return true;
        }
        false
    }
    fn move_ahead(&mut self) {
        let directions: Vec<(i32, i32)> = vec![(0, -1), (1, 0), (0, 1), (-1, 0)];
        let c = directions.get(self.guard_direction as usize).unwrap();
        self.guard_position = (self.guard_position.0 + c.0, self.guard_position.1 + c.1);
    }
}

impl Solution for Q6 {
    fn get_question_number(&self) -> u8 {
        6
    }
    fn solve_part_one(&self, path: Option<&str>) -> i32 {
        let mut input = self.get_input();
        match path {
            Some(path) => input = self.get_custom_input(path),
            None => {}
        }

        let m = input.lines().count() - 1;
        let n = input.lines().nth(0).unwrap().len() - 1;
        let mut obstacles: HashSet<(i32, i32)> = HashSet::new();
        let mut gp: (i32, i32) = (-1, -1);

        for (j, line) in input.lines().enumerate() {
            for (i, c) in line.chars().enumerate() {
                if c == '#' {
                    obstacles.insert((i.try_into().unwrap(), j.try_into().unwrap()));
                }
                if c == '^' {
                    gp = (i.try_into().unwrap(), j.try_into().unwrap());
                }
            }
        }

        let mut game = Game {
            game_limits: (m.try_into().unwrap(), n.try_into().unwrap()),
            obstacles,
            guard_position: gp,
            guard_direction: Direction::Up,
        };

        let mut unique_position: HashSet<(i32, i32)> = HashSet::new();

        loop {
            if game.check_out_of_bounds() {
                break;
            }

            if game.is_obstacle_ahead() {
                game.change_direction();
                continue;
            }

            unique_position.insert(game.guard_position);
            game.move_ahead();
        }

        unique_position.len().try_into().unwrap()
    }
}
