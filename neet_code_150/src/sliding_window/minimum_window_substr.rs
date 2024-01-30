use std::collections::HashMap;

// 解けなかった
// panicを取り除けなかった
struct Solution {}
impl Solution {
    pub fn min_window(s: String, t: String) -> String {
        let (m, n) = (s.len(), t.len());
        if m < n {
            return "".to_string();
        }

        let mut map: HashMap<char, i32> = HashMap::new();
        for c in t.chars() {
            *map.entry(c).or_default() += 1;
        }

        let (mut i, mut j) = (0, 0);
        let vec_char: Vec<char> = s.chars().collect();
        let mut res_len = 1 << 30;

        while j < m {
            let mut map_t = map.clone();
            while !Self::is_map_all_0(&map_t) {
                if map_t.contains_key(&vec_char[i]) {
                    *map_t.entry(vec_char[i]).or_default() -= 1;
                }
                i += 1;
            }

            res_len = res_len.min(j - i + 1);
            j += 1;
        }

        println!("{},{}", i, j);
        s[i..j].to_string()
        //"foo".to_string()
    }

    fn is_map_all_0(map: &HashMap<char, i32>) -> bool {
        for (_c, n) in map {
            if *n != 0 {
                return false;
            }
        }

        true
    }
}

// C++の模範解答より
struct SolutionAnsCpp {}
impl SolutionAnsCpp {
    pub fn min_window(s: String, t: String) -> String {
        let mut map: HashMap<char, i32> = HashMap::new();
        for c in t.chars() {
            *map.entry(c).or_default() += 1;
        }

        let vec_char: Vec<char> = s.chars().collect();

        let (mut counter, mut min_start, mut min_len) = (t.len(), 0, i32::MAX as usize);
        let (mut i, mut j) = (0, 0);
        while j < s.len() {
            if let Some(n) = map.get(&vec_char[j]) {
                if *n > 0 {
                    counter -= 1;
                }
            }

            *map.entry(vec_char[j]).or_default() -= 1;
            j += 1;

            while counter == 0 {
                if j - i < min_len {
                    min_start = i;
                    min_len = j - i;
                }

                *map.entry(vec_char[i]).or_default() += 1;
                if let Some(n) = map.get(&vec_char[i]) {
                    if *n > 0 {
                        counter += 1;
                    }
                }
                i += 1;
            }
        }

        if min_len != i32::MAX as usize {
            return s[min_start..min_len].to_string()
        }

        "".to_string()
    }
}

fn main() {
    let case_1 = ("ADOBECODEBANC".to_string(), "ABC".to_string());
    // => "BANC"
    let case_2 = ("a".to_string(), "a".to_string());
    // => "a"
    let case_3 = ("a".to_string(), "aa".to_string());
    // => ""

    //println!("case_1: {}", Solution::min_window(case_1.0.clone(), case_1.1.clone()));

    println!("case_1: {}", SolutionAnsCpp::min_window(case_1.0.clone(), case_1.1.clone()));
    println!("case_2: {}", SolutionAnsCpp::min_window(case_2.0.clone(), case_2.1.clone()));
    println!("case_3: {}", SolutionAnsCpp::min_window(case_3.0.clone(), case_3.1.clone()));
}
