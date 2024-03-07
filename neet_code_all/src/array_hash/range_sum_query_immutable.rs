// AC
struct Solution {
    prefix_sum: Vec<i32>,
}
impl Solution {
    pub fn new(nums: Vec<i32>) -> Self {
        let mut prefix_sum = vec![0; nums.len() + 1];
        for i in 0..nums.len() {
            prefix_sum[i + 1] = prefix_sum[i] + nums[i];
        }

        Self { prefix_sum }
    }

    pub fn sum_range(&self, left: i32, right: i32) -> i32 {
        let (left, right) = (left as usize, right as usize);

        self.prefix_sum[right + 1] - self.prefix_sum[left]
    }
}

// 模範解答
struct SolutionAns {
    prefix: Vec<i32>,
}
impl SolutionAns {
    pub fn new(nums: Vec<i32>) -> Self {
        let mut prefix = vec![];
        let mut current = 0;

        for n in nums {
            current += n;
            prefix.push(current);
        }

        Self { prefix }
    }

    pub fn sum_range(&self, left: i32, right: i32) -> i32 {
        let right_sum = self.prefix[right as usize];
        let left_sum = if left > 0 {
            self.prefix[(left - 1) as usize]
        } else {
            0
        };

        right_sum - left_sum
    }
}

fn main() {
    let case_1 = vec![-2, 0, 3, -5, 2, -1];
    let res_1 = Solution::new(case_1.clone());
    println!("res_1.sum_range(0,2): {}", res_1.sum_range(0, 2));
    // => 1
    println!("res_1.sum_range(2,5): {}", res_1.sum_range(2, 5));
    // => -1
    println!("res_1.sum_range(0,5): {}", res_1.sum_range(0, 5));
    // => -3
}
