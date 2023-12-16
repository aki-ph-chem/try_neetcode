// Hint 1: 何を再利用してより長い回文を探すか
// Hint 2: "aba"が回分なら"xabax"も回分
// Hint 3: 部分文字列の列挙でO(N^2), 各区間の回文判定にO(N)
// 最後の回文判定をキャッシュ化できる

struct Solution {}
impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        let n = s.len();
        let str: Vec<char> = s.chars().collect();
        let mut len_list: Vec<((usize, usize), i32)> = vec![];

        // i < j
        for i in 0..n {
            for j in (i + 1)..n {
                // i <= k <=j
                let mut len_sub_str = 0;

                // 区間[i,j]が回文か判定
                println!("{:?}",((i, j), len_sub_str));
                len_list.push(((i, j), len_sub_str));
            }
        }

        "hoge".to_string()
    }

    // 最大の回文の長さを返す
    pub fn palindrome_len(s: String) -> i32 {
        let n = s.len();
        let str: Vec<char> = s.chars().collect();
        let mut len_list = vec![];

        // i < j
        for i in 0..n {
            for j in i + 1..n {
                // i <= k <=j
                let mut len_sub_str = 0;
                // 区間[i,j]が回文か判定
                for k in 0..(j - i + 1){
                    if str[i + k] == str[j - k] {
                        len_sub_str += 1;
                    } else {
                        len_sub_str = -1;
                        break;
                    }
                }
                len_list.push(len_sub_str);
            }
        }

        *len_list.iter().max().unwrap()
    }

    // 最大の回文を返す
    pub fn palindrome_max_len(s: String) -> ((usize, usize, i32), String) {
        let n = s.len();
        let str: Vec<char> = s.chars().collect();
        let mut len_list: Vec<((usize, usize), i32)> = vec![];

        // i < j
        for i in 0..n {
            for j in (i + 1)..n {
                // i <= k <=j
                let mut len_sub_str = 0;
                // 区間[i,j]が回文か判定
                for k in 0..(j - i + 1) {
                    if str[i + k] == str[j - k] {
                        len_sub_str += 1;
                    } else {
                        len_sub_str = -1;
                        break;
                    }
                }
                println!("{:?}",((i, j), len_sub_str));
                len_list.push(((i, j), len_sub_str));
            }
        }

        let mut max_len = -(1 << 30);
        let (mut i_start, mut i_end) = (0, 0);
        for x in len_list {
            if max_len < x.1 {
                max_len = x.1;
                (i_start, i_end) = x.0;
            }
        }

        ((i_start, i_end, max_len), s[i_start..(i_end + 1)].to_string())
    }
}

// 模範解答
struct SolutionAns {}
impl SolutionAns {
    pub fn longest_palindrome(s: String) -> String {
        let s = s.chars().collect::<Vec<char>>();
        let (mut left, mut right, length): (i32, i32, i32) = (0, 0, s.len() as i32);

        if length == 1 {
            return s[0].to_string();
        }

        for i in 0..length {
            // odd length
            let (mut l, mut r) = (i, i);

            while l >= 0 && r < length && s[l as usize] == s[r as usize] {
                // r - l: 新しい部分文字列の長さが right - left: 前の部分文字列の長さ
                // より大きければ更新
                if r - l > right - left {
                    left = l;
                    right = r;
                }
                l -= 1;
                r += 1;
            }

            // even length
            let (mut l, mut r) = (i, i + 1);

            while l >= 0 && r < length && s[l as usize] == s[r as usize] {
                // r - l: 新しい部分文字列の長さが right - left: 前の部分文字列の長さ
                // より大きければ更新
                if r - l > right - left {
                    left = l;
                    right = r;
                }
                l -= 1;
                r += 1;
            }
        }

        s[left as usize..=right as usize].iter().collect::<String>()
    }
}

fn main() {
    let case_1 = "babad".to_string();
    // "bab" or "aba"
    let case_2 = "cbbd".to_string();
    // "bb"

    println!("{}", Solution::palindrome_len(case_1.clone()));
    println!("{}", Solution::palindrome_len(case_2.clone()));

    println!("{:?}", Solution::palindrome_max_len(case_1.clone()));
    println!("{:?}", Solution::palindrome_max_len(case_2.clone()));

    /*
    let hoge = "abcd";
    println!("hoge:[0,2]: {}", hoge[0..2].to_string()); // ab
    println!("hoge:[1,3]: {}", hoge[1..3].to_string()); // bc
    */
}
