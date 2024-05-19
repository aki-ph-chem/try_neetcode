struct Solution;
impl Solution {
    // i32 -> Stringの変換を使う
    // 模範解答と同じ
    pub fn is_palindrome(x: i32) -> bool {
        if x < 0 {
            return false;
        }

        let x = x.to_string().chars().collect::<Vec<char>>();
        let (mut left, mut right) = (0, x.len() - 1);
        while left < right {
            if x[left] != x[right] {
                return false;
            }

            left += 1;
            right -= 1;
        }

        true
    }

    // AC
    // i32 -> Stringの変換を使わないがバッファが必要
    pub fn is_palindrome_2(x: i32) -> bool {
        if x < 0 {
            return false;
        } else if x == 0 {
            return true;
        }

        let mut digigt = vec![];
        let mut x = x;
        while x > 0 {
            digigt.push(x % 10);
            x /= 10;
        }
        let (mut left, mut right) = (0, digigt.len() - 1);
        while left < right {
            if digigt[left] != digigt[right] {
                return false;
            }

            left += 1;
            right -= 1;
        }

        true
    }
}

// C++の模範解答より
// Stringへの変換もバッファも要らない
struct SolutionAnsCpp;
impl SolutionAnsCpp {
    // AC
    pub fn is_palindrome(x: i32) -> bool {
        let mut x = x as i64;
        if x < 0 {
            return false;
        }

        let mut div = 1_i64;
        while x >= 10 * div {
            div *= 10;
        }

        while x != 0 {
            let (right, left) = (x % 10, x / div);

            if left != right {
                return false;
            }

            x = (x % div) / 10;
            div /= 100;
        }

        true
    }
}

fn main() {
    let case_1 = 121;
    // => true
    let case_2 = -121;
    // => false
    let case_3 = 10;
    // => false
    let case_4 = 1410110141;
    // => true

    println!("case_1: {}", Solution::is_palindrome(case_1));
    println!("case_2: {}", Solution::is_palindrome(case_2));
    println!("case_3: {}", Solution::is_palindrome(case_3));

    println!("case_1: {}", Solution::is_palindrome_2(case_1));
    println!("case_2: {}", Solution::is_palindrome_2(case_2));
    println!("case_3: {}", Solution::is_palindrome_2(case_3));

    println!("case_1: {}", SolutionAnsCpp::is_palindrome(case_1));
    println!("case_2: {}", SolutionAnsCpp::is_palindrome(case_2));
    println!("case_3: {}", SolutionAnsCpp::is_palindrome(case_3));
    println!("case_4: {}", SolutionAnsCpp::is_palindrome(case_4));
}
