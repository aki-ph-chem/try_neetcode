// 解けなかった
struct Solution {}
impl Solution {
    pub fn remove_kdigits(num: String, k: i32) -> String {
        let mut num = num.chars().collect::<Vec<char>>();

        "".to_string()
    }
}

// 模範解答
struct SolutionAns;
impl SolutionAns {
    pub fn remove_kdigits(num: String, k: i32) -> String {
        let mut stack = vec![];
        let mut k = k as usize;

        for c in num.chars() {
            while let Some(&top) = stack.last() {
                if k > 0 && top > c {
                    k -= 1;
                    stack.pop();
                } else {
                    break;
                }
            }

            stack.push(c);
        }

        while k != 0 {
            stack.pop();
            k -= 1;
        }

        let result = stack.iter().skip_while(|&c| *c == '0').collect::<String>();

        if result.is_empty() {
            return "0".to_string();
        }
        result
    }
}

// Pythonの模範解答より
// AC
struct SolutionAnsPython;
impl SolutionAnsPython {
    pub fn remove_kdigits(num: String, k: i32) -> String {
        let mut stack = vec![];
        let mut k = k as usize;

        for c in num.chars() {
            while let Some(&top) = stack.last() {
                if k > 0 && top > c {
                    k -= 1;
                    stack.pop();
                } else {
                    break;
                }
            }

            if !stack.is_empty() || c != '0' {
                stack.push(c);
            }
        }

        //println!("k,stack: {}, {:?}", k, stack);

        while k != 0 {
            stack.pop();
            k -= 1;
        }

        if stack.is_empty() {
            return "0".to_string();
        }

        stack.iter().collect::<String>()
    }
}

fn main() {
    let case_1 = ("1432219".to_string(), 3);
    // => "1219"
    let case_2 = ("10200".to_string(), 1);
    // => "200"
    let case_3 = ("10".to_string(), 2);
    // => "0"

    println!(
        "case_1: {}",
        SolutionAns::remove_kdigits(case_1.0.clone(), case_1.1)
    );
    println!(
        "case_2: {}",
        SolutionAns::remove_kdigits(case_2.0.clone(), case_2.1)
    );
    println!(
        "case_3: {}",
        SolutionAns::remove_kdigits(case_3.0.clone(), case_3.1)
    );

    println!(
        "case_1: {}",
        SolutionAnsPython::remove_kdigits(case_1.0.clone(), case_1.1)
    );
    println!(
        "case_2: {}",
        SolutionAnsPython::remove_kdigits(case_2.0.clone(), case_2.1)
    );
    println!(
        "case_3: {}",
        SolutionAnsPython::remove_kdigits(case_3.0.clone(), case_3.1)
    );
}
