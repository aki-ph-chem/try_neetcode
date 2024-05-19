// 解けなかった
struct Solution;
impl Solution {
    pub fn gcd_of_strings(str1: String, str2: String) -> String {
        let str1 = str1.chars().collect::<Vec<char>>();
        let str2 = str2.chars().collect::<Vec<char>>();

        for i_1 in 0..str1.len() {
            for i_2 in 0..str2.len() {}
        }

        "".to_string()
    }
}

// 模範解答
struct SolutionAns;
impl SolutionAns {
    pub fn gcd_of_strings(str1: String, str2: String) -> String {
        let (len1, len2) = (str1.len(), str2.len());
        for i in (1..=len1.min(len2) as usize).rev() {
            if Self::is_divisor(&str1, &str2, i) {
                return str1[..i].to_string();
            }
        }

        "".to_string()
    }

    fn is_divisor(str1: &str, str2: &str, i: usize) -> bool {
        let (len1, len2) = (str1.len(), str2.len());
        if len1 % i != 0 || len2 % i != 0 {
            return false;
        }

        let (f1, f2) = (len1 / i, len2 / i);
        str1[..i].repeat(f1) == *str1 && str1[..i].repeat(f2) == *str2
    }
}

// AC
// C++の模範解答より
struct SolutionAnsCpp;
impl SolutionAnsCpp {
    pub fn gcd_of_strings(str1: String, str2: String) -> String {
        let (str1, str2) = (
            str1.chars().collect::<Vec<char>>(),
            str2.chars().collect::<Vec<char>>(),
        );
        let (str_short, str_long) = if str1.len() < str2.len() {
            (str1, str2)
        } else {
            (str2, str1)
        };

        let mut result = "".to_string();
        let (len_short, len_long) = (str_short.len(), str_long.len());

        for i in (1..=len_short).rev() {
            if len_long % i != 0 || len_short % i != 0 {
                continue;
            }

            for j in 0..len_long {
                let (ptr_fistr, ptr_second) = (j % i, j % len_short);
                if str_short[ptr_fistr] != str_long[j] || str_short[ptr_second] != str_long[j] {
                    result = "".to_string();
                    break;
                }

                if ptr_fistr == j {
                    result.push(str_long[j]);
                }
            }

            if result != "" {
                return result;
            }
        }

        "".to_string()
    }
}

fn main() {
    let case_1 = ("ABCABC".to_string(), "ABC".to_string());
    // => "ABC"
    let case_2 = ("ABABAB".to_string(), "ABAB".to_string());
    // => "AB"
    let case_3 = ("LEET".to_string(), "CODE".to_string());
    // => ""

    println!(
        "case_1: {}",
        SolutionAns::gcd_of_strings(case_1.0.clone(), case_1.1.clone())
    );
    println!(
        "case_2: {}",
        SolutionAns::gcd_of_strings(case_2.0.clone(), case_2.1.clone())
    );
    println!(
        "case_3: {}",
        SolutionAns::gcd_of_strings(case_3.0.clone(), case_3.1.clone())
    );

    println!(
        "case_1: {}",
        SolutionAnsCpp::gcd_of_strings(case_1.0.clone(), case_1.1.clone())
    );
    println!(
        "case_2: {}",
        SolutionAnsCpp::gcd_of_strings(case_2.0.clone(), case_2.1.clone())
    );
    println!(
        "case_3: {}",
        SolutionAnsCpp::gcd_of_strings(case_3.0.clone(), case_3.1.clone())
    );
}
