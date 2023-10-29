use std::collections::HashSet;

#[derive(Debug)]
struct Solution {}

impl Solution {
    pub fn is_valid(s: String) -> bool {
        let vec_char: Vec<char> = s.chars().collect();

        let mut number_maru = 0; 
        let mut number_nami = 0; 
        let mut number_kaku = 0; 

        for c in vec_char {
            match c {
                '(' => {number_maru += 1},
                ')' => {number_maru -= 1},
                '{' => {number_nami += 1},
                '}' => {number_nami -= 1},
                '[' => {number_kaku += 1},
                ']' => {number_kaku -= 1},
                _ => {
                    return false;
                }
            }
        }

        number_maru == 0 && number_nami == 0 && number_kaku == 0 }
}

pub fn is_valid_maru(s: &str) -> bool {
    let vec_char: Vec<char> = s.chars().collect();
    let mut number = 0;

    for c in vec_char {
        if c == '(' {
            number += 1;
        } else {
            number -= 1;
        }
    }

    number == 0
}

fn main() {
    let case_1 = "()".to_string();
    let case_2 = "()[]{}".to_string();
    let case_3 = "(]".to_string();
    let case_4 = "([)]".to_string();

    println!("case_1: {}", Solution::is_valid(case_1.clone())); // true
    println!("case_2: {}", Solution::is_valid(case_2.clone())); // false 
    println!("case_3: {}", Solution::is_valid(case_3.clone())); // false 
    println!("case_4: {}", Solution::is_valid(case_4.clone())); // false

    /*
    println!("()(): {}", is_valid_maru("()()"));
    println!("()): {}", is_valid_maru("())"));
    println!("()(: {}", is_valid_maru("()("));
    println!("(()): {}", is_valid_maru("(())"));
    println!("(()()): {}", is_valid_maru("(()())"));
    println!("())()): {}", is_valid_maru("())())"));
    */
}
