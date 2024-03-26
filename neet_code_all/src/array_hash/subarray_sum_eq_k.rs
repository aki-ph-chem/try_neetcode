use std::collections::{HashMap, HashSet};

struct Solution {}
impl Solution {
    // TLE: O(N^2)
    pub fn subarray_sum_sq(nums: Vec<i32>, k: i32) -> i32 {
        // prefix_sum[i]: i - 1番目までの和
        let mut prefix_sum = vec![0; nums.len() + 1];
        prefix_sum[0] = 0;
        for i in 0..nums.len() {
            prefix_sum[i + 1] = prefix_sum[i] + nums[i];
        }
        //println!("prefix_sum: {:?}", prefix_sum);
        let mut result = 0;
        for i in 0..nums.len() {
            for j in i..nums.len() {
                if prefix_sum[j + 1] - prefix_sum[i] == k {
                    //println!("i,j: {},{}", i, j);
                    result += 1;
                }
            }
        }

        result
    }

    pub fn subarray_sum(nums: Vec<i32>, k: i32) -> i32 {
        if nums.len() == 1 {
            if k == nums[0] {
                return 1;
            }

            return 0;
        }

        // prefix_sum[i]: i - 1番目までの和
        let mut prefix_sum = vec![0; nums.len() + 1];

        let mut set: HashSet<i32> = HashSet::new();
        prefix_sum[0] = 0;
        for i in 0..nums.len() {
            prefix_sum[i + 1] = prefix_sum[i] + nums[i];
            set.insert(prefix_sum[i + 1]);
        }
        println!("prefix_sum: {:?}", prefix_sum);
        println!("set: {:?}", set);

        let mut result = 0;
        for i in 0..=nums.len() {
            let diff = k + prefix_sum[i];

            if set.contains(&diff) {
                result += 1;
            }
        }

        result
    }
}

// 模範解答
struct SolutionAns {}
impl SolutionAns {
    pub fn subarray_sum(nums: Vec<i32>, k: i32) -> i32 {
        let (mut count, mut sum, mut map) = (0, 0, HashMap::with_capacity(nums.len() / 2));

        map.insert(0, 1);
        for n in nums {
            sum += n;
            count += map.get(&(sum - k)).copied().unwrap_or(0);
            map.entry(sum).and_modify(|e| *e = *e + 1).or_insert(1);
        }

        count
    }
}

// AC
// C++の模範解答より
struct SolutionAnsCpp {}
impl SolutionAnsCpp {
    pub fn subarray_sum(nums: Vec<i32>, k: i32) -> i32 {
        let n = nums.len();
        let mut j = 0;
        let mut result = 0;
        let mut sum = 0;

        let mut map: HashMap<i32, i32> = HashMap::new();
        while j < n {
            sum += nums[j];
            if sum == k {
                result += 1;
            }

            let diff = sum - k;
            if map.contains_key(&diff) {
                result += map[&diff];
            }

            *map.entry(sum).or_default() += 1;
            j += 1;
        }

        result
    }
}

fn main() {
    let case_1 = (vec![1, 1, 1], 2);
    // => 2
    let case_2 = (vec![1, 2, 3], 3);
    // => 2
    let case_3 = (vec![1], 0);
    // => 0
    let case_4 = (vec![-1, -1, 1], 0);
    // => 1

    println!(
        "case_1: {:?}",
        Solution::subarray_sum_sq(case_1.0.clone(), case_1.1)
    );
    println!(
        "case_2: {:?}",
        Solution::subarray_sum_sq(case_2.0.clone(), case_2.1)
    );

    /*
    println!(
        "case_1: {:?}",
        Solution::subarray_sum(case_1.0.clone(), case_1.1)
    );
    println!(
        "case_2: {:?}",
        Solution::subarray_sum(case_2.0.clone(), case_2.1)
    );
    println!(
        "case_3: {:?}",
        Solution::subarray_sum(case_3.0.clone(), case_3.1)
    );
    println!(
        "case_4: {:?}",
        Solution::subarray_sum(case_4.0.clone(), case_4.1)
    );
    */

    println!(
        "case_1: {:?}",
        SolutionAns::subarray_sum(case_1.0.clone(), case_1.1)
    );
    println!(
        "case_2: {:?}",
        SolutionAns::subarray_sum(case_2.0.clone(), case_2.1)
    );
    println!(
        "case_3: {:?}",
        SolutionAns::subarray_sum(case_3.0.clone(), case_3.1)
    );
    println!(
        "case_4: {:?}",
        SolutionAns::subarray_sum(case_4.0.clone(), case_4.1)
    );

    println!(
        "case_1: {:?}",
        SolutionAnsCpp::subarray_sum(case_1.0.clone(), case_1.1)
    );
    println!(
        "case_2: {:?}",
        SolutionAnsCpp::subarray_sum(case_2.0.clone(), case_2.1)
    );
    println!(
        "case_3: {:?}",
        SolutionAnsCpp::subarray_sum(case_3.0.clone(), case_3.1)
    );
    println!(
        "case_4: {:?}",
        SolutionAnsCpp::subarray_sum(case_4.0.clone(), case_4.1)
    );
}
