use std::collections::HashMap;

struct Solution {}
impl Solution {
    // AC
    pub fn max_number_of_ballons(text: String) -> i32 {
        let mut map: HashMap<char, i32> = HashMap::new();
        for c in text.chars() {
            *map.entry(c).or_default() += 1;
        }

        let mut map_ballon: HashMap<char, i32> = HashMap::new();
        for c in "balloon".chars() {
            *map_ballon.entry(c).or_default() += 1;
        }

        let mut result = i32::MAX;
        for (c, n) in map_ballon {
            if let Some(map_key) = map.get(&c) {
                result = result.min(map_key / n);
            } else {
                return 0;
            }
        }

        result
    }
}

// 模範解答
struct SolutionAns {}
impl SolutionAns {
    pub fn max_number_of_ballons(text: String) -> i32 {
        let mut count_text = HashMap::new();
        let mut balloon = HashMap::new();
        let mut result = text.len() as i32;

        for ch in text.chars() {
            *count_text.entry(ch).or_insert(0) += 1;
        }
        for ch in "balloon".chars() {
            *balloon.entry(ch).or_insert(0) += 1;
        }

        for ch in balloon.keys() {
            result = result.min(count_text.get(ch).unwrap_or(&0) / balloon.get(ch).unwrap());
        }

        result
    }
}

fn main() {
    let case_1 = "nlaebolko".to_string();
    // => 1
    let case_2 = "loonbalxballpoon".to_string();
    // => 2
    let case_3 = "leetcode".to_string();
    // => 0
    let case_4 = "lloo".to_string();
    // => 0

    println!(
        "case_1: {}",
        Solution::max_number_of_ballons(case_1.clone())
    );
    println!(
        "case_2: {}",
        Solution::max_number_of_ballons(case_2.clone())
    );
    println!(
        "case_3: {}",
        Solution::max_number_of_ballons(case_3.clone())
    );
    println!(
        "case_4: {}",
        Solution::max_number_of_ballons(case_4.clone())
    );

    println!(
        "case_1: {}",
        SolutionAns::max_number_of_ballons(case_1.clone())
    );
    println!(
        "case_2: {}",
        SolutionAns::max_number_of_ballons(case_2.clone())
    );
    println!(
        "case_3: {}",
        SolutionAns::max_number_of_ballons(case_3.clone())
    );
    println!(
        "case_4: {}",
        SolutionAns::max_number_of_ballons(case_4.clone())
    );
}
