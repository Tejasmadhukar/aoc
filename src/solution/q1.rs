use crate::question;
use std::collections::HashMap;

fn process_string(s: &String, result: &mut u32, digits_map: &HashMap<&str, u32>) {
    let mut first_digit: Option<u32> = None;
    let mut last_digit: Option<u32> = None;
    let mut last_digit_idx = 0;
    let mut first_digit_idx = s.len() - 1;

    for (d, val) in digits_map.iter() {
        for i in s.match_indices(d) {
            if i.0 <= first_digit_idx {
                first_digit = Some(*val);
                first_digit_idx = i.0;
                if last_digit.is_none() {
                    last_digit = first_digit;
                    last_digit_idx = first_digit_idx;
                }
            }
            if i.0 >= last_digit_idx {
                last_digit = Some(*val);
                last_digit_idx = i.0;
            }
        }
    }

    for (i, word) in s.chars().enumerate() {
        match word.to_digit(10) {
            Some(number) => {
                if i <= first_digit_idx {
                    first_digit = Some(number);
                    first_digit_idx = i;
                    if last_digit.is_none() {
                        last_digit = Some(number);
                        last_digit_idx = i;
                    }
                }
                if i >= last_digit_idx {
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
    let mut digits: HashMap<&str, u32> = HashMap::new();
    let mut result: u32 = 0;

    digits.insert("one", 1);
    digits.insert("two", 2);
    digits.insert("three", 3);
    digits.insert("four", 4);
    digits.insert("five", 5);
    digits.insert("six", 6);
    digits.insert("seven", 7);
    digits.insert("eight", 8);
    digits.insert("nine", 9);

    if let Ok(lines) = question::read_lines("input/day1.txt"){
        for line in lines.flatten() {
            process_string(&line, &mut result, &digits)
        }
    }
    println!("{}", result)
}
