use std::collections::HashMap;

use crate::solution::Solution;

pub struct Q1 {}

impl Solution for Q1 {
    fn get_question_number(&self) -> u8 {
        1
    }
    fn solve_part_one(&self, path: Option<&str>) -> i32 {
        let mut input = self.get_input();
        match path {
            Some(path) => input = self.get_custom_input(path),
            None => {}
        }

        let mut left_list = Vec::new();
        let mut right_list = Vec::new();

        for line in input.lines() {
            let mut parts = line.split("   ");
            left_list.push(parts.next().unwrap().parse::<i32>().unwrap());
            right_list.push(parts.next().unwrap().parse::<i32>().unwrap());
        }

        left_list.sort();
        right_list.sort();

        let answer: u32 = left_list
            .iter()
            .zip(right_list)
            .map(|t| t.0.abs_diff(t.1))
            .sum();

        answer.try_into().unwrap()
    }
    fn solve_part_two(&self, _path: Option<&str>) -> i32 {
        let mut input = self.get_input();
        match _path {
            Some(path) => input = self.get_custom_input(path),
            _none => {}
        }

        let mut left_list = Vec::new();
        let mut right_list = Vec::new();
        let mut right_count: HashMap<i32, i32> = HashMap::new();

        for line in input.lines() {
            let mut parts = line.split("   ");
            left_list.push(parts.next().unwrap().parse::<i32>().unwrap());
            right_list.push(parts.next().unwrap().parse::<i32>().unwrap());
        }

        for num in right_list.iter() {
            match right_count.get(num) {
                Some(&count) => right_count.insert(*num, count + 1),
                None => right_count.insert(*num, 1),
            };
        }

        left_list
            .iter()
            .map(|num| match right_count.get(num) {
                Some(&count) => num * count,
                None => 0,
            })
            .sum()
    }
}
