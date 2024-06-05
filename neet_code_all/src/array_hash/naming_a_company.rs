use std::collections::HashMap;
use std::collections::HashSet;

// ルール
// 1: ideasの中から二個idea_a,idea_b選ぶ
// 2: 二個の文字列の最初の文字をスワップする
// 3: 2の操作で得た文字列が両方ともideasの中になければ  idea_a + ' ' + idea_b は有効な名前
// 有効な名前の数を返せ

// 解けなかった
struct Solution;
impl Solution {
    // O(N^2)でTLE
    pub fn distinct_names_sq(ideas: Vec<String>) -> i64 {
        let ideas = ideas
            .iter()
            .map(|s| s.chars().collect::<Vec<char>>())
            .collect::<Vec<Vec<char>>>();
        let mut set_idea = HashSet::new();
        for s in &ideas {
            set_idea.insert(s);
        }

        let mut result = 0;
        for i in 0..ideas.len() {
            for j in (i + 1)..ideas.len() {
                let mut s_1 = ideas[i].clone();
                let mut s_2 = ideas[j].clone();

                let tmp_char = s_1[0];
                s_1[0] = s_2[0];
                s_2[0] = tmp_char;

                if !set_idea.contains(&s_1) && !set_idea.contains(&s_2) {
                    result += 2;
                }
            }
        }

        result
    }

    // だめ
    pub fn distinct_names(ideas: Vec<String>) -> i64 {
        let mut map_idea: HashMap<String, Vec<String>> = HashMap::new();
        for s in &ideas {
            if s.len() > 1 {
                if let Some(map_v) = map_idea.get_mut(&s[1..]) {
                    map_v.push(s.to_string());
                } else {
                    map_idea.insert(s[1..].to_string(), vec![s.to_string()]);
                }
            } else {
                if let Some(map_v) = map_idea.get_mut(s) {
                    map_v.push(s.to_string());
                } else {
                    map_idea.insert(s.to_string(), vec![s.to_string()]);
                }
            }
        }

        let values: Vec<&Vec<String>> = map_idea.values().collect();
        println!("valus: {:?}", values);
        let mut result = 0;
        for i in 0..values.len() {
            for j in (i + 1)..values.len() {
                result += values[i].len() * values[j].len();
            }
        }

        result as i64
    }
}

struct SolutionAnsKotlin;
impl SolutionAnsKotlin {
    // AC
    pub fn distinct_names(ideas: Vec<String>) -> i64 {
        let ideas = ideas
            .iter()
            .map(|s| s.as_bytes().to_vec())
            .collect::<Vec<Vec<u8>>>();

        let mut first_to_suffix = vec![HashSet::new(); 26];
        for s in ideas {
            first_to_suffix[(s[0] - b'a') as usize].insert(s[1..].to_vec());
        }

        let mut result = 0;
        for i in 0..26 {
            for j in i..26 {
                let mut common = 0;
                for word_a in &first_to_suffix[i] {
                    if first_to_suffix[j].contains(word_a) {
                        common += 1;
                    }
                }

                let (map_i, map_j) = (
                    first_to_suffix[i].len() - common,
                    first_to_suffix[j].len() - common,
                );
                result += (map_i * map_j) * 2;
            }
        }

        result as i64
    }
}

fn main() {
    let case_1 = vec!["coffee", "donuts", "time", "toffee"]
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>();
    // => 6
    let case_2 = vec!["lack", "back"]
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>();
    // => 0

    /*
    println!("case_1: {}", Solution::distinct_names_sq(case_1.clone()));
    println!("case_2: {}", Solution::distinct_names_sq(case_2.clone()));

    println!("case_1: {}", Solution::distinct_names(case_1.clone()));
    println!("case_2: {}", Solution::distinct_names(case_2.clone()));
    */

    println!(
        "case_1: {}",
        SolutionAnsKotlin::distinct_names(case_1.clone())
    );
    println!(
        "case_2: {}",
        SolutionAnsKotlin::distinct_names(case_2.clone())
    );
}
