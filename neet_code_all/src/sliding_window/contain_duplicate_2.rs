use std::collections::{HashMap, HashSet};

struct Solution {}
impl Solution {
    // AC
    pub fn contain_nearby_duplication(nums: Vec<i32>, k: i32) -> bool {
        let mut map: HashMap<i32, Vec<i32>> = HashMap::new();

        // {value: index} なmap
        for (idx, v) in nums.iter().enumerate() {
            if let Some(map_key) = map.get_mut(v) {
                map_key.push(idx as i32);
            } else {
                map.insert(*v, vec![idx as i32]);
            }
        }

        let mut min_diff = i32::MAX;
        for (_v, mut indxs) in map {
            //println!("idxs: {:?}", indxs);
            if indxs.len() > 1 {
                indxs.sort();
                for i in 0..indxs.len() - 1 {
                    min_diff = min_diff.min(indxs[i + 1] - indxs[i]);
                }
            }
        }
        //println!("min_diff: {}", min_diff);

        min_diff as i32 <= k
    }
}

// 模範解答
struct SolutionAns {}
impl SolutionAns {
    pub fn contain_nearby_duplication(nums: Vec<i32>, k: i32) -> bool {
        let mut window = HashSet::new();
        let mut l = 0;

        for (r, &num) in nums.iter().enumerate() {
            if r as i32 - l as i32 > k {
                window.remove(&nums[l]);
                l += 1;
            }

            if window.contains(&num) {
                return true;
            }

            window.insert(num);
        }

        false
    }
}

// C++の模範解答より
struct SolutionAnsCpp {}
impl SolutionAnsCpp {
    // AC
    pub fn contain_nearby_duplication(nums: Vec<i32>, k: i32) -> bool {
        let mut number_map: HashMap<i32, i32> = HashMap::new();

        for i in 0..nums.len() {
            let num = nums[i];

            if let Some(number_map_key) = number_map.get_mut(&num) {
                if i as i32 - *number_map_key <= k {
                    return true;
                } else {
                    *number_map_key = i as i32;
                }
            } else {
                number_map.insert(num, i as i32);
            }
        }

        false
    }
}

fn main() {
    let case_1 = (vec![1, 2, 3, 1], 3);
    // => true
    let case_2 = (vec![1, 0, 1, 1], 1);
    // => true
    let case_3 = (vec![1, 2, 3, 1, 2, 3], 2);
    // => false
    let case_4 = (vec![1], 1);
    // => false

    println!(
        "case_1: {}",
        Solution::contain_nearby_duplication(case_1.0.clone(), case_1.1)
    );
    println!(
        "case_2: {}",
        Solution::contain_nearby_duplication(case_2.0.clone(), case_2.1)
    );
    println!(
        "case_3: {}",
        Solution::contain_nearby_duplication(case_3.0.clone(), case_3.1)
    );
    println!(
        "case_4: {}",
        Solution::contain_nearby_duplication(case_4.0.clone(), case_4.1)
    );

    println!(
        "case_1: {}",
        SolutionAns::contain_nearby_duplication(case_1.0.clone(), case_1.1)
    );
    println!(
        "case_2: {}",
        SolutionAns::contain_nearby_duplication(case_2.0.clone(), case_2.1)
    );
    println!(
        "case_3: {}",
        SolutionAns::contain_nearby_duplication(case_3.0.clone(), case_3.1)
    );
    println!(
        "case_4: {}",
        SolutionAns::contain_nearby_duplication(case_4.0.clone(), case_4.1)
    );

    println!(
        "case_1: {}",
        SolutionAnsCpp::contain_nearby_duplication(case_1.0.clone(), case_1.1)
    );
    println!(
        "case_2: {}",
        SolutionAnsCpp::contain_nearby_duplication(case_2.0.clone(), case_2.1)
    );
    println!(
        "case_3: {}",
        SolutionAnsCpp::contain_nearby_duplication(case_3.0.clone(), case_3.1)
    );
    println!(
        "case_4: {}",
        SolutionAnsCpp::contain_nearby_duplication(case_4.0.clone(), case_4.1)
    );
}
