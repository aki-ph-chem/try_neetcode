struct Solution {}
impl Solution {
    pub fn count_substrings(s: String) -> i32 {
        let n = s.len() as i32;
        let vec_char:Vec<char> = s.chars().collect();

        if n == 1 {
            return 1;
        }

        for i in 0..n {
            // n is odd
            let (mut l, mut r) = (i, i);
            while l >=0 && r < n && vec_char[l as usize] == vec_char[r as usize] {

                l -= 1;
                r += 1;
            }

            // n is even
            let (mut l, mut r) = (i, i + 1);
            while l >=0 && r < n && vec_char[l as usize] == vec_char[r as usize] {

                l -= 1;
                r += 1;
            }
        }

        1
    }
}

fn main() {
    let case_1 = "abc".to_string();
    let case_2 = "aaa".to_string();
}
