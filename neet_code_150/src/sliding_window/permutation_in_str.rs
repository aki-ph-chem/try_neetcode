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

struct SolutionAnsCpp {}
impl SolutionAnsCpp {
    // AC
    pub fn check_inclusion(s1: String, s2: String) -> bool {
        let (m, n) = (s1.len(), s2.len());
        if m > n {
            return false;
        }

        let vec_char_1: Vec<char> = s1.chars().collect();
        let vec_char_2: Vec<char> = s2.chars().collect();

        let mut count_map: HashMap<char, i32> = HashMap::new();
        // 0 ~ m - 1 文字目まで
        for i in 0..m {
            *count_map.entry(vec_char_1[i]).or_default() += 1;
            *count_map.entry(vec_char_2[i]).or_default() -= 1;
        }
        if Self::is_permutation(&count_map) {
            return true;
        }

        // m ~ n - 1文字目まで
        for i in m..n {
            *count_map.entry(vec_char_2[i]).or_default() -= 1;
            // i - m: 0 ~ n - 1 - m
            *count_map.entry(vec_char_2[i - m]).or_default() += 1;
            if Self::is_permutation(&count_map) {
                return true;
            }
        }

        false
    }

    // AC
    pub fn check_inclusion_byte(s1: String, s2: String) -> bool {
        let (m, n) = (s1.len(), s2.len());
        if m > n {
            return false;
        }

        let vec_char_1: Vec<u8> = s1.bytes().collect();
        let vec_char_2: Vec<u8> = s2.bytes().collect();

        let mut count_map: HashMap<u8, i32> = HashMap::new();
        // 0 ~ m - 1 文字目まで
        for i in 0..m {
            *count_map.entry(vec_char_1[i]).or_default() += 1;
            *count_map.entry(vec_char_2[i]).or_default() -= 1;
        }
        if Self::is_permutation_byte(&count_map) {
            return true;
        }

        // m ~ n - 1文字目まで
        for i in m..n {
            *count_map.entry(vec_char_2[i]).or_default() -= 1;
            // i - m: 0 ~ n - 1 - m
            *count_map.entry(vec_char_2[i - m]).or_default() += 1;
            if Self::is_permutation_byte(&count_map) {
                return true;
            }
        }

        false
    }

    // AC
    // .chars().nth(i)を呼ぶと毎回O(i)かかるため遅くなる
    pub fn check_inclusion_2(s1: String, s2: String) -> bool {
        let (m, n) = (s1.len(), s2.len());
        if m > n {
            return false;
        }

        let mut count_map: HashMap<char, i32> = HashMap::new();
        // 0 ~ m - 1 文字目まで
        for i in 0..m {
            *count_map.entry(s1.chars().nth(i).unwrap()).or_default() += 1;
            *count_map.entry(s2.chars().nth(i).unwrap()).or_default() -= 1;
        }
        if Self::is_permutation(&count_map) {
            return true;
        }

        // m ~ n - 1文字目まで
        for i in m..n {
            *count_map.entry(s2.chars().nth(i).unwrap()).or_default() -= 1;
            // i - m: 0 ~ n - m
            *count_map.entry(s2.chars().nth(i - m).unwrap()).or_default() += 1;
            if Self::is_permutation(&count_map) {
                return true;
            }
        }

        false
    }

    // AC
    // .chars().nth(i)を排除したため速くなった
    pub fn check_inclusion_3(s1: String, s2: String) -> bool {
        let (m, n) = (s1.len(), s2.len());
        if m > n {
            return false;
        }

        let mut count_map: HashMap<char, i32> = HashMap::new();
        for (a, b) in s1.chars().zip(s2.chars().take(m)) {
            *count_map.entry(a).or_default() += 1;
            *count_map.entry(b).or_default() -= 1;
        }

        if Self::is_permutation(&count_map) {
            return true;
        }

        // m ~ n - 1文字目まで
        for (b_1, b_2) in s2.chars().skip(m).zip(s2.chars().take(n - m)) {
            *count_map.entry(b_1).or_default() -= 1;
            // i - m: 0 ~ n - 1 - m
            *count_map.entry(b_2).or_default() += 1;
            if Self::is_permutation(&count_map) {
                return true;
            }
        }

        false
    }

    fn is_permutation(map: &HashMap<char, i32>) -> bool {
        for (_i, x) in map {
            if *x != 0 {
                return false;
            }
        }

        true
    }

    fn is_permutation_byte(map: &HashMap<u8, i32>) -> bool {
        for (_i, x) in map {
            if *x != 0 {
                return false;
            }
        }

        true
    }
}

fn main() {
    let case_1 = ("ab".to_string(), "eidbaooo".to_string());
    let case_2 = ("ab".to_string(), "eidboaoo".to_string());

    println!(
        "case_1: {}",
        SolutionAns::check_inclusion(case_1.0.clone(), case_1.1.clone())
    );
    println!(
        "case_2: {}",
        SolutionAns::check_inclusion(case_2.0.clone(), case_2.1.clone())
    );

    println!(
        "case_1: {}",
        SolutionAnsCpp::check_inclusion(case_1.0.clone(), case_1.1.clone())
    );
    println!(
        "case_2: {}",
        SolutionAnsCpp::check_inclusion(case_2.0.clone(), case_2.1.clone())
    );

    println!(
        "case_1: {}",
        SolutionAnsCpp::check_inclusion_2(case_1.0.clone(), case_1.1.clone())
    );
    println!(
        "case_2: {}",
        SolutionAnsCpp::check_inclusion_2(case_2.0.clone(), case_2.1.clone())
    );

    println!(
        "case_1: {}",
        SolutionAnsCpp::check_inclusion_3(case_1.0.clone(), case_1.1.clone())
    );
    println!(
        "case_2: {}",
        SolutionAnsCpp::check_inclusion_3(case_2.0.clone(), case_2.1.clone())
    );

    println!(
        "case_1: {}",
        SolutionAnsCpp::check_inclusion_byte(case_1.0.clone(), case_1.1.clone())
    );
    println!(
        "case_2: {}",
        SolutionAnsCpp::check_inclusion_byte(case_2.0.clone(), case_2.1.clone())
    );
}
