use std::collections::{HashMap, HashSet};

// 解けなかった😂
struct Solution {}
impl Solution {
    pub fn check_inclusion_ex(s1: String, s2: String) -> bool {
        // s_1の構成
        let mut set_s1: HashMap<char, usize> = HashMap::new();
        for c in s1.chars() {
            *set_s1.entry(c).or_default() += 1;
        }

        true
    }
}

// 模範解答
struct SolutionAns {}
impl SolutionAns {
    pub fn check_inclusion(s1: String, s2: String) -> bool {
        if s1.len() > s2.len() {
            return false;
        }

        let (mut s1_cnt, mut s2_cnt) = ([0; 26], [0; 26]);
        for i in 0..s1.len() {
            s1_cnt[s1.chars().nth(i).unwrap() as usize - 'a' as usize] += 1;
            s2_cnt[s2.chars().nth(i).unwrap() as usize - 'a' as usize] += 1;
        }

        let mut matches = 0;
        for i in 0..26 {
            matches = if s1_cnt[i] == s2_cnt[i] {
                matches + 1
            } else {
                matches
            };
        }

        let mut l = 0;
        for r in s1.len()..s2.len() {
            if matches == 26 {
                return true;
            }

            let mut index = s2.chars().nth(r).unwrap() as usize - 'a' as usize;
            s2_cnt[index] += 1;
            if s1_cnt[index] == s2_cnt[index] {
                matches += 1;
            } else if s1_cnt[index] + 1 == s2_cnt[index] {
                matches -= 1;
            }

            index = s2.chars().nth(l).unwrap() as usize - 'a' as usize;
            s2_cnt[index] -= 1;
            if s1_cnt[index] == s2_cnt[index] {
                matches += 1;
            } else if s1_cnt[index] - 1 == s2_cnt[index] {
                matches -= 1;
            }

            l += 1;
        }

        matches == 26
    }
}

fn main() {
    let case_1 = ("ab".to_string(), "eidbaooo".to_string());
    let case_2 = ("ab".to_string(), "eidboaoo".to_string());

    println!("case_1: {}", SolutionAns::check_inclusion(case_1.0, case_1.1));
    println!("case_2: {}", SolutionAns::check_inclusion(case_2.0, case_2.1));
}
