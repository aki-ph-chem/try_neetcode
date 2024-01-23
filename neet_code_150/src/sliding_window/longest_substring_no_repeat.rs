use std::collections::{HashSet, VecDeque};

struct Solution {}
impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        if s.len() == 0 {
            return 0;
        }
        let mut set = HashSet::new();
        let mut len_list = vec![];
        let mut len_tmp = 0;
        for c in s.chars() {
            if !set.contains(&c) {
                set.insert(c);
                len_tmp += 1;
            } else {
                len_list.push(len_tmp);
                len_tmp = 1;
                set.clear();
                set.insert(c);
            }
        }
        len_list.push(len_tmp);

        println!("len_list: {:?}", len_list);
        *len_list.iter().max().unwrap()
    }

    // AC : O(N^2)
    pub fn length_of_longest_substring_sq(s: String) -> i32 {
        if s.len() == 0 {
            return 0;
        }
        let s_vec: Vec<char> = s.chars().collect();

        let mut set = HashSet::new();
        let mut len_list = vec![];
        let mut len_tmp = 1;

        for i in 0..s_vec.len() {
            set.insert(s_vec[i]);
            for j in (i + 1)..s_vec.len() {
                if !set.contains(&s_vec[j]) {
                    set.insert(s_vec[j]);
                    len_tmp += 1;
                } else {
                    break;
                }
            }
            len_list.push(len_tmp);
            set.clear();
            len_tmp = 1;
        }

        println!("len_list: {:?}", len_list);
        *len_list.iter().max().unwrap()
    }
}

// 模範解答
struct SolutionAns {}
impl SolutionAns {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut set: VecDeque<char> = VecDeque::new();
        let mut longest = 0;

        for c in s.chars() {
            while set.contains(&c) {
                set.pop_front();
            }

            set.push_back(c);
            longest = longest.max(set.len());
        }

        longest as i32
    }
}

// AC
// C++の模範解答より
struct SolutionAnsCpp {}
impl SolutionAnsCpp {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let vec_char:Vec<char> = s.chars().collect();
        let mut set:HashSet<char> = HashSet::new();
        let mut result = 0;
        let (mut i, mut j) = (0, 0);

        while j < vec_char.len() {
            while set.contains(&vec_char[j]) {
                set.remove(&vec_char[i]);
                i += 1;
            }

            result = result.max(j - i + 1);
            set.insert(vec_char[j]);
            j += 1;
        }

        result as i32
    }
}

fn main() {
    let case_1 = "abcabcbb".to_string();
    let case_2 = "bbbbb".to_string();
    let case_3 = "pwwkew".to_string();
    let case_4 = "xyz".to_string();
    let case_5 = "au".to_string();
    let case_6 = "a".to_string();
    let case_7 = "dvdf".to_string();

    println!("lenght_of_longest_substring");
    println!(
        "case_1: {}",
        Solution::length_of_longest_substring(case_1.clone())
    );
    println!(
        "case_2: {}",
        Solution::length_of_longest_substring(case_2.clone())
    );
    println!(
        "case_3: {}",
        Solution::length_of_longest_substring(case_3.clone())
    );
    println!(
        "case_4: {}",
        Solution::length_of_longest_substring(case_4.clone())
    );
    println!(
        "case_5: {}",
        Solution::length_of_longest_substring(case_5.clone())
    );
    println!(
        "case_6: {}",
        Solution::length_of_longest_substring(case_6.clone())
    );
    println!(
        "case_7: {}",
        Solution::length_of_longest_substring(case_7.clone())
    );

    println!("lenght_of_longest_substring_sq");
    println!(
        "case_1: {}",
        Solution::length_of_longest_substring_sq(case_1.clone())
    );
    println!(
        "case_2: {}",
        Solution::length_of_longest_substring_sq(case_2.clone())
    );
    println!(
        "case_3: {}",
        Solution::length_of_longest_substring_sq(case_3.clone())
    );
    println!(
        "case_4: {}",
        Solution::length_of_longest_substring_sq(case_4.clone())
    );
    println!(
        "case_5: {}",
        Solution::length_of_longest_substring_sq(case_5.clone())
    );
    println!(
        "case_6: {}",
        Solution::length_of_longest_substring_sq(case_6.clone())
    );
    println!(
        "case_7: {}",
        Solution::length_of_longest_substring_sq(case_7.clone())
    );

    println!("模範解答");
    println!(
        "case_1: {}",
        SolutionAns::length_of_longest_substring(case_1.clone())
    );
    println!(
        "case_2: {}",
        SolutionAns::length_of_longest_substring(case_2.clone())
    );
    println!(
        "case_3: {}",
        SolutionAns::length_of_longest_substring(case_3.clone())
    );
    println!(
        "case_4: {}",
        SolutionAns::length_of_longest_substring(case_4.clone())
    );
    println!(
        "case_5: {}",
        SolutionAns::length_of_longest_substring(case_5.clone())
    );
    println!(
        "case_6: {}",
        SolutionAns::length_of_longest_substring(case_6.clone())
    );
    println!(
        "case_7: {}",
        SolutionAns::length_of_longest_substring(case_7.clone())
    );

    println!("C++の模範解答より");
    println!(
        "case_1: {}",
        SolutionAnsCpp::length_of_longest_substring(case_1.clone())
    );
    println!(
        "case_2: {}",
        SolutionAnsCpp::length_of_longest_substring(case_2.clone())
    );
    println!(
        "case_3: {}",
        SolutionAnsCpp::length_of_longest_substring(case_3.clone())
    );
    println!(
        "case_4: {}",
        SolutionAnsCpp::length_of_longest_substring(case_4.clone())
    );
    println!(
        "case_5: {}",
        SolutionAnsCpp::length_of_longest_substring(case_5.clone())
    );
    println!(
        "case_6: {}",
        SolutionAnsCpp::length_of_longest_substring(case_6.clone())
    );
    println!(
        "case_7: {}",
        SolutionAnsCpp::length_of_longest_substring(case_7.clone())
    );
}
