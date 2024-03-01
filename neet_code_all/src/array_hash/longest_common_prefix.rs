struct Solution {}
impl Solution {
    // AC
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        let strs: Vec<Vec<char>> = strs.iter().map(|x| x.chars().collect()).collect();
        let mut w_len_min = usize::MAX;
        for word in &strs {
            w_len_min = w_len_min.min(word.len());
        }
        let mut result: Vec<char> = vec![];

        for w in 0..w_len_min {
            let c_tmp = strs[0][w];
            let mut i = 1;
            while i < strs.len() {
                if c_tmp != strs[i][w] {
                    break;
                }
                i += 1;
            }

            if i >= strs.len() {
                result.push(c_tmp);
            } else {
                break;
            }
        }

        result.iter().collect()
    }
}

// 模範解答
// 正直ごちゃごちゃしすぎ
struct SolutionAns {}
impl SolutionAns {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        if strs.len() == 0 {
            return "".to_string();
        }

        if strs.len() == 1 {
            return strs[0].clone();
        }

        if strs[0].len() == 0 {
            return "".to_string();
        }

        let mut common = String::new();
        let mut same = true;
        let mut i = 0;
        while same {
            let common_char = match strs[0].chars().nth(i) {
                Some(c) => {
                    common.push(c);
                    c
                }
                None => return common,
            };

            for s in &strs {
                if let Some(c) = s.chars().nth(i) {
                    if c != common_char {
                        same = false;
                        break;
                    }
                } else {
                    same = false;
                    break;
                }
            }

            i += 1;
        }

        if common.len() > 0 {
            common[..common.len() - 1].to_string()
        } else {
            common
        }
    }
}

// AC
// C++の模範解答より
struct SolutionAnsCpp {}
impl SolutionAnsCpp {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        let strs: Vec<Vec<char>> = strs.iter().map(|x| x.chars().collect()).collect();
        let mut result = strs[0].clone();
        let mut char_index = 0;

        let mut max_char_index = usize::MAX;
        for word in &strs {
            max_char_index = max_char_index.min(word.len());
        }

        while char_index < max_char_index {
            let prev_char = strs[0][char_index];
            for i in 1..strs.len() {
                if prev_char == strs[i][char_index] {
                    continue;
                }

                return result[0..char_index].iter().collect();
            }
            char_index += 1;
            result.push(prev_char);
        }

        result[0..char_index].iter().collect()
    }
}

fn main() {
    let case_1 = vec![
        "flower".to_string(),
        "flow".to_string(),
        "flight".to_string(),
    ];
    let case_2 = vec!["dog".to_string(), "racecar".to_string(), "car".to_string()];

    println!(
        "case_1: {}",
        Solution::longest_common_prefix(case_1.clone())
    );
    println!(
        "case_2: {}",
        Solution::longest_common_prefix(case_2.clone())
    );

    println!(
        "case_1: {}",
        SolutionAns::longest_common_prefix(case_1.clone())
    );
    println!(
        "case_2: {}",
        SolutionAns::longest_common_prefix(case_2.clone())
    );

    println!(
        "case_1: {}",
        SolutionAnsCpp::longest_common_prefix(case_1.clone())
    );
    println!(
        "case_2: {}",
        SolutionAnsCpp::longest_common_prefix(case_2.clone())
    );
}
