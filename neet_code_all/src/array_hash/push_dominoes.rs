// 解けなかった
struct Solution {}
impl Solution {
    pub fn push_dominoes(dominoes: String) -> String {
        let mut dominoes = dominoes.chars().collect::<Vec<char>>();
        let n = dominoes.len() as i32;

        for i in 1..n - 1 {
            let (left, mid, right) = (
                dominoes[i as usize - 1],
                dominoes[i as usize],
                dominoes[i as usize + 1],
            );
            if left == 'R' && mid == '.' && right == 'L' {
                continue;
            }
        }

        dominoes.iter().collect()
    }
}

// C++の模範解答より
struct SolutionAnsCpp {}
impl SolutionAnsCpp {
    // AC
    pub fn push_dominoes(dominoes: String) -> String {
        let mut result = "".to_string();
        let n = dominoes.len();
        let mut count = 1;
        let mut prev = '0';

        // 部分列が "R." となっている部分の'.'の位置idxと'.'の数count
        // right[idx] = count
        let mut right = vec![0; n];
        // 部分列が ".L" となっている部分の'.'の位置idxと'.'の数count
        // right[idx] = count
        let mut left = vec![0; n];

        // 左から
        for (idx, d) in dominoes.chars().enumerate() {
            if d == 'R' {
                count = 1;
                prev = 'R';
            } else if d != '.' {
                prev = d;
            }

            if prev == 'R' && d == '.' {
                right[idx] = count;
                count += 1;
            }
        }

        prev = '.';
        // 右から
        for (idx, d) in dominoes.chars().rev().enumerate() {
            let idx = n - 1 - idx;
            if d == 'L' {
                count = 1;
                prev = 'L';
            } else if d != '.' {
                prev = d;
            }

            if prev == 'L' && d == '.' {
                left[idx] = count;
                count += 1;
            }
        }

        for (idx, d) in dominoes.chars().enumerate() {
            match (left[idx], right[idx]) {
                (0, 0) => result.push(d),
                (0, _) => result.push('R'),
                (_, 0) => result.push('L'),
                (a, b) => {
                    if a == b {
                        result.push('.');
                    } else if a < b {
                        result.push('L');
                    } else {
                        result.push('R');
                    }
                }
            }
        }

        result
    }
}

fn main() {
    let case_1 = "RR.L".to_string();
    // => "RR.L"
    let case_2 = ".L.R...LR..L..".to_string();
    // => "LL.RR.LLRRLL.."

    println!("case_1: {}", SolutionAnsCpp::push_dominoes(case_1.clone()));
    println!("case_2: {}", SolutionAnsCpp::push_dominoes(case_2.clone()));
}
