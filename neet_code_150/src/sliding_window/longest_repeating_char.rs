use std::collections::{HashMap, VecDeque};

// できなかった😂
struct Solution {}
impl Solution {
    pub fn character_replacement(s: String, k: i32) -> i32 {
        let max_rep = k;
        let mut set: VecDeque<char> = VecDeque::new();

        3
    }
}

// 模範解答
struct SolutionAns {}
impl SolutionAns {
  pub fn character_replacement(s: String, k: i32) -> i32 {
        let s: Vec<char> = s.chars().collect();
        let (mut res, mut l, mut maxf) = (0, 0, 0);
        let mut count: HashMap<char, u64> = HashMap::new();

        for r in 0..s.len() {
            *count.entry(s[r]).or_default() += 1;
            maxf = maxf.max(*count.get(&s[r]).unwrap());

            while (r - l + 1) - maxf as usize > k as usize {
                *count.get_mut(&s[l]).unwrap() -= 1;
                l += 1;
            }

            res = res.max(r - l + 1);
        }

        res as i32
    }
}


fn main() {
    let case_1 = ("ABAB".to_string(), 2);
    let case_2 = ("AABABBA".to_string(), 1);
    println!("case_1: {}", SolutionAns::character_replacement(case_1.0, case_1.1));
    println!("case_2: {}", SolutionAns::character_replacement(case_2.0, case_2.1));
}
