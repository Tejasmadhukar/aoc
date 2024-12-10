use std::{collections::HashSet, usize};

use crate::solution::Solution;

pub struct Q8 {}

struct Node {
    frequency: char,
    position: (i32, i32),
}

impl Node {
    fn new(freq: char, pos: (i32, i32)) -> Node {
        Node {
            frequency: freq,
            position: pos,
        }
    }
}

impl Solution for Q8 {
    fn get_question_number(&self) -> u8 {
        8
    }
    fn solve_part_one(&self, path: Option<&str>) -> i32 {
        let mut input = self.get_input();
        match path {
            Some(path) => input = self.get_custom_input(path),
            None => {}
        }

        let n = input.lines().count();
        let m = input.lines().nth(0).unwrap().chars().count();

        let nodes: Vec<Node> = input
            .lines()
            .enumerate()
            .flat_map(|(j, line)| {
                line.chars().enumerate().filter_map(move |(i, c)| {
                    if c == '.' {
                        None
                    } else {
                        Some(Node::new(c, (i as i32, j as i32)))
                    }
                })
            })
            .collect();

        let mut antinodes: HashSet<(i32, i32)> = HashSet::new();

        for i in 1..nodes.len() {
            for j in 0..i {
                if nodes[i].frequency != nodes[j].frequency {
                    continue;
                }
                let n1 = (
                    2 * nodes[j].position.0 - nodes[i].position.0,
                    2 * nodes[j].position.1 - nodes[i].position.1,
                );
                let n2 = (
                    2 * nodes[i].position.0 - nodes[j].position.0,
                    2 * nodes[i].position.1 - nodes[j].position.1,
                );

                if (0..m).contains(&(n1.0 as usize)) && (0..n).contains(&(n1.1 as usize)) {
                    antinodes.insert(n1);
                }

                if (0..m).contains(&(n2.0 as usize)) && (0..n).contains(&(n2.1 as usize)) {
                    antinodes.insert(n2);
                }
            }
        }

        antinodes.len() as i32
    }
    fn solve_part_two(&self, _path: Option<&str>) -> i32 {
        let mut input = self.get_input();
        match _path {
            Some(path) => input = self.get_custom_input(path),
            None => {}
        }

        let n = input.lines().count();
        let m = input.lines().nth(0).unwrap().chars().count();

        let nodes: Vec<Node> = input
            .lines()
            .enumerate()
            .flat_map(|(j, line)| {
                line.chars().enumerate().filter_map(move |(i, c)| {
                    if c == '.' {
                        None
                    } else {
                        Some(Node::new(c, (i as i32, j as i32)))
                    }
                })
            })
            .collect();

        let mut antinodes: HashSet<(i32, i32)> = HashSet::new();

        for i in 1..nodes.len() {
            for j in 0..i {
                if nodes[i].frequency != nodes[j].frequency {
                    continue;
                }

                let dx = nodes[i].position.0 - nodes[j].position.0;
                let dy = nodes[i].position.1 - nodes[j].position.1;

                let mut k = 1;
                loop {
                    let n1 = (nodes[i].position.0 - dx * k, nodes[i].position.1 - dy * k);
                    let n2 = (nodes[j].position.0 + k * dx, nodes[j].position.1 + k * dy);

                    let check_bounds = |pos: (i32, i32)| -> bool {
                        (0..m).contains(&(pos.0 as usize)) && (0..n).contains(&(pos.1 as usize))
                    };

                    if check_bounds(n1) {
                        antinodes.insert(n1);
                    }

                    if check_bounds(n2) {
                        antinodes.insert(n2);
                    }

                    if !check_bounds(n1) && !check_bounds(n2) {
                        break;
                    }

                    k += 1
                }
            }
        }

        antinodes.len() as i32
    }
}
