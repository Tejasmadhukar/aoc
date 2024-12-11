use crate::solution::Solution;
use memoize::memoize;

pub struct Q11 {}

impl Solution for Q11 {
    fn get_question_number(&self) -> u8 {
        11
    }
    fn solve_part_one(&self, path: Option<&str>) -> i32 {
        let mut input = self.get_input();
        match path {
            Some(path) => input = self.get_custom_input(path),
            None => {}
        }

        let mut stones: Vec<u64> = input
            .split_whitespace()
            .map(|x| x.parse::<u64>().unwrap())
            .collect();

        let blink = |stones: &mut Vec<u64>| {
            let mut idx = 0;
            while idx < stones.len() {
                if stones[idx] == 0 {
                    stones[idx] = 1;
                    idx += 1;
                    continue;
                }

                if count_digits(stones[idx]) % 2 == 0 {
                    let new = split_digits(stones[idx]);
                    stones.insert(idx, new.0);
                    if idx + 1 < stones.len() {
                        stones[idx + 1] = new.1;
                    }
                    idx += 2;
                    continue;
                }

                stones[idx] *= 2024;
                idx += 1;
            }
        };

        for _ in 0..25 {
            blink(&mut stones);
        }

        stones.len() as i32
    }
}

impl Q11 {
    #[allow(dead_code)]
    pub fn custom_part_two(&self, _path: Option<&str>) -> u64 {
        let mut input = self.get_input();
        match _path {
            Some(path) => input = self.get_custom_input(path),
            None => {}
        }

        let stones: Vec<u64> = input
            .split_whitespace()
            .map(|x| x.parse::<u64>().unwrap())
            .collect();

        stones.iter().map(|x| solve(*x, 75)).sum()
    }
}

fn count_digits(num: u64) -> u32 {
    if num == 0 {
        return 1;
    }
    (num as f64).log10().floor() as u32 + 1
}

fn split_digits(num: u64) -> (u64, u64) {
    let divisor = 10u64.pow(count_digits(num) / 2);
    let left = num / divisor;
    let right = num % divisor;
    (left, right)
}

#[allow(dead_code)]
#[memoize]
fn solve(num: u64, steps: u8) -> u64 {
    if steps == 0 {
        return 1;
    }
    let mut result = 0;

    if num == 0 {
        result = solve(1, steps - 1);
    } else if count_digits(num) % 2 == 0 {
        let new = split_digits(num);
        result += solve(new.0, steps - 1);
        result += solve(new.1, steps - 1);
    } else {
        result = solve(num * 2024, steps - 1);
    }

    result
}
