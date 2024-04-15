use std::collections::HashSet;

struct Solution {}
impl Solution {
    // AC
    pub fn partitoin_string(s: String) -> i32 {
        let s = s.chars().collect::<Vec<char>>();
        let mut i = 0;
        let mut result = vec![];
        let mut set = HashSet::new();
        while i < s.len() {
            let mut j = i;
            while j < s.len() && !set.contains(&s[j]) {
                set.insert(s[j]);
                j += 1;
            }
            //println!("i, j: {i},{j}");
            //println!("set: {:?}", set);
            result.push((i, j - 1));
            i = j;
            set.clear();
        }
        //println!("result: {:?}", result);

        result.len() as i32
    }

    // AC: こっちの方が省メモリ
    pub fn partitoin_string_2(s: String) -> i32 {
        let s = s.chars().collect::<Vec<char>>();
        let mut i = 0;
        let mut result = 0;
        let mut set = HashSet::new();
        while i < s.len() {
            let mut j = i;
            while j < s.len() && !set.contains(&s[j]) {
                set.insert(s[j]);
                j += 1;
            }
            //println!("i, j: {i},{j}");
            //println!("set: {:?}", set);
            result += 1;
            i = j;
            set.clear();
        }
        //println!("result: {:?}", result);

        result
    }
}

// Kotlinの模範解答より
struct SolutionAnsKotlin {}
impl SolutionAnsKotlin {
    // AC
    pub fn partitoin_string(s: String) -> i32 {
        let mut set = HashSet::new();
        let mut result = 0;

        for c in s.chars() {
            if set.contains(&c) {
                result += 1;
                set.clear();
            }
            set.insert(c);
        }

        if set.len() == 0 {
            return result;
        }

        result + 1
    }
}

fn main() {
    let case_1 = "abacaba".to_string();
    // => 4
    let case_2 = "ssssss".to_string();
    // => 6

    println!("case_1:{}", Solution::partitoin_string(case_1.clone()));
    println!("case_2:{}", Solution::partitoin_string(case_2.clone()));

    println!("case_1:{}", Solution::partitoin_string_2(case_1.clone()));
    println!("case_2:{}", Solution::partitoin_string_2(case_2.clone()));

    println!(
        "case_1:{}",
        SolutionAnsKotlin::partitoin_string(case_1.clone())
    );
    println!(
        "case_2:{}",
        SolutionAnsKotlin::partitoin_string(case_2.clone())
    );
}
