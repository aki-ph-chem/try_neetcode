// 初見では解けなかった
struct Solution {}
impl Solution {
    pub fn count_substrings(s: String) -> i32 {
        let n = s.len() as i32;
        let vec_char: Vec<char> = s.chars().collect();

        if n == 1 {
            return 1;
        }

        let mut result = 0;
        for i in 0..n {
            // n is odd
            let (mut l, mut r) = (i, i);
            while l >= 0 && r < n && vec_char[l as usize] == vec_char[r as usize] {
                l -= 1;
                r += 1;
            }

            // n is even
            let (mut l, mut r) = (i, i + 1);
            while l >= 0 && r < n && vec_char[l as usize] == vec_char[r as usize] {
                l -= 1;
                r += 1;
            }
        }

        result
    }
}

// 模範解答
struct SolutionAns {}
impl SolutionAns {
    pub fn count_substring(s: String) -> i32 {
        let s = s.chars().collect::<Vec<char>>();
        let (mut count, length): (i32, i32) = (0, s.len() as i32);

        for i in 0..length {

            // odd length
            let (mut l, mut r) = (i, i);
            while l >= 0 && r < length && s[l as usize] == s[r as usize] {
                count += 1;
                l -= 1;
                r += 1;
            }

            // even length
            let (mut l, mut r) = (i, i + 1);
            while l >= 0 && r < length && s[l as usize] == s[r as usize] {
                count += 1;
                l -= 1;
                r += 1;
            }
        }

        count
    }
}

fn main() {
    let case_1 = "abc".to_string();
    let case_2 = "aaa".to_string();

    println!("case_1:{}", SolutionAns::count_substring(case_1));
    println!("case_2:{}", SolutionAns::count_substring(case_2));
}
