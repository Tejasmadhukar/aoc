use std::collections::{HashMap, HashSet, VecDeque};

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
    fn solve_part_two(&self, _path: Option<&str>) -> i32 {
        let mut input = self.get_input();
        match _path {
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
            let check = check_updates(&updates, &rules);
            if check == 0 {
                ans += correct_update(&updates, &rules);
            }
        }

        ans.try_into().unwrap()
    }
}

fn topological_sort(graph: &HashMap<u32, HashSet<u32>>) -> Vec<u32> {
    let mut in_degree: HashMap<u32, u32> = HashMap::new();
    for (k, v) in graph {
        in_degree.entry(*k).or_insert(0);
        for val in v {
            *in_degree.entry(*val).or_insert(0) += 1;
        }
    }

    let mut queue: VecDeque<u32> = in_degree
        .iter()
        .filter(|&(_, &deg)| deg == 0)
        .map(|(&node, _)| node)
        .collect();

    let mut correct: Vec<u32> = Vec::new();
    while let Some(node) = queue.pop_front() {
        correct.push(node);
        if let Some(v) = graph.get(&node) {
            for k in v {
                if let Some(deg) = in_degree.get_mut(k) {
                    *deg -= 1;
                    if *deg == 0 {
                        queue.push_back(*k);
                    }
                }
            }
        }
    }

    if correct.len() != in_degree.len() {
        panic!("cycle");
    }

    correct
}

fn correct_update(updates: &Vec<u32>, rules: &HashMap<u32, HashSet<u32>>) -> u32 {
    let curr_update: HashSet<u32> = updates.iter().cloned().collect();

    let mut sub_rules: HashMap<u32, HashSet<u32>> = HashMap::new();

    for (k, v) in rules {
        if curr_update.contains(k) {
            let filtered_nodes: HashSet<u32> = v
                .iter()
                .filter(|val| curr_update.contains(val))
                .cloned()
                .collect();
            sub_rules.insert(*k, filtered_nodes);
        }
    }

    //topo sort here then result, return a vector ?
    let updated = topological_sort(&sub_rules);
    updated[updated.len() / 2]
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
