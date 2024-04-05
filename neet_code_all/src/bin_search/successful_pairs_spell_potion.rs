struct Solution {}
impl Solution {
    // TLE
    // time: O(N^2)
    pub fn successful_pairs_sq(spells: Vec<i32>, potions: Vec<i32>, success: i64) -> Vec<i32> {
        let mut result = vec![];
        for s in &spells {
            let mut len_success = 0;
            for p in &potions {
                if (*s as i64) * (*p as i64) >= success {
                    len_success += 1;
                }
            }
            result.push(len_success);
        }

        result
    }

    // AC
    // time: O(NlogN)
    pub fn successful_pairs(spells: Vec<i32>, potions: Vec<i32>, success: i64) -> Vec<i32> {
        let mut potions = potions;
        potions.sort();
        let mut result = vec![];

        for s in spells {
            let (mut left, mut right) = (0, potions.len() as i32 - 1);
            let mut len_success = 0;
            while left <= right {
                let mid = left + (right - left) / 2;
                if success <= potions[mid as usize] as i64 * s as i64 {
                    len_success += right - mid + 1;
                    right = mid - 1;
                } else {
                    left = mid + 1;
                }
            }
            result.push(len_success);
        }

        result
    }
}

// C++の模範解答より
struct SolutionAnsCpp {}
impl SolutionAnsCpp {
    // AC
    pub fn successful_pairs(spells: Vec<i32>, potions: Vec<i32>, success: i64) -> Vec<i32> {
        let mut potions = potions;
        potions.sort();
        let m = potions.len() as i32;

        let mut result = vec![];
        for spell in spells {
            let (mut left, mut right) = (0, m - 1);
            while left <= right {
                let mid = left + (right - left) / 2;
                let prod_strength = potions[mid as usize] as i64 * spell as i64;

                if prod_strength < success {
                    left = mid + 1;
                } else {
                    right = mid - 1;
                }
            }
            result.push(m - left);
        }

        result
    }
}

fn main() {
    let case_1 = (vec![5, 1, 3], vec![1, 2, 3, 4, 5], 7);
    // => [4,0,3]
    let case_2 = (vec![3, 1, 2], vec![8, 5, 8], 16);
    // => [2,0,2]

    println!(
        "case_1: {:?}",
        Solution::successful_pairs_sq(case_1.0.clone(), case_1.1.clone(), case_1.2)
    );
    println!(
        "case_2: {:?}",
        Solution::successful_pairs_sq(case_2.0.clone(), case_2.1.clone(), case_2.2)
    );

    println!(
        "case_1: {:?}",
        Solution::successful_pairs(case_1.0.clone(), case_1.1.clone(), case_1.2)
    );
    println!(
        "case_2: {:?}",
        Solution::successful_pairs(case_2.0.clone(), case_2.1.clone(), case_2.2)
    );

    println!(
        "case_1: {:?}",
        SolutionAnsCpp::successful_pairs(case_1.0.clone(), case_1.1.clone(), case_1.2)
    );
    println!(
        "case_2: {:?}",
        SolutionAnsCpp::successful_pairs(case_2.0.clone(), case_2.1.clone(), case_2.2)
    );
}
