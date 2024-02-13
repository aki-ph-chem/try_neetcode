use std::collections::HashSet;

// AC
// ビット演算で部分集合を列挙して、ダブり判定をHashSetで行う
struct Solution {}
impl Solution {
    pub fn subsets_with_dup(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let n = nums.len();
        let mut result = vec![];
        let mut set: HashSet<Vec<i32>> = HashSet::new();

        for i in 0..(1 << n) {
            let mut sub_set = vec![];
            for j in 0..n {
                if i & (1 << j) != 0 {
                    sub_set.push(nums[j]);
                }
            }
            sub_set.sort();
            if !set.contains(&sub_set) {
                set.insert(sub_set.clone());
                result.push(sub_set);
            }
        }

        result
    }
}
fn main() {
    let case_1 = vec![1, 2, 2];
    let case_2 = vec![0];

    println!("case_1: {:?}", Solution::subsets_with_dup(case_1.clone()));
    println!("case_2: {:?}", Solution::subsets_with_dup(case_2.clone()));
}
