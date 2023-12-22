struct Solution {}
impl Solution {
    pub fn can_partition(nums: Vec<i32>) -> bool {
        true
    }

    // ビット全探索で部分集合を数え上げる
    // O(N2^N)
    pub fn can_partition_ex(nums: Vec<i32>) -> bool {
        let n = nums.len();

        for bit in 0..(1 << n) {
            let mut sum_part_1 = 0;
            let mut sum_part_2 = 0;

            for i in 0..n {
                if bit & (1 << i) != 0 {
                    sum_part_1 += nums[i];
                } else {
                    sum_part_2 += nums[i];
                }
            }

            if sum_part_1 == sum_part_2 {
                return true;
            }
        }

        false
    }
}

fn main() {
    let case_1 = vec![1, 5, 11, 5];
    // true
    let case_2 = vec![1, 2, 3, 5];
    // false
    let case_3 = vec![
        100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100,
        100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100,
        100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100,
        100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100,
        100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100,
        100, 100, 100, 100, 100, 100, 100, 100, 100, 100,
    ];
    // true

    println!("case_1: {}", Solution::can_partition_ex(case_1.clone()));
    println!("case_2: {}", Solution::can_partition_ex(case_2.clone()));
    //println!("case_3: {}", Solution::can_partition_ex(case_3.clone())); // over flow
}
