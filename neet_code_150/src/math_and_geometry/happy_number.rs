use std::collections::HashSet;

// AC
// しかし、速度、メモリともに悪い
struct Solution {}
impl Solution {
    pub fn is_happy(n: i32) -> bool {
        let mut m = n;
        let mut set = HashSet::new();

        while m != 1 {
            let mut digits = vec![];
            while m > 0 {
                digits.push(m % 10);
                m /= 10;
            }
            m = digits.iter().map(|x| x * x).sum();
            // mが同じ数が複数回現れると無限ループとなってしまう
            if set.contains(&m) {
                return false;
            } else {
                set.insert(m);
            }
            /*
            println!("digits: {:?}", digits);
            println!("m: {}", m);
            */
        }

        true
    }
}

// 模範解答
// match式の "1|4"が??
struct SolutionAns {}
impl SolutionAns {
    pub fn is_happy(mut n: i32) -> bool {
        loop {
            let mut s = 0;
            while n > 0 {
                s += (n % 10).pow(2);
                n /= 10;
            }
            match s {
                1 | 4 => break s == 1,
                _ => n = s,
            }
        }
    }
}

fn main() {
    println!("case_1: {}", Solution::is_happy(19));
    println!("case_2: {}", Solution::is_happy(2));
    println!("case_3: {}", Solution::is_happy(12));

    println!("case_1: {}", SolutionAns::is_happy(19));
    println!("case_2: {}", SolutionAns::is_happy(2));
    println!("case_3: {}", SolutionAns::is_happy(12));
}
