use std::collections::HashMap;

struct Solution {}
impl Solution {
    // AC
    pub fn word_patern(pattern: String, s: String) -> bool {
        let words: Vec<&str> = s.split(' ').collect();
        if words.len() != pattern.len() {
            return false;
        }

        let mut map_pattern: HashMap<char, Vec<i32>> = HashMap::new();
        let mut map_s: HashMap<&str, Vec<i32>> = HashMap::new();

        for ((i, c), word) in pattern.chars().enumerate().zip(words.iter()) {
            if let Some(map_pattern_key) = map_pattern.get_mut(&c) {
                map_pattern_key.push(i as i32);
            } else {
                map_pattern.insert(c, vec![i as i32]);
            }

            if let Some(map_s_key) = map_s.get_mut(word) {
                map_s_key.push(i as i32);
            } else {
                map_s.insert(word, vec![i as i32]);
            }

            if let (Some(map_s_key), Some(map_pattern_key)) = (map_s.get(word), map_pattern.get(&c))
            {
                if map_pattern_key != map_s_key {
                    return false;
                }
            }
        }

        true
    }
}

// 模範解答
struct SolutionAns {}
impl SolutionAns {
    pub fn word_patern(pattern: String, s: String) -> bool {
        if pattern.len() != s.split_whitespace().count() {
            return false;
        }

        let mut char_to_word = HashMap::new();
        let mut word_to_char = HashMap::new();

        for (c, word) in pattern.chars().zip(s.split_whitespace()) {
            if let Some(w) = char_to_word.insert(c, word) {
                if w != word {
                    return false;
                }
            }

            if let Some(w) = word_to_char.insert(word, c) {
                if w != c {
                    return false;
                }
            }
        }

        true
    }
}

fn main() {
    let case_1 = ("abba".to_string(), "dog cat cat dog".to_string());
    // => true
    let case_2 = ("abba".to_string(), "dog cat cat ".to_string());
    // => false
    let case_3 = ("aaaa".to_string(), "dog cat cat dog".to_string());
    // => false

    println!(
        "case_1: {}",
        Solution::word_patern(case_1.0.clone(), case_1.1.clone())
    );
    println!(
        "case_2: {}",
        Solution::word_patern(case_2.0.clone(), case_2.1.clone())
    );
    println!(
        "case_3: {}",
        Solution::word_patern(case_3.0.clone(), case_3.1.clone())
    );

    println!(
        "case_1: {}",
        SolutionAns::word_patern(case_1.0.clone(), case_1.1.clone())
    );
    println!(
        "case_2: {}",
        SolutionAns::word_patern(case_2.0.clone(), case_2.1.clone())
    );
    println!(
        "case_3: {}",
        SolutionAns::word_patern(case_3.0.clone(), case_3.1.clone())
    );
}
