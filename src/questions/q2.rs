use crate::solution::Solution;

pub struct Q2 {}

impl Solution for Q2 {
    fn get_question_number(&self) -> u8 {
        2
    }
    fn solve_part_one(&self, path: Option<&str>) -> i32 {
        let mut input = self.get_input();
        match path {
            Some(path) => input = self.get_custom_input(path),
            None => {}
        }

        input
            .lines()
            .map(|line| {
                let levels: Vec<i32> = line
                    .split_whitespace()
                    .map(|level| level.parse::<i32>().unwrap())
                    .collect();

                if !is_level_safe(&levels) {
                    return 0;
                }
                1
            })
            .sum()
    }
    fn solve_part_two(&self, _path: Option<&str>) -> i32 {
        let mut input = self.get_input();
        match _path {
            Some(path) => input = self.get_custom_input(path),
            None => {}
        }
        input
            .lines()
            .map(|line| {
                let levels: Vec<i32> = line
                    .split_whitespace()
                    .map(|level| level.parse::<i32>().unwrap())
                    .collect();

                if is_level_safe(&levels) {
                    return 1;
                }

                for idx in 0..levels.len() {
                    let mut new_level = levels.clone();
                    new_level.remove(idx);
                    if is_level_safe(&new_level) {
                        return 1;
                    }
                }

                0
            })
            .sum()
    }
}

fn is_level_safe(levels: &Vec<i32>) -> bool {
    let mut itr = levels.windows(2);

    let pair = itr.next().unwrap();
    let increasing = pair[0] < pair[1];

    let difference = pair[0].abs_diff(pair[1]);

    if difference < 1 || difference > 3 {
        return false;
    }

    for pair in itr {
        let difference = pair[0].abs_diff(pair[1]);
        if difference < 1 || difference > 3 {
            return false;
        }

        if (pair[0] > pair[1] && increasing) || (pair[1] > pair[0] && !increasing) {
            return false;
        }
    }

    true
}
