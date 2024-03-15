// 解けなかった
struct Solution {}
impl Solution {
    pub fn add_to_array_form(mut num: Vec<i32>, mut k: i32) -> Vec<i32> {
        let mut result = vec![];
        let mut carry = 0;

        while let Some(v_back) = num.pop() {
            while k > 0 {
                let digit = (v_back + k % 10 + carry) % 10;
                carry = (v_back + k % 10 + carry) / 10;
                k /= 10;
                result.push(digit);
            }
        }

        result
    }
}

// 模範解答
struct SolutionAns {}
impl SolutionAns {
    pub fn add_to_array_form(num: Vec<i32>, k: i32) -> Vec<i32> {
        let mut num: Vec<i32> = num.into_iter().rev().collect();
        let (mut k, mut i) = (k, 0);

        while k > 0 {
            let digit = k % 10;

            if i < num.len() {
                num[i] += digit;
            } else {
                num.push(digit);
            }

            let carry = num[i] / 10;
            num[i] %= 10;

            k /= 10;
            k += carry;
            i += 1;
        }

        num.into_iter().rev().collect()
    }
}

fn main() {
    let case_1 = (vec![1, 2, 0, 0], 34);
    // => [1,2,3,4]
    let case_2 = (vec![2, 7, 4], 181);
    // => [4,5,5]
    let case_3 = (vec![2, 1, 5], 806);
    // => [1,0,2,1]

    /*
    println!(
        "case_1: {:?}",
        Solution::add_to_array_form(case_1.0.clone(), case_1.1)
    );
    println!(
        "case_2: {:?}",
        Solution::add_to_array_form(case_2.0.clone(), case_2.1)
    );
    println!(
        "case_3: {:?}",
        Solution::add_to_array_form(case_3.0.clone(), case_3.1)
    );
    */

    println!(
        "case_1: {:?}",
        SolutionAns::add_to_array_form(case_1.0.clone(), case_1.1)
    );
    println!(
        "case_2: {:?}",
        SolutionAns::add_to_array_form(case_2.0.clone(), case_2.1)
    );
    println!(
        "case_3: {:?}",
        SolutionAns::add_to_array_form(case_3.0.clone(), case_3.1)
    );
}
