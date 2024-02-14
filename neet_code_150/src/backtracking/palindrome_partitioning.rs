// 解けなかった
struct Solution {}
impl Solution {
    pub fn partition(s: String) -> Vec<Vec<String>> {
        let vec_char: Vec<char> = s.chars().collect();

        let mut current = vec![];
        let mut result = vec![];

        Self::dfs(
            &vec_char,
            0,
            vec_char.len() as i32 - 1,
            &mut current,
            &mut result,
        );

        result
    }

    fn dfs(
        s: &Vec<char>,
        start: i32,
        end: i32,
        current: &mut Vec<String>,
        result: &mut Vec<Vec<String>>,
    ) {
        let (mut l, mut r) = (start, end);
    }
}

// 模範解答
struct SolutionAns {}
impl SolutionAns {
    pub fn partition(s: String) -> Vec<Vec<String>> {
        let mut res = vec![];
        let mut part = vec![];

        Self::dfs(&s, &mut res, &mut part, 0);

        res
    }

    fn dfs(s: &String, res: &mut Vec<Vec<String>>, part: &mut Vec<String>, i: usize) {
        if i >= s.len() {
            res.push(part.clone());
            return;
        }

        for j in i..s.len() {
            if Self::is_palindrome(s, i, j) {
                let substr = s[i..j + 1].to_string();
                part.push(substr);
                Self::dfs(s, res, part, j + 1);
                part.pop();
            }
        }
    }

    fn is_palindrome(s: &String, mut left: usize, mut right: usize) -> bool {
        let s = s.as_bytes();
        while left < right {
            if s[left] != s[right] {
                return false;
            }

            left += 1;
            right -= 1;
        }

        true
    }
}

fn main() {
    let case_1 = "aab".to_string();
    // => [["a", "a", "b"], ["aa", "b"]]
    let case_2 = "a".to_string();
    // => [["a"]]

    println!("case_1 {:?}", SolutionAns::partition(case_1.clone()));
    println!("case_2 {:?}", SolutionAns::partition(case_2.clone()));
}
