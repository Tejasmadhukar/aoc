use std::collections::{HashMap, HashSet};

use crate::solution::Solution;

pub struct Q5 {}

impl Solution for Q5 {
    fn get_question_number(&self) -> u8 {
        5
    }
    fn solve_part_one(&self, path: Option<&str>) -> i32 {
        let mut input = self.get_input();
        match path {
            Some(path) => input = self.get_custom_input(path),
            None => {}
        }

        let rule_input = input.split("\n\n").nth(0).unwrap();
        let update_input = input.split("\n\n").nth(1).unwrap();

        let mut rules: HashMap<u32, HashSet<u32>> = HashMap::new();

        for line in rule_input.split("\n") {
            let k = line.split("|").nth(0).unwrap().parse::<u32>().unwrap();
            let v = line.split("|").nth(1).unwrap().parse::<u32>().unwrap();
            rules.entry(k).or_insert_with(HashSet::new).insert(v);
        }

        let mut ans = 0;

        for line in update_input.split("\n") {
            let updates: Vec<u32> = line.split(",").map(|x| x.parse::<u32>().unwrap()).collect();
            ans += check_updates(&updates, &rules);
        }

        ans.try_into().unwrap()
    }
}

fn check_updates(updates: &Vec<u32>, rules: &HashMap<u32, HashSet<u32>>) -> u32 {
    for i in 0..updates.len() {
        let curr = updates[i];
        match rules.get(&curr) {
            Some(check) => {
                for j in 0..i {
                    if check.get(&updates[j]).is_some() {
                        return 0;
                    }
                }
            }
            None => continue,
        }
    }

    updates[updates.len() / 2]
}
