// 解けなかった
struct Solution {}
impl Solution {
    pub fn find_closest_element(arr: Vec<i32>, k: i32, x: i32) -> Vec<i32> {
        let (mut left, mut mid, mut right) = (0, 0, arr.len() as i32 - 1);
        while left <= right {
            mid = left + (right - left) / 2;
            if arr[mid as usize] == x {
                break;
            }

            if arr[mid as usize] < x {
                left = mid + 1;
            } else {
                right = mid - 1;
            }
        }

        println!("mid: {}", mid);
        let mut result = vec![];
        let mut k = k;
        result.push(arr[mid as usize]);
        k -= 1;
        println!("k : {}", k);

        (left, right) = (mid - 1, mid + 1);
        while 0 <= left && right < arr.len() as i32 && k > 0 {
            if (arr[left as usize] - x).abs() == (arr[right as usize] - x).abs() {
                if arr[left as usize] < arr[right as usize] {
                    result.push(arr[left as usize]);
                    left -= 1;
                } else {
                    result.push(arr[right as usize]);
                    right += 1;
                }
            } else if (arr[left as usize] - x).abs() < (arr[right as usize] - x).abs() {
                result.push(arr[left as usize]);
                left -= 1;
            } else {
                result.push(arr[right as usize]);
                right += 1;
            }
            println!("k : {}", k);

            k -= 1;
        }

        result.sort();
        result
    }
}

// AC
// Pythonの模範解答より
struct SolutionAnsPython {}
impl SolutionAnsPython {
    pub fn find_closest_element(arr: Vec<i32>, k: i32, x: i32) -> Vec<i32> {
        let (mut left, mut right) = (0, arr.len() as i32 - 1);
        let (mut val, mut idx) = (arr[0], 0);
        while left <= right {
            let mid = left + (right - left) / 2;
            let current_diff = (arr[mid as usize] - x).abs();
            let res_diff = (val - x).abs();

            if current_diff < res_diff || (current_diff == res_diff && arr[mid as usize] < val) {
                (val, idx) = (arr[mid as usize], mid);
            }

            if arr[mid as usize] < x {
                left = mid + 1;
            } else if arr[mid as usize] > x {
                right = mid - 1;
            } else {
                break;
            }
        }

        (left, right) = (idx, idx);
        for _i in 0..k - 1 {
            if left == 0 {
                right += 1;
            } else if right == arr.len() as i32 - 1
                || x - arr[left as usize - 1] <= arr[right as usize + 1] - x
            {
                left -= 1;
            } else {
                right += 1;
            }
        }

        arr[left as usize..right as usize + 1].to_vec()
    }
}

fn main() {
    let case_1 = (vec![1, 2, 3, 4, 5], 4, 3);
    // => [1, 2, 3, 4]
    let case_2 = (vec![1, 2, 3, 4, 5], 4, -1);
    // => [1, 2, 3, 4]
    let case_3 = (vec![1, 1, 1, 10, 10, 10], 1, 9);
    // => [10]

    /*
    println!(
        "case_1: {:?}",
        Solution::find_closest_element(case_1.0.clone(), case_1.1, case_1.2)
    );
    println!(
        "case_2: {:?}",
        Solution::find_closest_element(case_2.0.clone(), case_2.1, case_2.2)
    );
    println!(
        "case_3: {:?}",
        Solution::find_closest_element(case_3.0.clone(), case_3.1, case_3.2)
    );
    */

    println!(
        "case_1: {:?}",
        SolutionAnsPython::find_closest_element(case_1.0.clone(), case_1.1, case_1.2)
    );
    println!(
        "case_2: {:?}",
        SolutionAnsPython::find_closest_element(case_2.0.clone(), case_2.1, case_2.2)
    );
    println!(
        "case_3: {:?}",
        SolutionAnsPython::find_closest_element(case_3.0.clone(), case_3.1, case_3.2)
    );
}
