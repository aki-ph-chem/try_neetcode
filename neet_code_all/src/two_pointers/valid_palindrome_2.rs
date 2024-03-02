// 解けなかった
// 文字削除 -> 添字をスキップの考え方はok
struct Solution {}
impl Solution {
    pub fn valid_palindrome(s: String) -> bool {
        let len = s.len();
        let s: Vec<char> = s.chars().collect();

        let (mut l, mut r) = (0_i32, len as i32 - 1);
        let mut is_skiped = false;
        while l < r {
            if s[l as usize] == s[r as usize] {
                l += 1;
                r -= 1;
            } else if is_skiped {
                return false;
            } else {
                if s[l as usize + 1] == s[r as usize] {
                    //println!("l: {} -> {}", l, l + 1);
                    l += 1;
                } else if s[l as usize] == s[r as usize - 1] {
                    //println!("r: {} -> {}", r, r - 1);
                    r -= 1;
                }
                is_skiped = true;
            }
        }

        true
    }
}

// 模範解答
struct SolutionAns {}
impl SolutionAns {
    pub fn valid_palindrome(s: String) -> bool {
        let s = s.as_bytes().to_vec();
        match Self::is_palindrome(&s) {
            Some((i, j)) => {
                if Self::is_palindrome(&s[i + 1..=j]).is_none() {
                    return true;
                }

                if Self::is_palindrome(&s[i..=j - 1]).is_none() {
                    return true;
                }
            }

            None => {
                return true;
            }
        }

        false
    }

    fn is_palindrome(s: &[u8]) -> Option<(usize, usize)> {
        let (mut i, mut j) = (0, s.len() - 1);
        while i < j {
            if s[i] != s[j] {
                return Some((i, j));
            }

            i += 1;
            j -= 1;
        }

        None
    }
}

// C++の模範解答より
struct SolutionAnsCpp {}
impl SolutionAnsCpp {
    pub fn valid_palindrome(s: String) -> bool {
        let len = s.len();
        let s: Vec<char> = s.chars().collect();

        let (mut l, mut r) = (0_i32, len as i32 - 1);
        while l < r {
            if s[l as usize] == s[r as usize] {
                l += 1;
                r -= 1;
            } else {
                return Self::valid_palindrome_utl(&s, l + 1, r)
                    || Self::valid_palindrome_utl(&s, l, r - 1);
            }
        }

        true
    }

    fn valid_palindrome_utl(s: &Vec<char>, mut l: i32, mut r: i32) -> bool {
        while l < r {
            if s[l as usize] == s[r as usize] {
                l += 1;
                r -= 1;
            } else {
                return false;
            }
        }

        true
    }
}

fn main() {
    let case_1 = "aba".to_string();
    // => true
    let case_2 = "abca".to_string();
    // => true
    let case_3 = "abc".to_string();
    // => false
    let case_4 = "cbbcc".to_string();
    // => true
    let case_5 = "xdddbababeccebababddd".to_string();
    // => true

    println!("case_1: {}", Solution::valid_palindrome(case_1.clone()));
    println!("case_2: {}", Solution::valid_palindrome(case_2.clone()));
    println!("case_3: {}", Solution::valid_palindrome(case_3.clone()));
    println!("case_4: {}", Solution::valid_palindrome(case_4.clone()));
    println!("case_5: {}", Solution::valid_palindrome(case_5.clone()));

    println!(
        "case_1: {}",
        SolutionAnsCpp::valid_palindrome(case_1.clone())
    );
    println!(
        "case_2: {}",
        SolutionAnsCpp::valid_palindrome(case_2.clone())
    );
    println!(
        "case_3: {}",
        SolutionAnsCpp::valid_palindrome(case_3.clone())
    );
    println!(
        "case_4: {}",
        SolutionAnsCpp::valid_palindrome(case_4.clone())
    );
    println!(
        "case_5: {}",
        SolutionAnsCpp::valid_palindrome(case_5.clone())
    );

    println!("case_1: {}", SolutionAns::valid_palindrome(case_1.clone()));
    println!("case_2: {}", SolutionAns::valid_palindrome(case_2.clone()));
    println!("case_3: {}", SolutionAns::valid_palindrome(case_3.clone()));
    println!("case_4: {}", SolutionAns::valid_palindrome(case_4.clone()));
    println!("case_5: {}", SolutionAns::valid_palindrome(case_5.clone()));
}
