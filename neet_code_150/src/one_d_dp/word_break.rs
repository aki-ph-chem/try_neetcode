use std::collections::HashSet;

// 解けなかった
struct Solution {}
impl Solution {
    pub fn word_break(s: String, word_dict: Vec<String>) -> bool {
        true
    }

    // とりあえず全探索してみる
    // case_4に対応できない
    pub fn word_break_ex(s: String, word_dict: Vec<String>) -> bool {
        let n = s.len();

        if n == 1 && word_dict.len() == 1 {
            return s == word_dict[0];
        }

        let mut set_ans = HashSet::new();
        for v in &word_dict {
            set_ans.insert(v);
        }
        let mut set = HashSet::new();

        let mut i = 0;
        while i < n {
            for b in (i + 1)..n {
                for v in &word_dict {
                    if &s[i..=b] == v {
                        set.insert(v);
                        i = b + 1;
                    }
                }
            }
            i += 1;
        }

        set_ans == set
    }
}

// 模範解答
// C++の模範解答より
struct SolutionAns {}
impl SolutionAns {
    pub fn word_break(s: String, word_dict: Vec<String>) -> bool {
        // wordリストをHashSetで作成する
        let mut words = HashSet::new();
        for v in &word_dict {
            words.insert(v);
        }

        let n = s.len() as i32;
        // dp[i]: i - 1 までの真偽
        let mut dp = vec![false; n as usize + 1];
        dp[0] = true;

        for i in 0..=n {
            for k in (0..=(i - 1)).rev() {
                if dp[k as usize] {
                    if words.contains(&s[k as usize..i as usize].to_string()) {
                        dp[i as usize] = true;
                        break;
                    }
                }
            }
        }

        dp[n as usize]
    }
}

fn main() {
    let case_1 = (
        "leetcode".to_string(),
        vec!["leet".to_string(), "code".to_string()],
    );
    // true
    let case_2 = (
        "applepenapple".to_string(),
        vec!["apple".to_string(), "pen".to_string()],
    );
    // true
    let case_3 = (
        "catsandog".to_string(),
        vec![
            "cats".to_string(),
            "dog".to_string(),
            "sand".to_string(),
            "and".to_string(),
            "cat".to_string(),
        ],
    );
    // false
    let case_4 = ("ab".to_string(), vec!["a".to_string(), "b".to_string()]);

    /*
    println!(
        "case_1: {}",
        Solution::word_break_ex(case_1.0.clone(), case_1.1.clone())
    );
    println!(
        "case_2: {}",
        Solution::word_break_ex(case_2.0.clone(), case_2.1.clone())
    );
    println!(
        "case_3: {}",
        Solution::word_break_ex(case_3.0.clone(), case_3.1.clone())
    );
    println!(
        "case_4: {}",
        Solution::word_break_ex(case_4.0.clone(), case_4.1.clone())
    );
    */

    println!(
        "case_1: {}",
        SolutionAns::word_break(case_1.0.clone(), case_1.1.clone())
    );
    println!(
        "case_2: {}",
        SolutionAns::word_break(case_2.0.clone(), case_2.1.clone())
    );
    println!(
        "case_3: {}",
        SolutionAns::word_break(case_3.0.clone(), case_3.1.clone())
    );
    println!(
        "case_4: {}",
        SolutionAns::word_break(case_4.0.clone(), case_4.1.clone())
    );
}
