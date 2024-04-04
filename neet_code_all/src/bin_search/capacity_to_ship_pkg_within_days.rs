// 解けなかった
struct Solution {}
impl Solution {
    pub fn ship_within_days(weights: Vec<i32>, days: i32) -> i32 {
        let mut weights = weights;
        weights.sort();
        let (mut left, mut right) = (0_i32, weights.len() as i32 - 1);
        let mut result = weights[right as usize];

        result
    }

    // 重さ制限capでdays日で完了できるか否かの判定
    pub fn can_complete(weights: &Vec<i32>, mut days: i32, cap: i32) -> bool {
        let mut tmp_weight = 0;

        for v in weights.iter().rev() {
            if tmp_weight + v < cap {
                tmp_weight += v;
            } else {
                days -= 1;
                tmp_weight = *v;
            }
            //println!("tmp_weight: {}", tmp_weight);
        }

        //println!("days: {}", days);
        days == 0
    }
}

// AC
// Pythonの模範解答より
struct SolutionAnsPython {}
impl SolutionAnsPython {
    pub fn ship_within_days(weights: Vec<i32>, days: i32) -> i32 {
        // 重さ制限capでdaysで完了できるか否か
        let can_ship = |cap: i32| {
            let (mut ships, mut current_cap) = (1, cap);
            for w in &weights {
                if current_cap - w < 0 {
                    ships += 1;
                    current_cap = cap;
                }
                current_cap -= w;
            }

            ships <= days
        };

        // left: weightの最大値
        // right: weightの和
        let (mut left, mut right) = (0, 0);
        for v in &weights {
            left = left.max(*v);
            right += v;
        }
        let mut result = right;

        while left <= right {
            let cap = left + (right - left) / 2;
            if can_ship(cap) {
                result = result.min(cap);
                right = cap - 1;
            } else {
                left = cap + 1;
            }
        }

        result
    }
}

fn main() {
    let case_1 = (vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10], 5);
    // => 15
    let case_2 = (vec![3, 2, 2, 4, 1, 4], 3);
    // => 6
    let case_3 = (vec![1, 2, 3, 1, 1], 4);
    // => 3

    /*
    println!(
        "case_1: {:?}",
        Solution::can_complete(&case_1.0, case_1.1, 15)
    );
    println!(
        "case_2: {:?}",
        Solution::can_complete(&case_2.0, case_2.1, 6)
    );
    */

    println!(
        "case_1: {:?}",
        SolutionAnsPython::ship_within_days(case_1.0.clone(), case_1.1)
    );
    println!(
        "case_2: {:?}",
        SolutionAnsPython::ship_within_days(case_2.0.clone(), case_2.1)
    );
    println!(
        "case_3: {:?}",
        SolutionAnsPython::ship_within_days(case_3.0.clone(), case_3.1)
    );
}
