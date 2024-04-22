struct Solution {}
impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        if s.len() == 0 {
            return true;
        }

        let mut str: Vec<char> = s.to_lowercase().chars().collect();
        str.retain(|&c| c.is_ascii_alphanumeric());

        let length = str.len() - 1;
        for i in 0..(length + 1) {
            if str[i] != str[length - i] {
                return false;
            }
        }

        true
    }
}

// 模範解答
struct SolutionAns {}
impl SolutionAns {
    pub fn is_palindrome(s: String) -> bool {
        let s: Vec<char> = s
            .chars()
            .filter(|c| c.is_alphanumeric())
            .map(|c| c.to_lowercase().next().unwrap())
            .collect();

        let len = s.len();

        for i in 0..(len / 2) {
            if s[i] != s[len - i - 1] {
                return false;
            }
        }

        true
    }
}

// C++の模範解答より
struct SolutionAnsCpp {}
impl SolutionAnsCpp {
    pub fn is_palindrome(s: String) -> bool {
        let mut i = 0 as i32;
        let mut j = s.len() as i32 - 1;
        let vec_char: Vec<char> = s.chars().collect();

        while i < j {
            while !vec_char[i as usize].is_alphanumeric() && i < j {
                i += 1;
            }
            while !vec_char[j as usize].is_alphanumeric() && i < j {
                j -= 1;
            }
            if vec_char[i as usize].to_lowercase().to_string()
                != vec_char[j as usize].to_lowercase().to_string()
            {
                return false;
            }
            i += 1;
            j -= 1;
        }

        true
    }

    // AC
    // 部分的別解
    // 文字がアルファベットもしくは数字かの判定
    // 大文字->小文字変換
    // まで自分で実装するならこう書く
    pub fn is_palindrome_2(s: String) -> bool {
        let is_alphanumeric = |c: char| {
            let c = c as u8;
            let (num_start, num_end) = (b'0', b'9');
            let (alphabet_start, alphabet_end) = (b'a', b'z');
            let (alphabet_capital_start, alphabet_capital_end) = (b'A', b'Z');

            if (num_start <= c && c <= num_end)
                || (alphabet_start <= c && c <= alphabet_end)
                || (alphabet_capital_start <= c && c <= alphabet_capital_end)
            {
                return true;
            }

            false
        };

        let to_lowercase = |c: char| {
            let c = c as u8;
            let (alphabet_start, alphabet_end) = (b'A', b'Z');
            if alphabet_start <= c && c <= alphabet_end {
                let res_u8 = (c as i32 - b'A' as i32 + b'a' as i32) as u8;
                return res_u8 as char;
            }

            c as char
        };

        let s = s.chars().collect::<Vec<char>>();
        let (mut left, mut right) = (0, s.len() as i32 - 1);

        while left < right {
            while !is_alphanumeric(s[left as usize]) && left < right {
                left += 1;
            }
            while !is_alphanumeric(s[right as usize]) && left < right {
                right -= 1;
            }

            if to_lowercase(s[left as usize]) != to_lowercase(s[right as usize]) {
                return false;
            }

            left += 1;
            right -= 1;
        }

        true
    }

    // AC
    // space: O(N)なのでメモリ効率が悪い
    pub fn is_palindrome_3(s: String) -> bool {
        let s = s.to_lowercase().chars().collect::<Vec<char>>();
        let mut s_new = vec![];
        let mut s_new_rev = vec![];

        for (c, c_rev) in s.iter().zip(s.iter().rev()) {
            if !" !@#$%^&*()_-=+\\|~`{}[]:;'\"<>,./?".contains(*c) {
                s_new.push(*c);
            }
            if !" !@#$%^&*()_-=+\\|~`{}[]:;'\"<>,./?".contains(*c_rev) {
                s_new_rev.push(*c_rev);
            }
        }

        s_new == s_new_rev
    }
}

fn main() {
    let case_1 = "A man, a plan, a canal: Panama".to_string();
    let case_2 = "race a car".to_string();
    let case_3 = "".to_string();

    println!("ase_1: {}", Solution::is_palindrome(case_1.clone()));
    println!("ase_2: {}", Solution::is_palindrome(case_2.clone()));
    println!("ase_3: {}", Solution::is_palindrome(case_3.clone()));

    println!("ase_1: {}", SolutionAns::is_palindrome(case_1.clone()));
    println!("ase_2: {}", SolutionAns::is_palindrome(case_2.clone()));
    println!("ase_3: {}", SolutionAns::is_palindrome(case_3.clone()));

    println!("ase_1: {}", SolutionAnsCpp::is_palindrome(case_1.clone()));
    println!("ase_2: {}", SolutionAnsCpp::is_palindrome(case_2.clone()));
    println!("ase_3: {}", SolutionAnsCpp::is_palindrome(case_3.clone()));

    println!("ase_1: {}", SolutionAnsCpp::is_palindrome_2(case_1.clone()));
    println!("ase_2: {}", SolutionAnsCpp::is_palindrome_2(case_2.clone()));
    println!("ase_3: {}", SolutionAnsCpp::is_palindrome_2(case_3.clone()));

    println!("ase_1: {}", SolutionAnsCpp::is_palindrome_3(case_1.clone()));
    println!("ase_2: {}", SolutionAnsCpp::is_palindrome_3(case_2.clone()));
    println!("ase_3: {}", SolutionAnsCpp::is_palindrome_3(case_3.clone()));
}
