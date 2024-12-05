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

                let mut itr = levels.windows(2);

                let pair = itr.next().unwrap();
                let increasing = pair[0] < pair[1];

                let difference = pair[0].abs_diff(pair[1]);

                if difference < 1 || difference > 3 {
                    return 0;
                }

                for pair in itr {
                    let difference = pair[0].abs_diff(pair[1]);
                    if difference < 1 || difference > 3 {
                        return 0;
                    }

                    if (pair[0] > pair[1] && increasing) || (pair[1] > pair[0] && !increasing) {
                        return 0;
                    }
                }

                1
            })
            .sum()
    }
}
