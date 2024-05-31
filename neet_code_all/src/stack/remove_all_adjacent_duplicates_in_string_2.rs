// 解けなかった
struct Solution;
impl Solution {
    pub fn remove_duplicates(s: String, k: i32) -> String {
        let mut stack = vec![];
        for c in s.chars() {
            stack.push(c);
            let mut len_stack = stack.len();
            for i in (0..len_stack).rev() {}
        }

        stack.iter().collect::<String>()
    }
}

// 模範解答
struct SolutionAns;
impl SolutionAns {
    pub fn remove_duplicates(s: String, k: i32) -> String {
        let mut stack: Vec<(char, usize)> = vec![];
        for c in s.chars() {
            if !stack.is_empty() && stack.last().unwrap().0 == c {
                let mut last = stack.pop().unwrap();
                last.1 += 1;
                stack.push(last);
            } else {
                stack.push((c, 1));
            }
            if stack.last().unwrap().1 == k as usize {
                stack.pop();
            }
        }

        stack.iter().fold(String::new(), |acc, &(c, count)| {
            acc + &c.to_string().repeat(count)
        })
    }

    // AC
    pub fn remove_duplicates_2(s: String, k: i32) -> String {
        let mut stack: Vec<(char, usize)> = vec![];
        for c in s.chars() {
            if let Some(stack_last) = stack.last() {
                if stack_last.0 == c {
                    let mut last = stack.pop().unwrap();
                    last.1 += 1;
                    stack.push(last);
                } else {
                    stack.push((c, 1));
                }
            } else {
                stack.push((c, 1));
            }

            if let Some(stack_last) = stack.last() {
                if stack_last.1 == k as usize {
                    stack.pop();
                }
            }
        }

        stack.iter().fold(String::new(), |acc, &(c, count)| {
            acc + &c.to_string().repeat(count)
        })
    }
}

fn main() {
    let case_1 = ("abcd".to_string(), 2);
    // => "abcd"
    let case_2 = ("deeedbbcccbdaa".to_string(), 3);
    // => "aa"
    let case_3 = ("pbbcggttciiippooaais".to_string(), 2);
    // => "ps"

    /*
    println!(
        "case_2: {}",
        Solution::remove_duplicates(case_2.0.clone(), case_2.1)
    )
    */

    println!(
        "case_1: {}",
        SolutionAns::remove_duplicates(case_1.0.clone(), case_1.1)
    );
    println!(
        "case_2: {}",
        SolutionAns::remove_duplicates(case_2.0.clone(), case_2.1)
    );
    println!(
        "case_3: {}",
        SolutionAns::remove_duplicates(case_3.0.clone(), case_3.1)
    );

    println!(
        "case_1: {}",
        SolutionAns::remove_duplicates_2(case_1.0.clone(), case_1.1)
    );
    println!(
        "case_2: {}",
        SolutionAns::remove_duplicates_2(case_2.0.clone(), case_2.1)
    );
    println!(
        "case_3: {}",
        SolutionAns::remove_duplicates_2(case_3.0.clone(), case_3.1)
    );
}
