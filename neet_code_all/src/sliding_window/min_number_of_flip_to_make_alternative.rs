// 解けなかった
struct Solution {}
impl Solution {
    pub fn min_flips(s: String) -> i32 {
        1
    }
}

struct SolutionAnsPython {}
impl SolutionAnsPython {
    // AC
    pub fn min_flips(s: String) -> i32 {
        let mut s = s;
        let s_len_prev = s.len();
        s += &s.clone();
        let s = s.chars().collect::<Vec<char>>();
        let (mut alt_1, mut alt_2) = (vec![], vec![]);

        for i in 0..s.len() {
            if i % 2 == 0 {
                alt_1.push('0');
                alt_2.push('1');
            } else {
                alt_1.push('1');
                alt_2.push('0');
            }
        }

        let mut result = i32::MAX;
        let (mut diff_1, mut diff_2) = (0, 0);
        let mut left = 0;
        for right in 0..s.len() {
            if s[right] != alt_1[right] {
                diff_1 += 1;
            }
            if s[right] != alt_2[right] {
                diff_2 += 1;
            }
            if right - left + 1 > s_len_prev {
                if s[left] != alt_1[left] {
                    diff_1 -= 1;
                }
                if s[left] != alt_2[left] {
                    diff_2 -= 1;
                }

                left += 1;
            }

            if right - left + 1 == s_len_prev {
                result = result.min(diff_1.min(diff_2));
            }
        }

        result
    }
}

fn main() {
    let case_1 = "111000".to_string();
    // => 2
    let case_2 = "010".to_string();
    // => 0
    let case_3 = "1110".to_string();
    // => 1
    let case_4 = "01001001101".to_string();
    // => 2

    println!("case_1: {}", SolutionAnsPython::min_flips(case_1.clone()));
    println!("case_2: {}", SolutionAnsPython::min_flips(case_2.clone()));
    println!("case_3: {}", SolutionAnsPython::min_flips(case_3.clone()));
    println!("case_4: {}", SolutionAnsPython::min_flips(case_4.clone()));
}
