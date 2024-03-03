struct Solution {}
impl Solution {
    // O(kN) 実質O(N^2)
    // AC
    pub fn num_of_subarray(arr: Vec<i32>, k: i32, threshold: i32) -> i32 {
        let mut result = 0;

        for i in 0..(arr.len() as i32 - k + 1) {
            let mut sub_array_sum = 0;
            for j in i..(i + k) {
                sub_array_sum += arr[j as usize];
            }

            if sub_array_sum / k >= threshold {
                result += 1;
            }
        }

        result
    }

    //AC
    //O(N)
    pub fn num_of_subarray_2(arr: Vec<i32>, k: i32, threshold: i32) -> i32 {
        let mut result = 0;
        let mut sub_array_sum = 0;

        let mut i = 0;
        while i < k {
            sub_array_sum += arr[i as usize];
            i += 1;
        }
        if sub_array_sum / k >= threshold {
            result += 1;
        }

        while i < arr.len() as i32 {
            sub_array_sum -= arr[(i - k) as usize];
            sub_array_sum += arr[i as usize];

            if sub_array_sum / k >= threshold {
                result += 1;
            }

            i += 1;
        }

        result
    }
}

// 長さがkで平均値がthreshold以下の部分列の個数

fn main() {
    let case_1 = (vec![2, 2, 2, 2, 5, 5, 5, 8], 3, 4);
    // => 3
    let case_2 = (vec![11, 13, 17, 23, 29, 31, 7, 5, 2, 3], 3, 6);
    // => 6

    println!(
        "case_1: {}",
        Solution::num_of_subarray(case_1.0.clone(), case_1.1, case_1.2)
    );
    println!(
        "case_2: {}",
        Solution::num_of_subarray(case_2.0.clone(), case_2.1, case_2.2)
    );

    println!(
        "case_1: {}",
        Solution::num_of_subarray_2(case_1.0.clone(), case_1.1, case_1.2)
    );
    println!(
        "case_2: {}",
        Solution::num_of_subarray_2(case_2.0.clone(), case_2.1, case_2.2)
    );
}
