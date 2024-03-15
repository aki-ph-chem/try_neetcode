struct Solution {}
impl Solution {
    // AC
    pub fn shuffle(nums: Vec<i32>, n: i32) -> Vec<i32> {
        let mut result = vec![];

        let mut i = 0;
        let mut j = n;
        while i < n && j < 2 * n {
            result.push(nums[i as usize]);
            i += 1;
            result.push(nums[j as usize]);
            j += 1;
        }

        result
    }
}

// Cの模範解答より
struct SolutionC {}
impl SolutionC {
    pub fn shuffle(nums: Vec<i32>, n: i32) -> Vec<i32> {
        let mut result = vec![0; nums.len()];

        let (mut i, mut k) = (0, 0);

        while i < n as usize {
            result[k] = nums[i];
            k += 1;
            result[k] = nums[i + n as usize];
            k += 1;
            i += 1;
        }

        result
    }
}

fn main() {
    let case_1 = (vec![2, 5, 1, 3, 4, 7], 3);
    // => [2,3,5,4,1,7]
    let case_2 = (vec![1, 2, 3, 4, 4, 3, 2, 1], 4);
    // => [1,4,2,3,3,2,4,1]
    let case_3 = (vec![1, 1, 2, 2], 2);
    // => [1,2,1,2]

    println!(
        "case_1: {:?}",
        Solution::shuffle(case_1.0.clone(), case_1.1)
    );
    println!(
        "case_2: {:?}",
        Solution::shuffle(case_2.0.clone(), case_2.1)
    );
    println!(
        "case_3: {:?}",
        Solution::shuffle(case_3.0.clone(), case_3.1)
    );

    println!(
        "case_1: {:?}",
        SolutionC::shuffle(case_1.0.clone(), case_1.1)
    );
    println!(
        "case_2: {:?}",
        SolutionC::shuffle(case_2.0.clone(), case_2.1)
    );
    println!(
        "case_3: {:?}",
        SolutionC::shuffle(case_3.0.clone(), case_3.1)
    );
}
