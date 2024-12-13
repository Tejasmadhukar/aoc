use std::{
    collections::{HashMap, HashSet},
    usize,
};

use crate::solution::Solution;

pub struct Q12 {}

impl Solution for Q12 {
    fn get_question_number(&self) -> u8 {
        12
    }
    fn solve_part_one(&self, path: Option<&str>) -> i32 {
        let mut input = self.get_input();
        match path {
            Some(path) => input = self.get_custom_input(path),
            None => {}
        }

        let garden_plot: Vec<Vec<char>> =
            input.lines().map(|line| line.chars().collect()).collect();

        let mut plant_plot: HashMap<(usize, usize), usize> = HashMap::new();

        let mut k = 0;
        for i in 0..garden_plot.len() {
            for j in 0..garden_plot[0].len() {
                if !plant_plot.contains_key(&(i, j)) {
                    dfs(i, j, garden_plot[i][j], k, &garden_plot, &mut plant_plot);
                    k += 1;
                }
            }
        }

        let plant_plot = plant_plot
            .into_iter()
            .fold(HashMap::new(), |mut acc, (k, v)| {
                acc.entry(v).or_insert_with(HashSet::new).insert(k);
                acc
            });

        let directions: Vec<(isize, isize)> = vec![(0, -1), (-1, 0), (0, 1), (1, 0)];

        let mut plant_perimeter: HashMap<usize, usize> = HashMap::new();

        for (k, v) in plant_plot.iter() {
            let mut perimeter = 0;
            for c in v.iter() {
                for dir in directions.iter() {
                    let nx = c.0 as isize + dir.0;
                    let ny = c.1 as isize + dir.1;

                    if !(0..garden_plot[0].len()).contains(&(ny as usize))
                        || !(0..garden_plot.len()).contains(&(nx as usize))
                    {
                        perimeter += 1;
                        continue;
                    }

                    if !v.contains(&(nx as usize, ny as usize)) {
                        perimeter += 1;
                    }
                }
            }
            plant_perimeter.insert(*k, perimeter);
        }

        plant_plot
            .iter()
            .map(|(k, v)| v.len() * plant_perimeter.get(k).unwrap())
            .sum::<usize>() as i32
    }
}

fn dfs(
    i: usize,
    j: usize,
    target: char,
    curr: usize,
    garden_plot: &Vec<Vec<char>>,
    plant_plot: &mut HashMap<(usize, usize), usize>,
) {
    let directions: Vec<(isize, isize)> = vec![(-1, 0), (0, 1), (1, 0), (0, -1)];

    if plant_plot.contains_key(&(i, j)) {
        return;
    }

    if garden_plot[i][j] == target {
        plant_plot.insert((i, j), curr);
    }

    for dir in directions {
        let nx = i as isize + dir.0;
        let ny = j as isize + dir.1;

        if !(0..garden_plot[0].len()).contains(&(ny as usize))
            || !(0..garden_plot.len()).contains(&(nx as usize))
        {
            continue;
        }

        if garden_plot[nx as usize][ny as usize] == target {
            dfs(
                nx as usize,
                ny as usize,
                target,
                curr,
                garden_plot,
                plant_plot,
            );
        }
    }
}
