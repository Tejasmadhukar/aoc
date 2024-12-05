use std::{collections::BTreeMap, usize};

use regex::Regex;

use crate::solution::Solution;

pub struct Q3 {}

impl Solution for Q3 {
    fn get_question_number(&self) -> u8 {
        3
    }
    fn solve_part_one(&self, path: Option<&str>) -> i32 {
        let mut input = self.get_input();
        match path {
            Some(path) => input = self.get_custom_input(path),
            None => {}
        }

        let re = Regex::new(r"mul\(((?<first>[0-9]{1,3}),(?<second>[0-9]{1,3}))\)").unwrap();

        re.captures_iter(&input)
            .map(|caps| {
                let first = caps.name("first").unwrap().as_str().parse::<i32>().unwrap();
                let second = caps
                    .name("second")
                    .unwrap()
                    .as_str()
                    .parse::<i32>()
                    .unwrap();
                first * second
            })
            .sum()
    }
    fn solve_part_two(&self, _path: Option<&str>) -> i32 {
        let mut input = self.get_input();
        match _path {
            Some(path) => input = self.get_custom_input(path),
            None => {}
        }

        let re_mul = Regex::new(r"mul\(((?<first>[0-9]{1,3}),(?<second>[0-9]{1,3}))\)").unwrap();
        let re_do = Regex::new(r"do\(\)").unwrap();
        let re_dont = Regex::new(r"don't\(\)").unwrap();

        let muls: Vec<i32> = re_mul
            .captures_iter(&input)
            .map(|caps| {
                let first = caps.name("first").unwrap().as_str().parse::<i32>().unwrap();
                let second = caps
                    .name("second")
                    .unwrap()
                    .as_str()
                    .parse::<i32>()
                    .unwrap();
                first * second
            })
            .collect();

        let mut safe: BTreeMap<usize, bool> = BTreeMap::new();
        let mut mul_start: BTreeMap<usize, i32> = BTreeMap::new();

        let mut insert_idx = 0;
        for mat in re_mul.find_iter(&input) {
            mul_start.insert(mat.start(), muls[insert_idx]);
            insert_idx += 1;
        }

        for mat in re_do.find_iter(&input) {
            safe.insert(mat.start(), true);
        }

        for mat in re_dont.find_iter(&input) {
            safe.insert(mat.start(), false);
        }

        let mut enabled = true;
        let mut answer = 0;

        for idx in 0..input.len() {
            match mul_start.get(&idx) {
                Some(i) => {
                    if enabled {
                        answer += i
                    }
                }
                None => {}
            }
            match safe.get(&idx) {
                Some(enable) => {
                    enabled = *enable;
                }
                None => {}
            }
        }

        answer
    }
}
