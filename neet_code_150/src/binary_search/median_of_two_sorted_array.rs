// 解けなかった
// 縛り: O(log(m + n))
struct Solution {}
impl Solution {
    pub fn find_median_sorted_arrays(num1: Vec<i32>, num2: Vec<i32>) -> f64 {
        let (mut left_1, mut right_1) = (0, num1.len() as i32 - 1);
        let (mut left_2, mut right_2) = (0, num2.len() as i32 - 1);

        // num2[0]がnum1のなかでどのへんにあるか調べる
        while left_1 <= right_1 {
            let mid = (left_1 + right_1) / 2;
            if num1[mid as usize] == num2[left_2] {
                break;
            }

            if num1[mid as usize] < num2[left_2] {
                left_1 = mid + 1;
            } else {
                right_1 = mid - 1;
            }
        }

        println!("left_1, right_1: {},{}", left_1, right_1);

        1.2
    }
}

// 模範解答
struct SolutionAns {}
impl SolutionAns {
    pub fn find_median_sorted_arrays_1(mut nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        for val in nums2 {
            nums1.insert(nums1.binary_search(&val).unwrap_or_else(|e| e), val);
        }
        if nums1.len() % 2 == 0 {
            (nums1[(nums1.len() - 1) / 2] + nums1[nums1.len() / 2]) as f64 / 2.0
        } else {
            nums1[(nums1.len() - 1) / 2] as f64
        }
    }

    pub fn find_median_sorted_arrays_2(mut nums1: Vec<i32>, mut nums2: Vec<i32>) -> f64 {
        let total = nums1.len() + nums2.len();
        let half = total / 2;

        if nums1.len() > nums2.len() {
            std::mem::swap(&mut nums1, &mut nums2);
        }

        let mut left = 0;
        let mut right = nums1.len();

        while left <= right {
            let mid = left + (right - left) / 2;
            let pointer = half - mid;

            let base_left = if mid > 0 {
                nums1[mid - 1] as f64
            } else {
                f64::MIN
            };

            let base_right = if mid < nums1.len() {
                nums1[mid] as f64
            } else {
                f64::MAX
            };

            let ref_left = if pointer > 0 {
                nums2[pointer - 1] as f64
            } else {
                f64::MIN
            };

            let ref_right = if pointer < nums2.len() {
                nums2[pointer] as f64
            } else {
                f64::MAX
            };

            if base_left <= ref_right && ref_left <= base_right {
                if total % 2 == 1 {
                    return base_right.min(ref_right);
                } else {
                    return (base_left.max(ref_left) + base_right.min(ref_right)) / 2.0;
                }
            } else if base_left > ref_right {
                right = mid - 1;
            } else {
                left = mid + 1;
            }
        }

        panic!("Arrays are not sorted");
    }
}

// C++の模範解答より
struct SolutionAnsCpp {}
impl SolutionAnsCpp {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let m = nums1.len() as i32;
        let n = nums2.len() as i32;

        if m > n {
            return Self::find_median_sorted_arrays(nums2, nums1);
        }

        let total = m + n;
        let (mut low, mut high) = (0, m);
        let mut result = 0_f64;

        while low <= high {
            // nums1
            let i = low + (high - low) / 2;
            // nums2
            let j = (total + 1) / 2 - i;

            let left_1 = if i > 0 {
                nums1[i as usize - 1]
            } else {
                i32::MIN
            };
            let right_1 = if i < m { nums1[i as usize] } else { i32::MAX };
            let left_2 = if j > 0 {
                nums2[j as usize - 1]
            } else {
                i32::MIN
            };
            let right_2 = if j < n { nums2[j as usize] } else { i32::MAX };

            if left_1 <= right_2 && left_2 <= right_1 {
                if total % 2 == 0 {
                    result = (std::cmp::max(left_1, left_2) + std::cmp::min(right_1, right_2))
                        as f64
                        / 2_f64;
                } else {
                    result = std::cmp::max(left_1, left_2) as f64;
                }
                break;
            } else if left_2 < left_1 {
                high = i - 1;
            } else {
                low = i + 1;
            }
        }

        result
    }
}

fn main() {
    let case_1 = (vec![1, 3], vec![2]);
    // => 2
    let case_2 = (vec![1, 2], vec![3, 4]);
    // => 2.5

    println!(
        "case_1: {}",
        SolutionAnsCpp::find_median_sorted_arrays(case_1.0.clone(), case_1.1.clone())
    );
    println!(
        "case_2: {}",
        SolutionAnsCpp::find_median_sorted_arrays(case_2.0.clone(), case_2.1.clone())
    );
}
