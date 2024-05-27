// 題意が理解できなかった
struct Solution;
impl Solution {
    pub fn validate_stack_sequences(pushed: Vec<i32>, popped: Vec<i32>) -> bool {
        true
    }
}

// Pythonの模範解答より
struct SolutionAnsPython;
impl SolutionAnsPython {
    // AC
    pub fn validate_stack_sequences(pushed: Vec<i32>, popped: Vec<i32>) -> bool {
        let mut idx = 0;
        let mut stack = vec![];
        for n in pushed {
            stack.push(n);

            while idx < popped.len() && stack.len() != 0 && popped[idx] == *stack.last().unwrap() {
                stack.pop();
                idx += 1;
            }
        }

        stack.len() == 0
    }
}

fn main() {
    let case_1 = (vec![1, 2, 3, 4, 5], vec![4, 5, 3, 2, 1]);
    // => true
    let case_2 = (vec![1, 2, 3, 4, 5], vec![4, 3, 5, 1, 2]);
    // => false

    println!(
        "case_1: {}",
        SolutionAnsPython::validate_stack_sequences(case_1.0.clone(), case_1.1.clone())
    );
    println!(
        "case_2: {}",
        SolutionAnsPython::validate_stack_sequences(case_2.0.clone(), case_2.1.clone())
    );
}
