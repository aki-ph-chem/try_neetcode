// 解けなかった
struct Solution {}
impl Solution {
    // n < kとなったときにダメ
    pub fn rotate(nums: &mut Vec<i32>, k: i32) {
        let n = nums.len() as i32;

        let mut tmp = vec![];
        for i in (n - k)..n {
            tmp.push(nums[i as usize]);
        }

        //println!("tmp: {:?}", tmp);

        for i in (k..n).rev() {
            nums[i as usize] = nums[(i - k) as usize]
        }

        for i in 0..k {
            nums[i as usize] = tmp[i as usize];
        }
    }

    // TLE: O(kN)
    pub fn rotate_2(nums: &mut Vec<i32>, k: i32) {
        let n = nums.len() as i32;

        let right = n - 1;

        for _i in 0..k {
            let mut left = 0;
            while left <= right {
                let tmp = nums[left as usize];
                nums[left as usize] = nums[right as usize];
                nums[right as usize] = tmp;

                left += 1;
            }
        }
    }
}

fn rev_array(nums: &mut Vec<i32>) {
    let (mut left, mut right) = (0, nums.len() as i32 - 1);

    while left <= right {
        let tmp = nums[left as usize];
        nums[left as usize] = nums[right as usize];
        nums[right as usize] = tmp;

        left += 1;
        right -= 1;
    }
}

// AC
// C++の模範解答より(Go,Pythonもやってることは同じだった)
struct SolutionAnsCpp {}
impl SolutionAnsCpp {
    pub fn rotate(nums: &mut Vec<i32>, k: i32) {
        let k = k % nums.len() as i32;
        Self::reverse_sub_array(nums, 0, nums.len() as i32 - 1);
        Self::reverse_sub_array(nums, 0, k - 1);
        Self::reverse_sub_array(nums, k, nums.len() as i32 - 1);
    }

    fn reverse_sub_array(nums: &mut Vec<i32>, mut left: i32, mut right: i32) {
        while left <= right {
            nums.swap(left as usize, right as usize);
            left += 1;
            right -= 1;
        }
    }
}

fn main() {
    let case_1 = (vec![1, 2, 3, 4, 5, 6, 7], 3);
    // => [5,6,7,1,2,3,4]
    let case_2 = (vec![-1, -100, 3, 99], 2);
    // => [3,99,-1,-100]

    let mut res_1 = case_1.0.clone();
    Solution::rotate(&mut res_1, case_1.1);
    println!("case_1: {:?}", res_1);

    let mut res_2 = case_2.0.clone();
    Solution::rotate(&mut res_2, case_2.1);
    println!("case_2: {:?}", res_2);

    let mut res_1 = case_1.0.clone();
    Solution::rotate_2(&mut res_1, case_1.1);
    println!("case_1: {:?}", res_1);

    let mut res_2 = case_2.0.clone();
    Solution::rotate_2(&mut res_2, case_2.1);
    println!("case_2: {:?}", res_2);

    let mut res_1 = case_1.0.clone();
    SolutionAnsCpp::rotate(&mut res_1, case_1.1);
    println!("case_1: {:?}", res_1);

    let mut res_2 = case_2.0.clone();
    SolutionAnsCpp::rotate(&mut res_2, case_2.1);
    println!("case_2: {:?}", res_2);
}
