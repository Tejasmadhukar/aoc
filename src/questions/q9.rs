use std::{collections::BTreeMap, usize};

use crate::solution::Solution;

pub struct Q9 {}

impl Solution for Q9 {
    fn get_question_number(&self) -> u8 {
        9
    }
    fn solve_part_one(&self, _path: Option<&str>) -> i32 {
        -1
    }
}

impl Q9 {
    #[allow(dead_code)]
    pub fn custom_part_one(&self, path: Option<&str>) -> u64 {
        let mut input = self.get_input();
        match path {
            Some(path) => input = self.get_custom_input(path),
            None => {}
        }

        let mut disk_map: Vec<Option<u32>> = Vec::new();

        let mut k = 0;
        let mut flip = true;
        for i in input.trim().chars() {
            let x = i.to_digit(10).unwrap();

            if flip {
                let mut n: Vec<Option<u32>> = vec![Some(k); x as usize];
                disk_map.append(&mut n);
                k += 1;
            } else {
                let mut n: Vec<Option<u32>> = vec![None; x as usize];
                disk_map.append(&mut n);
            }

            flip = !flip;
        }

        while disk_map.contains(&None) {
            let l = disk_map.pop().unwrap();

            if l.is_none() {
                continue;
            }

            if let Some(i) = disk_map.iter_mut().find(|x| x.is_none()) {
                *i = l;
            }
        }

        disk_map
            .iter()
            .enumerate()
            .map(|(idx, e)| idx as u64 * e.unwrap() as u64)
            .sum()
    }

    #[allow(dead_code)]
    pub fn custom_part_two(&self, path: Option<&str>) -> u64 {
        let mut input = self.get_input();
        match path {
            Some(path) => input = self.get_custom_input(path),
            None => {}
        }

        let mut disk_map: Vec<Option<u32>> = Vec::new();
        let mut file_count: BTreeMap<u32, (u32, u32)> = BTreeMap::new();

        let mut k = 0;
        let mut l = 0;
        let mut flip = true;
        for i in input.trim().chars() {
            let x = i.to_digit(10).unwrap();

            if flip {
                let mut n: Vec<Option<u32>> = vec![Some(k); x as usize];
                file_count.insert(k, (l, l + x - 1));
                l += x;
                disk_map.append(&mut n);
                k += 1;
            } else {
                let mut n: Vec<Option<u32>> = vec![None; x as usize];
                l += x;
                disk_map.append(&mut n);
            }

            flip = !flip;
        }

        for (&k, &v) in file_count.iter().rev() {
            let r = v.1;
            let window = (v.1 - v.0 + 1) as usize;

            if let Some(s) = disk_map
                .windows(window)
                .position(|x| x == vec![None; window])
            {
                if s < r as usize {
                    for i in s..(s + window) {
                        disk_map[i] = Some(k);
                    }
                    for i in (v.0 as usize)..=(v.1 as usize) {
                        disk_map[i] = None;
                    }
                }
            }
        }

        disk_map
            .iter()
            .enumerate()
            .map(|(idx, e)| {
                if e.is_none() {
                    return 0;
                }

                idx as u64 * e.unwrap() as u64
            })
            .sum()
    }
}
