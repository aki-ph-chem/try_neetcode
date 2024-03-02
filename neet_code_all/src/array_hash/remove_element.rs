// 解けなかった
struct Solution {}
impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        /*
        for i in 0..nums.len() {
            if nums[i] == val {
                for j in i..nums.len() - 1 {
                    nums[j] = nums[j + 1];
                }
                println!("nums: {:?}", nums);
                //nums.pop();
            }
        }
        */

        let mut i = 0;
        let mut j = 0;
        while i < nums.len() {
            if nums[i] == val {
                j = i;
                while nums[j] == val {
                    j += 1;
                }

                for k in i..nums.len() {
                    if k + j - i < nums.len() {
                        nums[k] = nums[k + j - i];
                    } else {
                        break;
                    }
                }
            }

            i += 1;
        }

        nums.len() as i32
    }
}

struct SolutionAns {}
impl SolutionAns {
    // 解1
    fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        while let Some(idx) = nums.iter().position(|v| *v == val) {
            nums.swap_remove(idx);
        }

        nums.len() as i32
    }

    // 解2
    pub fn remove_element_2(nums: &mut Vec<i32>, val: i32) -> i32 {
        let mut k = 0;
        for i in 0..nums.len() {
            if nums[i] != val {
                nums[k] = nums[i];
                k += 1;
            }
        }

        k as i32
    }
}

// 模範解答

fn main() {
    let case_1 = (vec![3, 2, 2, 3], 3);
    // => 2, nums = [2, 2, _, _]
    let case_2 = (vec![0, 1, 2, 2, 3, 0, 4, 2], 2);
    // => 5, nums = [0, 1, 4, 0, 3, _, _, _]
    let case_3 = (vec![3, 3, 2, 2, 3], 3);

    /*
    let mut res_1 = case_1.0.clone();
    println!("case_1: {}", Solution::remove_element(&mut res_1, case_1.1));
    println!("{:?}", res_1);

    let mut res_2 = case_2.0.clone();
    println!("case_2: {}", Solution::remove_element(&mut res_2, case_2.1));
    println!("{:?}", res_2);

    let mut res_3 = case_3.0.clone();
    println!("case_3: {}", Solution::remove_element(&mut res_3, case_3.1));
    println!("{:?}", res_3);
    */

    let mut res_1 = case_1.0.clone();
    println!(
        "case_1: {}",
        SolutionAns::remove_element(&mut res_1, case_1.1)
    );
    println!("{:?}", res_1);

    let mut res_2 = case_2.0.clone();
    println!(
        "case_2: {}",
        SolutionAns::remove_element(&mut res_2, case_2.1)
    );
    println!("{:?}", res_2);

    let mut res_3 = case_3.0.clone();
    println!(
        "case_3: {}",
        SolutionAns::remove_element(&mut res_3, case_3.1)
    );
    println!("{:?}", res_3);
}
