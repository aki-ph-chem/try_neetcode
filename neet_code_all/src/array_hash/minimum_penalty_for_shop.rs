struct Solution {}
impl Solution {
    // 最初に考えたやつ:ダメ
    pub fn best_closing_time_sq(customers: String) -> i32 {
        let customers = customers.chars().collect::<Vec<char>>();
        let n = customers.len();

        let mut penalty = i32::MAX;
        let mut result = 0;
        for i_close in 0..n {
            let mut penalty_new = 0;
            for i in 0..n {
                if i < i_close {
                    match customers[i] {
                        'N' => {
                            penalty_new += 1;
                        }
                        _ => {}
                    }
                } else {
                    match customers[i] {
                        'Y' => {
                            penalty_new += 1;
                        }
                        _ => {}
                    }
                }
            }
            if penalty_new < penalty {
                penalty = penalty_new;
                result = i_close;
            }
        }

        result as i32
    }

    // AC
    pub fn best_closing_time(customers: String) -> i32 {
        let customers = customers.chars().collect::<Vec<char>>();
        let n = customers.len();
        let mut prefix_sum_left = vec![0; n + 1];
        let mut prefix_sum_right = vec![0; n + 1];
        for i in 0..n {
            let p_open = if customers[i] == 'N' { 1 } else { 0 };
            prefix_sum_left[i + 1] = prefix_sum_left[i] + p_open;
            let p_close = if customers[n - 1 - i] == 'Y' { 1 } else { 0 };
            prefix_sum_right[n - 1 - i] = prefix_sum_right[n - 1 - i + 1] + p_close;
        }

        //println!("left: {:?}", prefix_sum_left);
        //println!("right: {:?}", prefix_sum_right);

        let (mut idx_penalty, mut penalty) = (0, i32::MAX);
        for ((idx, v_left), v_right) in prefix_sum_left
            .iter()
            .enumerate()
            .zip(prefix_sum_right.iter())
        {
            if v_left + v_right < penalty {
                penalty = v_left + v_right;
                idx_penalty = idx;
            }
        }

        idx_penalty as i32
    }
}

// Kotlinの模範解答より(time: O(N), space: O(1)のKdane's Algorithmの方)
// もう一方は自分と同じ解法だった..
struct SolutionAnsKotlin {}
impl SolutionAnsKotlin {
    // AC
    pub fn best_closing_time(customers: String) -> i32 {
        let (mut current, mut max, mut close_time) = (0, 0, 0);

        for (idx, c) in customers.chars().enumerate() {
            current += if c == 'Y' { 1 } else { -1 };
            if current > max {
                max = current;
                close_time = idx + 1;
            }
        }

        close_time as i32
    }
}

// C++の模範解答より
struct SolutionAnsCpp {}
impl SolutionAnsCpp {
    // AC
    pub fn best_closing_time(customers: String) -> i32 {
        let (mut result, mut idx_max, mut penalty) = (-1_i32, 0, 0);

        for (idx, v) in customers.chars().enumerate() {
            if v == 'Y' {
                penalty += 1;
            } else {
                penalty -= 1;
            }

            if penalty > idx_max {
                idx_max = penalty;
                result = idx as i32;
            }
        }

        result += 1;
        result
    }
}

fn main() {
    let case_1 = "YYNY".to_string();
    // => 2
    let case_2 = "NNNNN".to_string();
    // => 0
    let case_3 = "YYYY".to_string();
    // => 4

    println!("case_1: {}", Solution::best_closing_time(case_1.clone()));
    println!("case_2: {}", Solution::best_closing_time(case_2.clone()));
    println!("case_3: {}", Solution::best_closing_time(case_3.clone()));

    println!(
        "case_1: {}",
        SolutionAnsKotlin::best_closing_time(case_1.clone())
    );
    println!(
        "case_2: {}",
        SolutionAnsKotlin::best_closing_time(case_2.clone())
    );
    println!(
        "case_3: {}",
        SolutionAnsKotlin::best_closing_time(case_3.clone())
    );

    println!(
        "case_1: {}",
        SolutionAnsCpp::best_closing_time(case_1.clone())
    );
    println!(
        "case_2: {}",
        SolutionAnsCpp::best_closing_time(case_2.clone())
    );
    println!(
        "case_3: {}",
        SolutionAnsCpp::best_closing_time(case_3.clone())
    );
}
