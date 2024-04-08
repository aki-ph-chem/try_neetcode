use std::collections::HashMap;

// 解けなかった
struct Solution {}
impl Solution {
    pub fn is_alien_sorted(words: Vec<String>, order: String) -> bool {
        let mut len_min = usize::MAX;
        let words: Vec<Vec<char>> = words
            .iter()
            .map(|x| {
                len_min = len_min.min(x.len());
                x.chars().collect()
            })
            .collect();

        let mut map: HashMap<char, usize> = HashMap::new();
        for (idx, c) in order.chars().enumerate() {
            map.insert(c, idx);
        }

        let mut idx = 0;
        for j in 0..len_min {
            for i in 1..words.len() {
                /*
                print!(
                    "c_0, n_c_0: {}, {} ",
                    words[i - 1][idx],
                    map[&words[i - 1][idx]]
                );
                println!("c_1, n_c_1: {}, {}", words[i][idx], map[&words[i][idx]]);
                */

                if map[&words[i - 1][j]] > map[&words[i][j]] {
                    return false;
                } else if map[&words[i - 1][j]] == map[&words[i][j]] {
                    idx += 1;
                }

                if j == idx {
                    return true;
                }
            }
        }

        true
    }
}

// 模範解答
struct SolutionAns {}
impl SolutionAns {
    pub fn is_alien_sorted(words: Vec<String>, order: String) -> bool {
        let n = words.len();
        let mut map = HashMap::new();

        order.as_bytes().iter().enumerate().for_each(|(i, &v)| {
            map.insert(v, i);
        });

        for i in 1..n {
            let s_1 = words[i - 1].as_bytes();
            let s_2 = words[i].as_bytes();

            let mut is_equal = true;

            for j in 0..s_1.len().min(s_2.len()) {
                if map.get(&s_1[j]) > map.get(&s_2[j]) {
                    return false;
                }

                if map.get(&s_1[j]) < map.get(&s_2[j]) {
                    is_equal = false;
                    break;
                }
            }

            if is_equal && s_1.len() > s_2.len() {
                return false;
            }
        }

        true
    }
}

fn main() {
    let case_1 = (
        vec!["hello".to_string(), "leetcode".to_string()],
        "hlabcdefgijkmnopqrstuvwxyz".to_string(),
    );
    // => true
    let case_2 = (
        vec!["word".to_string(), "world".to_string(), "row".to_string()],
        "worldabcefghijkmnpqstuvxyz".to_string(),
    );
    // => false
    let case_3 = (
        vec!["apple".to_string(), "app".to_string()],
        "abcdefghijklmnopqrstuvwxyz".to_string(),
    );
    // => false

    /*
    println!(
        "case_1: {}",
        Solution::is_alien_sorted(case_1.0.clone(), case_1.1.clone())
    );
    println!(
        "case_2: {}",
        Solution::is_alien_sorted(case_2.0.clone(), case_2.1.clone())
    );
    println!(
        "case_3: {}",
        Solution::is_alien_sorted(case_3.0.clone(), case_3.1.clone())
    );
    */

    println!(
        "case_1: {}",
        SolutionAns::is_alien_sorted(case_1.0.clone(), case_1.1.clone())
    );
    println!(
        "case_2: {}",
        SolutionAns::is_alien_sorted(case_2.0.clone(), case_2.1.clone())
    );
    println!(
        "case_3: {}",
        SolutionAns::is_alien_sorted(case_3.0.clone(), case_3.1.clone())
    );
}
