use std::collections::HashSet;

use crate::solution::Solution;

pub struct Q14 {}

const WIDTH: isize = 101;
const HEIGHT: isize = 103;

impl Solution for Q14 {
    fn get_question_number(&self) -> u8 {
        14
    }
    fn solve_part_one(&self, path: Option<&str>) -> i32 {
        let mut input = self.get_input();
        match path {
            Some(path) => input = self.get_custom_input(path),
            None => {}
        }

        let mut quadrants: Vec<isize> = vec![0; 4];

        for line in input.lines() {
            let position: Vec<isize> = line
                .split_whitespace()
                .nth(0)
                .unwrap()
                .replace("p=", "")
                .split(",")
                .map(|x| x.parse::<isize>().unwrap())
                .collect();
            let velocity: Vec<isize> = line
                .split_whitespace()
                .nth(1)
                .unwrap()
                .replace("v=", "")
                .split(",")
                .map(|x| x.parse::<isize>().unwrap())
                .collect();

            let nx = if (velocity[0] * 100 + position[0]) % WIDTH < 0 {
                ((velocity[0] * 100 + position[0]) % WIDTH) + WIDTH
            } else {
                (velocity[0] * 100 + position[0]) % WIDTH
            };

            let ny = if (velocity[1] * 100 + position[1]) % HEIGHT < 0 {
                ((velocity[1] * 100 + position[1]) % HEIGHT) + HEIGHT
            } else {
                (velocity[1] * 100 + position[1]) % HEIGHT
            };

            if nx < WIDTH / 2 && ny < HEIGHT / 2 {
                quadrants[0] += 1;
            }

            if nx > WIDTH / 2 && ny < HEIGHT / 2 {
                quadrants[1] += 1;
            }

            if nx > WIDTH / 2 && ny > HEIGHT / 2 {
                quadrants[2] += 1;
            }

            if nx < WIDTH / 2 && ny > HEIGHT / 2 {
                quadrants[3] += 1;
            }
        }

        let mut ans = 1;
        for pos in quadrants.iter() {
            ans *= pos;
        }
        ans as i32
    }
}
