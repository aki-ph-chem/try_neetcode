struct Solution {}
impl Solution {
    // AC
    pub fn str_str(haystack: String, needle: String) -> i32 {
        if haystack.len() < needle.len() {
            return -1;
        }

        let haystack: Vec<char> = haystack.chars().collect();
        let needle: Vec<char> = needle.chars().collect();

        for i in 0..(haystack.len() - needle.len() + 1) {
            let mut len_needle = 0;
            for j in 0..needle.len() {
                if haystack[i + j] == needle[j] {
                    len_needle += 1;
                } else {
                    break;
                }
            }
            if len_needle == needle.len() {
                return i as i32;
            }
        }

        -1
    }
}

fn main() {
    let case_1 = ("sadbutsad".to_string(), "sad".to_string());
    // => 0
    let case_2 = ("leetcode".to_string(), "leeto".to_string());
    // => -1

    println!(
        "case_1: {}",
        Solution::str_str(case_1.0.clone(), case_1.1.clone())
    );
    println!(
        "case_2: {}",
        Solution::str_str(case_2.0.clone(), case_2.1.clone())
    );
}
