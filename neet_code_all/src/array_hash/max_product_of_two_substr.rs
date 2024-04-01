// 解けなかった
struct Solution {}
impl Solution {
    pub fn max_product(s: String) -> i32 {
        let s: Vec<char> = s.chars().collect();
        let mut n = s.len();

        let mut len_list = vec![];
        for bit in 0..(1 << n) {
            let mut substr = vec![];
            for i in 0..n {
                if bit & (1 << i) != 0 {
                    substr.push(s[i]);
                }
            }
            if Self::is_palindromic(&substr) {
                len_list.push(substr.len());
            }
        }
        len_list.sort();

        let mut num = 2;
        let mut result = 1;
        for v in len_list.iter().rev() {
            if num > 0 {
                if *v < n {
                    result *= v;
                    num -= 1;
                }
            } else {
                break;
            }
        }

        result as i32
    }

    fn is_palindromic(s: &Vec<char>) -> bool {
        let (mut left, mut right) = (0 as i32, s.len() as i32 - 1);
        while left <= right {
            if s[left as usize] != s[right as usize] {
                return false;
            }
            left += 1;
            right -= 1;
        }

        true
    }
}

// 模範解答
struct SolutionAns {}
impl SolutionAns {
    pub fn max_product(s: String) -> i32 {
        let s: Vec<char> = s.chars().collect();
        let (mut s_1, mut s_2) = (vec![], vec![]);
        let mut res = 0;

        SolutionAns::dfs(&s, &mut s_1, &mut s_2, &mut res, 0);

        res
    }

    fn dfs(s: &[char], s_1: &mut Vec<char>, s_2: &mut Vec<char>, res: &mut i32, idx: usize) {
        if idx == s.len() {
            if SolutionAns::is_palindrome(s_1) && SolutionAns::is_palindrome(s_2) {
                let new_max = s_1.len() * s_2.len();
                *res = std::cmp::max(*res, new_max as i32);
            }

            return;
        }

        SolutionAns::dfs(s, s_1, s_2, res, idx + 1);

        s_1.push(s[idx]);
        SolutionAns::dfs(s, s_1, s_2, res, idx + 1);
        s_1.pop();

        s_2.push(s[idx]);
        SolutionAns::dfs(s, s_1, s_2, res, idx + 1);
        s_2.pop();
    }

    fn is_palindrome(s: &[char]) -> bool {
        if s.len() <= 1 {
            return true;
        }

        let (mut left, mut right) = (0, s.len() - 1);
        while left < right {
            if s[left] != s[right] {
                return false;
            }

            left += 1;
            right -= 1;
        }

        true
    }
}

fn main() {
    let case_1 = "leetcodecom".to_string();
    // => 9: (ete, cdc)
    let case_2 = "bb".to_string();
    // => 1
    let case_3 = "accbcaxxcxx".to_string();
    // => 25

    println!("case_1: {}", Solution::max_product(case_1.clone()));
    println!("case_2: {}", Solution::max_product(case_2.clone()));
    println!("case_3: {}", Solution::max_product(case_3.clone()));

    println!("case_1: {}", SolutionAns::max_product(case_1.clone()));
    println!("case_2: {}", SolutionAns::max_product(case_2.clone()));
    println!("case_3: {}", SolutionAns::max_product(case_3.clone()));
}
