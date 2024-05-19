struct Solution;
impl Solution {
    pub fn convert_to_title(column_number: i32) -> String {
        let mut column_number = column_number;
        let num_to_alphabet = |n: i32| (b'A' + (n as u8 - 1)) as char;

        let mut result = "".to_string();
        /*
        if column_number <= 26 {
            let c_res = num_to_alphabet(column_number);
            result.push(c_res);
        }
        */
        while column_number > 0 {
            let (q, r) = (column_number / 26, column_number % 26);
            println!("q = {q}, r = {r}");
            column_number /= 26;
        }

        result
    }
}

struct SolutionAnsPython;
impl SolutionAnsPython {
    pub fn convert_to_title(column_number: i32) -> String {
        let mut column_number = column_number;
        let mut result = vec![];
        while column_number > 0 {
            let tmp = ((column_number - 1) % 26) as u8;
            result.push((b'A' + tmp) as char);
            column_number = (column_number - 1) / 26;
        }
        result.reverse();

        result.iter().collect()
    }
}

fn main() {
    let case_1 = 1;
    // => "A"
    let case_2 = 28;
    // => "AB"
    let case_3 = 701;
    // => "ZY"

    //println!("case_1: {}", Solution::convert_to_title(case_1));
    //println!("case_3: {}", Solution::convert_to_title(case_3));

    println!("case_1: {}", SolutionAnsPython::convert_to_title(case_1));
    println!("case_2: {}", SolutionAnsPython::convert_to_title(case_2));
    println!("case_3: {}", SolutionAnsPython::convert_to_title(case_3));
}
