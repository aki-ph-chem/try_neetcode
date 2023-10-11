use std::collections::HashMap;

// 自分の回答
struct Solution {}
impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        let mut s_sorted: Vec<char> = s.chars().collect();
        let mut t_sorted: Vec<char> = t.chars().collect();
        s_sorted.sort();
        t_sorted.sort();

        s_sorted == t_sorted
    }
}

// 模範解答
struct SolutionAns {}
impl SolutionAns {
    pub fn is_anagram(s: String, t: String) -> bool {
        if t.len() != s.len() {
            return false;
        }

        let mut map: HashMap<char, i64> = HashMap::new();

        for (a, b) in s.chars().zip(t.chars()) {
            *map.entry(a).or_default() += 1;
            *map.entry(b).or_default() -= 1;
        }

        map.into_values().all(|cnt| cnt == 0)
    }
}

fn main() {
    let s_1 = "anagram".to_string();
    let t_1 = "nagaram".to_string();
    println!("1");
    if Solution::is_anagram(s_1, t_1) {
        println!("anagram");
    }

    let s_2 = "rat".to_string();
    let t_2 = "car".to_string();
    println!("2");
    if Solution::is_anagram(s_2, t_2) {
        println!("anagram");
    }

    let s_1 = "anagram".to_string();
    let t_1 = "nagaram".to_string();
    println!("1");
    if SolutionAns::is_anagram(s_1, t_1) {
        println!("anagram");
    }

    let s_2 = "rat".to_string();
    let t_2 = "car".to_string();
    println!("2");
    if SolutionAns::is_anagram(s_2, t_2) {
        println!("anagram");
    }
}
