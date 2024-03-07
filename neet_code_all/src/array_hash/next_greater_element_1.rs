use std::collections::HashMap;

// 解けなかった
struct Solution {}
impl Solution {
    pub fn next_greater_element(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let mut result = vec![];

        for n_1 in nums1.iter() {
            for (i_2, n_2) in nums2.iter().enumerate() {
                if n_1 == n_2 {
                    let mut i_tmp = i_2;
                    //println!("before while: i_tmp: {}", i_tmp);
                    while i_tmp < nums2.len() - 1 {
                        //while i_tmp < nums2.len() {
                        if nums2[i_tmp] > *n_1 {
                            result.push(nums2[i_tmp]);
                            break;
                        }
                        i_tmp += 1;
                    }

                    //println!("after while: i_tmp: {}", i_tmp);
                    if i_tmp == nums2.len() - 1 {
                        result.push(-1);
                    }
                    //println!("result: {:?}", result);
                }
            }
        }

        result
    }
}

// 模範解答
struct SolutionAns {}
impl SolutionAns {
    pub fn next_greater_element(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        // nums1に対する {value, index}のmap
        let map: HashMap<i32, usize> = nums1
            .iter()
            .cloned()
            .enumerate()
            // key <-> value と入れ替える
            .map(|idx_value| (idx_value.1, idx_value.0))
            .collect();

        let mut result = vec![0; nums1.len()];

        for (idx, val) in nums2.iter().enumerate() {
            if let Some(map_key) = map.get(val) {
                let next_greater = nums2.iter().skip(idx).find(|&x| x > val).unwrap_or(&-1);
                result[*map_key] = *next_greater;
            }
        }

        result
    }
}

fn main() {
    let case_1 = (vec![4, 1, 2], vec![1, 3, 4, 2]);
    // => [-1,3,-1]
    let case_2 = (vec![2, 4], vec![1, 2, 3, 4]);
    // => [3,-1]
    let case_3 = (vec![3, 1, 5, 7, 9, 2, 6], vec![1, 2, 3, 5, 6, 7, 9, 11]);
    //=> [5,2,6,9,11,3,7]

    /*
    println!(
        "case_1: {:?}",
        Solution::next_greater_element(case_1.0.clone(), case_1.1.clone())
    );
    println!(
        "case_2: {:?}",
        Solution::next_greater_element(case_2.0.clone(), case_2.1.clone())
    );
    println!(
        "case_3: {:?}",
        Solution::next_greater_element(case_3.0.clone(), case_3.1.clone())
    );
    */

    println!(
        "case_1: {:?}",
        SolutionAns::next_greater_element(case_1.0.clone(), case_1.1.clone())
    );
    println!(
        "case_2: {:?}",
        SolutionAns::next_greater_element(case_2.0.clone(), case_2.1.clone())
    );
    println!(
        "case_3: {:?}",
        SolutionAns::next_greater_element(case_3.0.clone(), case_3.1.clone())
    );
}
