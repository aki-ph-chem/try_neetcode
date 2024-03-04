use std::collections::HashSet;

// in-place 縛りあり
struct Solution {}
impl Solution {
    // AC: C++の模範解答に近い
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let mut l = 0;

        for r in 0..nums.len() {
            if nums[l] != nums[r] {
                nums[l + 1] = nums[r];
                l += 1;
            }
        }

        l as i32 + 1
    }
    // 縛り無視の実装(力技)
    // AC
    pub fn remove_duplicates_out_place(nums: &mut Vec<i32>) -> i32 {
        let mut set = HashSet::new();
        let mut result = vec![];

        for v in nums.iter() {
            if !set.contains(v) {
                set.insert(v);
                result.push(*v);
            }
        }

        *nums = result.clone();
        result.len() as i32
    }
}

// 模範解答
struct SolutionAns {}
impl SolutionAns {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let mut dup_count = 0;

        for i in 1..nums.len() {
            if nums[i] == nums[i - 1] {
                dup_count += 1;
            }

            nums[i - dup_count] = nums[i];
        }

        (nums.len() - dup_count) as i32
    }
}

// C++の模範解答より
struct SolutionAnsCpp {}
impl SolutionAnsCpp {
    // AC
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let mut idx = 1;
        let n = nums.len();

        for i in 1..n {
            if nums[i] != nums[i - 1] {
                nums[idx] = nums[i];
                idx += 1;
            }
        }

        idx as i32
    }
}

//numsはsort済み => ダブってる要素は必ず隣
fn main() {
    let case_1 = vec![1, 1, 2];
    // => 2, [1,2,_]
    let case_2 = vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4];
    // => 5 , [0,1,2,3,4,_,_,_,_,_]

    let mut res_1 = case_1.clone();
    println!("case_1: {}", Solution::remove_duplicates(&mut res_1));
    println!("{:?}", res_1);

    let mut res_2 = case_2.clone();
    println!("case_2: {}", Solution::remove_duplicates(&mut res_2));
    println!("{:?}", res_2);

    let mut res_1 = case_1.clone();
    println!(
        "case_1: {}",
        Solution::remove_duplicates_out_place(&mut res_1)
    );
    println!("{:?}", res_1);

    let mut res_2 = case_2.clone();
    println!(
        "case_2: {}",
        Solution::remove_duplicates_out_place(&mut res_2)
    );
    println!("{:?}", res_2);

    let mut res_1 = case_1.clone();
    println!("case_1: {}", SolutionAns::remove_duplicates(&mut res_1));
    println!("{:?}", res_1);

    let mut res_2 = case_2.clone();
    println!("case_2: {}", SolutionAns::remove_duplicates(&mut res_2));
    println!("{:?}", res_2);

    let mut res_1 = case_1.clone();
    println!("case_1: {}", SolutionAnsCpp::remove_duplicates(&mut res_1));
    println!("{:?}", res_1);

    let mut res_2 = case_2.clone();
    println!("case_2: {}", SolutionAnsCpp::remove_duplicates(&mut res_2));
    println!("{:?}", res_2);
}
