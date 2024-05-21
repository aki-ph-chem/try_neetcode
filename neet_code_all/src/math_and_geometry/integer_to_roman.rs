//use std::collections::HashMap;
// 解けなかった
struct Solution;
impl Solution {
    pub fn int_to_roman(num: i32) -> String {
        let map = vec![
            (1000, 'M'),
            (500, 'D'),
            (100, 'C'),
            (50, 'L'),
            (10, 'X'),
            (5, 'V'),
            (1, 'I'),
        ];

        let mut result = vec![];
        let mut num = num;
        for (n, c) in map {
            let q = num / n;
            if q % 4 == 0 || q % 9 == 0 {
            } else {
                for _ in 0..q {
                    result.push(c);
                }
            }
            num %= n;
        }

        result.iter().collect()
    }
}

// 模範解答
struct SolutionAns;
impl SolutionAns {
    pub fn int_to_roman(num: i32) -> String {
        let int_roman = [
            (1000, "M"),
            (900, "CM"),
            (500, "D"),
            (400, "CD"),
            (100, "C"),
            (90, "XC"),
            (50, "L"),
            (40, "XL"),
            (10, "X"),
            (9, "IX"),
            (5, "V"),
            (4, "IV"),
            (1, "I"),
        ];
        let mut result = "".to_string();
        let mut num = num;

        for (int, roman) in int_roman {
            if num / int > 0 {
                let count = num / int;
                result.push_str(&roman.repeat(count as usize));
                num %= int;
            }
        }

        result
    }
}

// C++の模範解答より
struct SolutionAnsCpp;
impl SolutionAnsCpp {
    // AC
    pub fn int_to_roman(num: i32) -> String {
        let int_roman = [
            (1000, "M"),
            (900, "CM"),
            (500, "D"),
            (400, "CD"),
            (100, "C"),
            (90, "XC"),
            (50, "L"),
            (40, "XL"),
            (10, "X"),
            (9, "IX"),
            (5, "V"),
            (4, "IV"),
        ];
        let mut result = "".to_string();
        let mut num = num;
        for i in 0..int_roman.len() {
            if i + 1 < int_roman.len() {
                while num >= int_roman[i].0 {
                    result.push_str(int_roman[i].1);
                    num -= int_roman[i].0;
                }
                if num >= int_roman[i + 1].0 {
                    result.push_str(int_roman[i + 1].1);
                    num -= int_roman[i + 1].0;
                }
            }
        }
        while num >= 1 {
            result.push_str("I");
            num -= 1;
        }

        result
    }
}

fn main() {
    let case_1 = 3749;
    // => "MMMDCCXLIX"
    let case_2 = 58;
    // => "LVIII"
    let case_3 = 1994;
    // => "MCMXCIV"
    let case_4 = 20;
    // => "XX"

    /*
    println!("case_2: {}", Solution::int_to_roman(case_2));
    println!("case_1: {}", Solution::int_to_roman(case_1));
    */

    println!("case_1: {}", SolutionAns::int_to_roman(case_1));
    println!("case_2: {}", SolutionAns::int_to_roman(case_2));
    println!("case_3: {}", SolutionAns::int_to_roman(case_3));
    println!("case_4: {}", SolutionAns::int_to_roman(case_4));

    println!("case_1: {}", SolutionAnsCpp::int_to_roman(case_1));
    println!("case_2: {}", SolutionAnsCpp::int_to_roman(case_2));
    println!("case_3: {}", SolutionAnsCpp::int_to_roman(case_3));
    println!("case_4: {}", SolutionAnsCpp::int_to_roman(case_4));
}
