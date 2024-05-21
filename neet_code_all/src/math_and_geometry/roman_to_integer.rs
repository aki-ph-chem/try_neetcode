use std::collections::HashMap;

// 解けなかった
struct Solution;
impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        let s = s.chars().collect::<Vec<char>>();
        let map = HashMap::from([
            ('I', 1),
            ('V', 5),
            ('X', 10),
            ('L', 50),
            ('C', 100),
            ('D', 500),
            ('M', 1000),
        ]);
        let mut result = 0;
        for c in s {
            result += map[&c];
        }

        result
    }
}

// 模範解答
struct SolutionAns;
impl SolutionAns {
    pub fn roman_to_int(s: String) -> i32 {
        let s = s.chars().collect::<Vec<char>>();
        let mut result = 0;
        /*
        let to_value = |c| {
            let map = HashMap::from([
                ('I', 1),
                ('V', 5),
                ('X', 10),
                ('L', 50),
                ('C', 100),
                ('D', 500),
                ('M', 1000),
            ]);
            if let Some(map_key) = map.get(&c) {
                *map_key
            } else {
                0
            }
        };
        */
        let to_value_match = |c| match c {
            'I' => 1,
            'V' => 5,
            'X' => 10,
            'L' => 50,
            'C' => 100,
            'D' => 500,
            'M' => 1000,
            _ => 0,
        };

        for i in 0..s.len() {
            if i + 1 < s.len() && to_value_match(s[i]) < to_value_match(s[i + 1]) {
                result -= to_value_match(s[i]);
            } else {
                result += to_value_match(s[i]);
            }
        }

        result
    }
}

fn main() {
    let case_1 = "III".to_string();
    // => 3
    let case_2 = "LVIII".to_string();
    // => 58
    let case_3 = "MCMXCIV".to_string();
    // => 1994

    /*
    println!("case_1: {}", Solution::roman_to_int(case_1.clone()));
    println!("case_2: {}", Solution::roman_to_int(case_2.clone()));
    println!("case_3: {}", Solution::roman_to_int(case_3.clone()));
    */

    println!("case_1: {}", SolutionAns::roman_to_int(case_1.clone()));
    println!("case_2: {}", SolutionAns::roman_to_int(case_2.clone()));
    println!("case_3: {}", SolutionAns::roman_to_int(case_3.clone()));
}
