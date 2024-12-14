use regex::Regex;

use crate::solution::Solution;

pub struct Q13 {}

impl Solution for Q13 {
    fn get_question_number(&self) -> u8 {
        13
    }
    fn solve_part_one(&self, path: Option<&str>) -> i32 {
        let mut input = self.get_input();
        match path {
            Some(path) => input = self.get_custom_input(path),
            None => {}
        }

        let mut ans = 0;
        for block in input.split("\n\n") {
            let mut coeff_matrix: Vec<usize> = Vec::new();
            let mut res_matrix: Vec<usize> = Vec::new();

            let mut k = 0;
            for line in block.lines() {
                let re = Regex::new(r"(?<number>[0-9]{1,9})").unwrap();

                if k == 2 {
                    for (_, [number]) in re.captures_iter(line).map(|c| c.extract()) {
                        res_matrix.push(number.parse::<usize>().unwrap());
                    }
                    break;
                }

                for (_, [number]) in re.captures_iter(line).map(|c| c.extract()) {
                    coeff_matrix.push(number.parse::<usize>().unwrap());
                }
                k += 1;
            }

            let denom = calculate_deteminant(&coeff_matrix);
            if denom == 0 {
                continue;
            }

            let a = {
                let mut calc = coeff_matrix.clone();
                calc[0] = res_matrix[0];
                calc[1] = res_matrix[1];
                calculate_deteminant(&calc)
            };

            let b = {
                let mut calc = coeff_matrix.clone();
                calc[2] = res_matrix[0];
                calc[3] = res_matrix[1];
                calculate_deteminant(&calc)
            };

            if a % denom != 0 || b % denom != 0 {
                continue;
            }

            ans += 3 * (a / denom) + (b / denom);
        }

        ans as i32
    }
}


fn calculate_deteminant(matrix: &Vec<usize>) -> isize {
    (matrix[0] * matrix[3]) as isize - (matrix[1] * matrix[2]) as isize
}
