use std::collections::HashSet;

struct Solution {}
impl Solution {
    // AC
    pub fn find_difference(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<Vec<i32>> {
        let mut set_1 = HashSet::new();
        let mut set_2 = HashSet::new();

        for v in &nums1 {
            set_1.insert(v);
        }
        for v in &nums2 {
            set_2.insert(v);
        }

        let mut result = vec![vec![]; 2];
        for v in &set_1 {
            if !set_2.contains(v) {
                result[0].push(**v);
            }
        }
        for v in set_2 {
            if !set_1.contains(v) {
                result[1].push(*v);
            }
        }

        result
    }
}

fn main() {
    let case_1 = (vec![1, 2, 3], vec![2, 4, 6]);
    // => [[1,3], [4,6]]
    let case_2 = (vec![1, 2, 3, 3], vec![1, 2, 2, 2]);
    // => [[3], []]

    println!(
        "case_1: {:?}",
        Solution::find_difference(case_1.0.clone(), case_1.1.clone())
    );
    println!(
        "case_2: {:?}",
        Solution::find_difference(case_2.0.clone(), case_2.1.clone())
    );
}
