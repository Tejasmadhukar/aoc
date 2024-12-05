use std::fs;

pub trait Solution {
    fn get_question_number(&self) -> u8;
    fn get_input(&self) -> String {
        let nu = self.get_question_number();
        let input_file = format!("input/day{}.txt", nu);
        fs::read_to_string(input_file)
            .expect(&format!("Could not read file at input/day{}.txt", nu))
    }
    fn get_custom_input(&self, path: &str) -> String {
        fs::read_to_string(path).expect(&format!("Could not read file at {}", path))
    }
    fn solve_part_one(&self, path: Option<&str>) -> i32;
    fn solve_part_two(&self, _path: Option<&str>) -> i32 {
        -1
    }
}
