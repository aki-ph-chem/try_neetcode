use std::collections::HashMap;
use std::collections::HashSet;

// 解けなかった
struct Solution {}
impl Solution {
    pub fn count_palindromic_subsequence(s: String) -> i32 {
        let s: Vec<char> = s.chars().collect();
        let mut map: HashMap<char, Vec<usize>> = HashMap::new();

        for (idx, c) in s.iter().enumerate() {
            if let Some(map_key) = map.get_mut(c) {
                map_key.push(idx);
            } else {
                map.insert(*c, vec![idx]);
            }
        }
        //println!("map: {:?}", map);

        1
    }
}

// 模範解答
struct SolutionAns {}
impl SolutionAns {
    pub fn count_palindromic_subsequence(s: String) -> i32 {
        let mut result = 0;
        let mut ranges: [(i32, i32); 26] = [(-1, -1); 26];

        for (idx_1, c) in s.chars().enumerate() {
            let idx_2 = SolutionAns::char_to_idx(c);
            if ranges[idx_2].0 == -1 {
                ranges[idx_2].0 = idx_1 as i32;
            }

            if idx_1 as i32 > ranges[idx_2].1 {
                ranges[idx_2].1 = idx_1 as i32;
            }
        }

        for range in ranges {
            if range.1 > range.0 {
                let mut set: u32 = 0;
                for c in s[range.0 as usize + 1..range.1 as usize].chars() {
                    set |= 1 << SolutionAns::char_to_idx(c);
                }
                result += set.count_ones() as i32;
            }
        }

        result
    }

    pub fn char_to_idx(c: char) -> usize {
        c as usize - 'a' as usize
    }
}

// AC
// C++の模範解答より
// 三文字の回分はそもそもXYXの形式しか存在しないことに着目する
struct SolutionAnsCpp {}
impl SolutionAnsCpp {
    pub fn count_palindromic_subsequence(s: String) -> i32 {
        let mut map: HashMap<char, (i32, i32)> = HashMap::new();
        let s: Vec<char> = s.chars().collect();

        for idx in 0..s.len() {
            if let Some(map_key) = map.get_mut(&s[idx]) {
                map_key.1 = idx as i32;
            } else {
                map.insert(s[idx], (idx as i32, -1));
            }
        }

        let mut result = 0;
        for (_c, (idx_1, idx_2)) in map {
            if idx_2 != -1 {
                let mut tmp: HashSet<char> = HashSet::new();
                for j in idx_1 + 1..idx_2 {
                    tmp.insert(s[j as usize]);
                }

                result += tmp.len();
            }
        }

        result as i32
    }
}

fn main() {
    let case_1 = "aabca".to_string();
    // => 3: (aba, aaa, aca)
    let case_2 = "adc".to_string();
    // => 0
    let case_3 = "bbcbaba".to_string();
    // => 4

    println!(
        "case_1: {}",
        SolutionAnsCpp::count_palindromic_subsequence(case_1.clone())
    );
    println!(
        "case_2: {}",
        SolutionAnsCpp::count_palindromic_subsequence(case_2.clone())
    );
    println!(
        "case_3: {}",
        SolutionAnsCpp::count_palindromic_subsequence(case_3.clone())
    );

    println!(
        "case_1: {}",
        SolutionAns::count_palindromic_subsequence(case_1.clone())
    );
    println!(
        "case_2: {}",
        SolutionAns::count_palindromic_subsequence(case_2.clone())
    );
    println!(
        "case_3: {}",
        SolutionAns::count_palindromic_subsequence(case_3.clone())
    );
}
