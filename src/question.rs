use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

// NOTE: for future use
pub struct Question {
    question_input: io::Result<io::Lines<io::BufReader<File>>>,
}

impl Question {
    pub fn new<P>(filename: P) -> Result<Question,io::Error>
    where P: AsRef<Path>
    {
        let file = File::open(filename)?;
        Ok(Question {
            question_input: Ok(io::BufReader::new(file).lines())
        })
    }
}

pub trait Solution {
    fn part_one(&self) -> String;
    fn part_two(&self) -> String;
}
