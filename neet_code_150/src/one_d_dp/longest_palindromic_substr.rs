// Hint 1: 何を再利用してより長い回文を探すか
// Hint 2: "aba"が回分なら"xabax"も回分
// Hint 3: 部分文字列の列挙でO(N^2), 各区間の回文判定にO(N)
// 最後の回文判定をキャッシュ化できる

struct Solution{}
impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        "hoge".to_string()
    }

    // 最大の回文の長さを返す
    pub fn palindrome_len(s: String) -> i32 {
        let n = s.len();
        let str: Vec<char> = s.chars().collect();
        let mut len_list = vec![];

        // i < j
        for i in 0..n {
            for j in i + 1 ..n {
                // i <= k <=j
                let mut len_sub_str = 0;
                // 区間[i,j]が回文か判定
                for k in i..(j + 1) {
                    if str[k] == str[j - k] {
                        len_sub_str += 1;
                    } else {
                        break;
                    }
                }
                len_list.push(len_sub_str);
            }
        }

        *len_list.iter().max().unwrap()
    }
}

fn main() {
    let case_1 = "babad".to_string(); 
    // "bad" or "aba"
    let case_2 = "cbbd".to_string();
    // "bb"

    println!("{}",Solution::palindrome_len(case_1.clone()));
    println!("{}",Solution::palindrome_len(case_2.clone()));
}
