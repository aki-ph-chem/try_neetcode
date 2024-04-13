use std::collections::HashMap;
use std::collections::HashSet;

struct Solution {}
impl Solution {
    // AC
    pub fn has_all_codes(s: String, k: i32) -> bool {
        let k = k as usize;
        let n = s.len();

        if n < k {
            return false;
        }

        let mut bit_list = HashMap::new();
        for bit in 0..1 << k {
            bit_list.insert(bit, 0);
        }
        /*
        for (bit, n) in &bit_list {
            println!("bit, n: {:#b}, {n}", bit);
        }
        */

        for i in 0..=(n - k) {
            let sub_str = &s[i..(i + k)];
            if let Some(num_bit) = bit_list.get_mut(&Self::str_to_bit(sub_str)) {
                *num_bit += 1;
            }
        }

        for (_, n) in bit_list {
            if n == 0 {
                return false;
            }
        }

        true
    }

    pub fn str_to_bit(bit_string: &str) -> i32 {
        let mut bit = 0;
        for (i, c) in bit_string.chars().rev().enumerate() {
            if c == '1' {
                bit += 1 << i;
            }
        }

        bit
    }
}

// C++の模範解答より
struct SolutionAnsCpp {}
impl SolutionAnsCpp {
    // AC
    pub fn has_all_codes(s: String, k: i32) -> bool {
        let mut set = HashSet::new();
        let n = s.len();
        let k = k as usize;
        if n < k {
            return false;
        }
        let total = 1 << k;

        for i in 0..=(n - k) {
            set.insert(s[i..(i + k)].to_string());
            if set.len() == total {
                return true;
            }
        }

        false
    }
}

// Pythonの模範解答より(ワンライナー)
struct SolutionAnsPython {}
impl SolutionAnsPython {
    pub fn has_all_codes(s: String, k: i32) -> bool {
        (0..=(s.len() as i32 - k))
            .fold(HashSet::new(), |mut set, i| {
                set.insert(s[i as usize..(i as usize + k as usize)].to_string());
                set
            })
            .len()
            == 1 << k
    }
}

fn main() {
    let case_1 = ("00110110".to_string(), 2);
    // => true
    let case_2 = ("0110".to_string(), 1);
    // => true
    let case_3 = ("0110".to_string(), 2);
    // => false

    println!(
        "case_1: {}",
        Solution::has_all_codes(case_1.0.clone(), case_1.1)
    );
    println!(
        "case_2: {}",
        Solution::has_all_codes(case_2.0.clone(), case_2.1)
    );
    println!(
        "case_3: {}",
        Solution::has_all_codes(case_3.0.clone(), case_3.1)
    );

    println!(
        "case_1: {}",
        SolutionAnsCpp::has_all_codes(case_1.0.clone(), case_1.1)
    );
    println!(
        "case_2: {}",
        SolutionAnsCpp::has_all_codes(case_2.0.clone(), case_2.1)
    );
    println!(
        "case_3: {}",
        SolutionAnsCpp::has_all_codes(case_3.0.clone(), case_3.1)
    );

    println!(
        "case_1: {}",
        SolutionAnsPython::has_all_codes(case_1.0.clone(), case_1.1)
    );
    println!(
        "case_2: {}",
        SolutionAnsPython::has_all_codes(case_2.0.clone(), case_2.1)
    );
    println!(
        "case_3: {}",
        SolutionAnsPython::has_all_codes(case_3.0.clone(), case_3.1)
    );
}
