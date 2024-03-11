// 解けなかった
struct Solution {}
impl Solution {
    pub fn remvo_duplicates(nums: &mut Vec<i32>) -> i32 {
        let mut l = 0;

        for r in 0..nums.len() {
            if nums[l] != nums[r] {
                nums[l + 1] = nums[r];
                l += 1;
            }
        }

        l as i32 + 1
    }
}

// C++の模範解答より
struct SolutionAns {}
impl SolutionAns {
    pub fn remvo_duplicates(nums: &mut Vec<i32>) -> i32 {
        let (k, mut l, mut count) = (2_i32, 1_i32, 1_i32);

        for r in 1..nums.len() {
            if nums[r as usize] == nums[r as usize - 1] {
                if count < k {
                    nums[l as usize] = nums[r as usize];
                    l += 1;
                    count += 1;
                }
            } else {
                count = 1;
                nums[l as usize] = nums[r as usize];
                l += 1;
            }
        }

        l
    }
}

fn main() {
    let case_1 = vec![1, 1, 1, 2, 2, 3];
    // => [1,1,2,2,3,_]
    let case_2 = vec![0, 0, 1, 1, 1, 1, 2, 3, 3];
    // => [0,0,1,1,2,3,3,_,_]

    /*
    let mut res_1 = case_1.clone();
    println!("case_1: {}", Solution::remvo_duplicates(&mut res_1));
    println!("{:?}", res_1);
    */

    let mut res_1_ans = case_1.clone();
    println!("case_1: {}", SolutionAns::remvo_duplicates(&mut res_1_ans));
    println!("res_1_ans: {:?}", res_1_ans);

    let mut res_2_ans = case_2.clone();
    println!("case_2: {}", SolutionAns::remvo_duplicates(&mut res_2_ans));
    println!("res_2_ans: {:?}", res_2_ans);
}
