use crate::solution::Solution;

pub struct Q7 {}

impl Solution for Q7 {
    fn get_question_number(&self) -> u8 {
        7
    }
    fn solve_part_one(&self, _path: Option<&str>) -> i32 {
        -1
    }
}

impl Q7 {
    #[allow(dead_code)]
    pub fn custom_solve_part_one(&self, path: Option<&str>) -> u64 {
        let mut input = self.get_input();
        match path {
            Some(path) => input = self.get_custom_input(path),
            None => {}
        }

        let mut ans = 0;

        for line in input.lines() {
            let target = line.split(":").nth(0).unwrap().parse::<u64>().unwrap();
            let nums: Vec<u64> = line
                .split(":")
                .nth(1)
                .unwrap()
                .split_whitespace()
                .map(|x| x.parse::<u64>().unwrap())
                .collect();

            if solve(1, nums[0], target, &nums) {
                ans += target;
            }
        }

        ans
    }
    pub fn custom_solve_part_two(&self, path: Option<&str>) -> u64 {
        let mut input = self.get_input();
        match path {
            Some(path) => input = self.get_custom_input(path),
            None => {}
        }

        let mut ans = 0;

        for line in input.lines() {
            let target = line.split(":").nth(0).unwrap().parse::<u64>().unwrap();
            let nums: Vec<u64> = line
                .split(":")
                .nth(1)
                .unwrap()
                .split_whitespace()
                .map(|x| x.parse::<u64>().unwrap())
                .collect();

            if solve_two(1, nums[0], target, &nums) {
                ans += target;
            }
        }

        ans
    }
}

fn concat(a: u64, b: u64) -> u64 {
    a * 10u64.pow(b.ilog10() + 1) + b
}

fn solve_two(i: usize, curr: u64, tar: u64, nums: &Vec<u64>) -> bool {
    if i == nums.len() && curr == tar {
        return true;
    }

    if i >= nums.len() {
        return false;
    }

    solve_two(i + 1, curr * nums[i], tar, nums)
        || solve_two(i + 1, curr + nums[i], tar, nums)
        || solve_two(i + 1, concat(curr, nums[i]), tar, nums)
}

fn solve(i: usize, curr: u64, tar: u64, nums: &Vec<u64>) -> bool {
    if i == nums.len() && curr == tar {
        return true;
    }

    if i >= nums.len() {
        return false;
    }

    solve(i + 1, curr * nums[i], tar, nums) || solve(i + 1, curr + nums[i], tar, nums)
}
