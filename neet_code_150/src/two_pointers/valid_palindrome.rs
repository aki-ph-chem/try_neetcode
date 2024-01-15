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
        let vec_char:Vec<char> = s.chars().collect();

        while i < j {
            while !vec_char[i as usize].is_alphanumeric() && i < j {
                i += 1;
            }
            while !vec_char[j as usize].is_alphanumeric() && i < j {
                j -= 1;
            }
            if vec_char[i as usize].to_lowercase().to_string() != vec_char[j as usize].to_lowercase().to_string() {
                return false;
            }
            i += 1;
            j -= 1;
        }

        true
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
}
