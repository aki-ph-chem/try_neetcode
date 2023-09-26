use std::collections::HashSet;

// 自分の回答: Accepted
struct Solution {}
impl Solution {
    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        let mut num_set = HashSet::new();
        let mut num_set_size = 0;
        for v in nums {
            num_set_size += 1;
            num_set.insert(v);
            if num_set_size != num_set.len() {
                return true;
            }
        }
        return false;
    }
}

// 模範解答
struct SolutionAns {}
impl SolutionAns {
    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        let mut map = HashSet::new();

        for &n in nums.iter() {
            if map.contains(&n) {
                return true;
            }

            map.insert(n);
        }

        false
    }
}

fn main() {
    let nums_1 = vec![1, 2, 1, 3];
    let nums_2 = vec![1, 2, 3, 4];
    let nums_3 = vec![1, 1, 1, 3, 3, 4, 3, 2, 4, 2];

    println!("nums_1");
    if Solution::contains_duplicate(nums_1) {
        println!("Duplicate");
    }

    println!("nums_2");
    if Solution::contains_duplicate(nums_2) {
        println!("Duplicate");
    }

    println!("nums_3");
    if Solution::contains_duplicate(nums_3) {
        println!("Duplicate");
    }

    let nums_1 = vec![1, 2, 1, 3];
    let nums_2 = vec![1, 2, 3, 4];
    let nums_3 = vec![1, 1, 1, 3, 3, 4, 3, 2, 4, 2];

    println!("nums_1");
    if SolutionAns::contains_duplicate(nums_1) {
        println!("Duplicate");
    }

    println!("nums_2");
    if SolutionAns::contains_duplicate(nums_2) {
        println!("Duplicate");
    }

    println!("nums_3");
    if SolutionAns::contains_duplicate(nums_3) {
        println!("Duplicate");
    }
}
