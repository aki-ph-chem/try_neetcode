use std::collections::HashMap;

// 変換のルール
/*
2 => a,b,c
3 => d,e,f
4 => g,h,i
5 => j,k,l
6 => m,n,o
7 => p,q,r,s
8 => t,u,v
9 => w,x,y,z
*/

// AC
struct Solution {}
impl Solution {
    pub fn letter_combinations(digits: String) -> Vec<String> {
        if digits.len() < 1 {
            return vec![];
        }

        let vec_char: Vec<char> = digits.chars().collect();

        let mut map: HashMap<u8, Vec<char>> = HashMap::new();
        map.insert(2, vec!['a', 'b', 'c']);
        map.insert(3, vec!['d', 'e', 'f']);
        map.insert(4, vec!['g', 'h', 'i']);
        map.insert(5, vec!['j', 'k', 'l']);
        map.insert(6, vec!['m', 'n', 'o']);
        map.insert(7, vec!['p', 'q', 'r', 's']);
        map.insert(8, vec!['t', 'u', 'v']);
        map.insert(9, vec!['w', 'x', 'y', 'z']);

        let mut result = vec![];
        let mut current = vec![];
        Self::dfs(&map, &vec_char, 0, &mut result, &mut current);

        result
    }

    fn dfs(
        map: &HashMap<u8, Vec<char>>,
        digits: &Vec<char>,
        idx: u8,
        result: &mut Vec<String>,
        current: &mut Vec<char>,
    ) {
        if digits.len() <= idx as usize {
            result.push(current.iter().collect());
            return;
        }

        let n = digits[idx as usize] as u8 - b'0';
        for c in &map[&n] {
            current.push(*c);
            Self::dfs(map, digits, idx + 1, result, current);
            current.pop();
        }
    }
}

// 模範解答
struct SolutionAns {}
impl SolutionAns {
    pub fn letter_combinations(digits: String) -> Vec<String> {
        let mut result = vec![];
        let d_c_map = HashMap::from([
            ('2', "abc"),
            ('3', "def"),
            ('4', "ghi"),
            ('5', "jkl"),
            ('6', "mno"),
            ('7', "qprs"),
            ('8', "tuv"),
            ('9', "wxyz"),
        ]);

        if !digits.is_empty() {
            Self::backtrack(0, "".to_string(), &mut result, &digits, &d_c_map);
        }

        result
    }

    fn backtrack(
        i: usize,
        current: String,
        result: &mut Vec<String>,
        digits: &String,
        d_c_map: &HashMap<char, &str>,
    ) {
        if current.len() == digits.len() {
            result.push(current);
            return;
        }

        let letters = d_c_map
            .get(&digits.chars().nth(i).unwrap())
            .unwrap()
            .to_string();

        for ch in letters.chars() {
            let mut append_str = current.clone();
            append_str.push(ch);
            Self::backtrack(i + 1, append_str, result, digits, d_c_map);
        }
    }
}

fn main() {
    let case_1 = "23".to_string();
    // => ["ad","ae","af","bd","be","bf","cd","ce","cf"]
    let case_2 = "".to_string();
    // => []
    let case_3 = "2".to_string();
    // => ["a", "b", "c"]

    println!(
        "case_1: {:?}",
        Solution::letter_combinations(case_1.clone())
    );
    println!(
        "case_2: {:?}",
        Solution::letter_combinations(case_2.clone())
    );
    println!(
        "case_3: {:?}",
        Solution::letter_combinations(case_3.clone())
    );

    println!(
        "case_1: {:?}",
        SolutionAns::letter_combinations(case_1.clone())
    );
    println!(
        "case_2: {:?}",
        SolutionAns::letter_combinations(case_2.clone())
    );
    println!(
        "case_3: {:?}",
        SolutionAns::letter_combinations(case_3.clone())
    );
}
