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
}
