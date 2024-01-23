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

// C++の模範解答より
struct SolutionAnsCpp {}
impl SolutionAnsCpp {
    pub fn character_replacement(s: String, k: i32) -> i32 {
        let vec_char: Vec<char> = s.chars().collect();
        let mut count = vec![0; 26];
        let mut max_count = 0;

        let (mut i, mut j) = (0, 0);
        let mut result = 0;

        while j < vec_char.len() {
            count[vec_char[j] as usize - 'A' as usize] += 1;
            max_count = max_count.max(count[vec_char[j] as usize - 'A' as usize]);

            if (j - i + 1) as i32 - max_count > k {
                count[vec_char[i] as usize - 'A' as usize] -= 1;
                i += 1;
            }

            result = result.max((j - i + 1) as i32);
            j += 1;
        }

        result
    }
}

fn main() {
    let case_1 = ("ABAB".to_string(), 2);
    let case_2 = ("AABABBA".to_string(), 1);

    println!(
        "case_1: {}",
        SolutionAns::character_replacement(case_1.0.clone(), case_1.1)
    );
    println!(
        "case_2: {}",
        SolutionAns::character_replacement(case_2.0.clone(), case_2.1)
    );

    println!(
        "case_1: {}",
        SolutionAnsCpp::character_replacement(case_1.0.clone(), case_1.1)
    );
    println!(
        "case_2: {}",
        SolutionAnsCpp::character_replacement(case_2.0.clone(), case_2.1)
    );
}
