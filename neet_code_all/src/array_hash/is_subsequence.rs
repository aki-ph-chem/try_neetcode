use std::collections::HashMap;

// 初見では解けなかった
struct Solution {}
impl Solution {
    // whileループでs,tを同時に探索するアイディアまではok
    pub fn is_subsequence(s: String, t: String) -> bool {
        let t_vec: Vec<char> = t.chars().collect();
        let s_vec: Vec<char> = s.chars().collect();

        let (mut t_l, mut s_l) = (0, 0);
        let mut c_list = vec![];

        while t_l < t_vec.len() && s_l < s_vec.len() {
            while t_l < t_vec.len() && s_vec[s_l] != t_vec[t_l] {
                t_l += 1;
            }

            c_list.push(s_vec[s_l]);
            s_l += 1;
        }

        c_list == s_vec
    }

    // まったくだめ
    pub fn is_subsequence_2(s: String, t: String) -> bool {
        let t_vec: Vec<char> = t.chars().collect();
        let s_vec: Vec<char> = s.chars().collect();

        let mut t_l = 0;
        for s_l in 0..s_vec.len() {
            while t_l < t_vec.len() && s_vec[s_l] != t_vec[t_l] {
                t_l += 1;
            }
        }

        true
    }

    // 模範解答を参考にして、while in whileで書くとしたらこう書く
    // AC
    pub fn is_subsequence_rev(s: String, t: String) -> bool {
        let t_vec: Vec<char> = t.chars().collect();
        let s_vec: Vec<char> = s.chars().collect();

        let (mut t_l, mut s_l) = (0, 0);

        while s_l < s_vec.len() && t_l < t_vec.len() {
            while t_l < t_vec.len() && s_vec[s_l] != t_vec[t_l] {
                t_l += 1;
            }

            if t_l < t_vec.len() && s_vec[s_l] == t_vec[t_l] {
                t_l += 1;
                s_l += 1;
            }
        }

        s_l == s_vec.len()
    }
}

// 模範解答
struct SolutionAns {}
impl SolutionAns {
    pub fn is_subsequence(s: String, t: String) -> bool {
        let s: Vec<char> = s.chars().collect();
        let t: Vec<char> = t.chars().collect();

        let (mut l, mut r) = (0, 0);
        while l < s.len() && r < t.len() {
            if s[l] == t[r] {
                l += 1;
                r += 1;
            } else {
                r += 1;
            }
        }

        l == s.len()
    }
}

//　時間を置いて解いたときの
struct SolutionLatter;
impl SolutionLatter {
    pub fn is_subsequence(s: String, t: String) -> bool {
        if s.len() > t.len() {
            return false;
        }

        let s = s.chars().collect::<Vec<char>>();
        let mut i_s = 0;
        for c_t in t.chars() {
            if i_s < s.len() {
                if s[i_s] == c_t {
                    i_s += 1;
                }
            }
        }

        i_s == s.len()
    }
}

// AC
// C++の模範解答より
struct SolutionAnsCpp {}
impl SolutionAnsCpp {
    pub fn is_subsequence(s: String, t: String) -> bool {
        let s: Vec<char> = s.chars().collect();
        let t: Vec<char> = t.chars().collect();

        let (mut i, mut j) = (0, 0);
        while i < s.len() && j < t.len() {
            if s[i] == t[j] {
                i += 1;
            }

            j += 1;
        }

        if i >= s.len() {
            return true;
        }

        false
    }
}

fn main() {
    let case_1 = ("abc".to_string(), "ahbgdc".to_string());
    // => true
    let case_2 = ("axc".to_string(), "ahbgdc".to_string());
    // => false

    /*
    println!(
        "case_1: {}",
        Solution::is_subsequence(case_1.0.clone(), case_1.1.clone())
    );
    println!(
        "case_2: {}",
        Solution::is_subsequence(case_2.0.clone(), case_2.1.clone())
    );
    */

    println!(
        "case_1: {}",
        SolutionAns::is_subsequence(case_1.0.clone(), case_1.1.clone())
    );
    println!(
        "case_2: {}",
        SolutionAns::is_subsequence(case_2.0.clone(), case_2.1.clone())
    );

    println!(
        "case_1: {}",
        Solution::is_subsequence_rev(case_1.0.clone(), case_1.1.clone())
    );
    println!(
        "case_2: {}",
        Solution::is_subsequence_rev(case_2.0.clone(), case_2.1.clone())
    );

    println!(
        "case_1: {}",
        SolutionAnsCpp::is_subsequence(case_1.0.clone(), case_1.1.clone())
    );
    println!(
        "case_2: {}",
        SolutionAnsCpp::is_subsequence(case_2.0.clone(), case_2.1.clone())
    );

    println!(
        "case_1: {}",
        SolutionLatter::is_subsequence(case_1.0.clone(), case_1.1.clone())
    );
    println!(
        "case_2: {}",
        SolutionLatter::is_subsequence(case_2.0.clone(), case_2.1.clone())
    );
}
