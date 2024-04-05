use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn process_string(s: &String, result: &mut u32) {
    let mut first_digit: Option<u32> = None;
    let mut last_digit: Option<u32> = None;
    let mut last_digit_idx = 0;

    for (i, word) in s.chars().enumerate() {
        match word.to_digit(10) {
            Some(number) => {
                if first_digit.is_none() {
                    first_digit = Some(number);
                    last_digit = first_digit;
                    last_digit_idx = i;
                }
                if i > last_digit_idx {
                    last_digit = Some(number);
                    last_digit_idx = i;
                }
            }
            None => continue,
        }
    }

    *result += first_digit.unwrap_or(0) * 10 + last_digit.unwrap_or(0);
}

pub fn solve_q1() {
    let mut result: u32 = 0;

    if let Ok(lines) = read_lines("./input/q1.txt") {
        for line in lines.flatten() {
            process_string(&line, &mut result)
        }
    }
    println!("{}", result)
}
