// 縛り1: 標準ライブラリのsort関数は禁止
// 縛り2: O(NlogN)で実装せよ
// 縛り3: できる限り少ないメモリで

struct Solution {}
impl Solution {
    // マージソート
    // AC
    pub fn sort_array(nums: Vec<i32>) -> Vec<i32> {
        let mut result = nums;
        let n = result.len();
        Self::m_sort(&mut result, 0, n);

        result
    }

    // クイックソート
    // AC
    pub fn sort_array_2(nums: Vec<i32>) -> Vec<i32> {
        let mut result = nums;
        let n = result.len();
        Self::q_sort(&mut result, 0, n);

        result
    }

    // 挿入ソート
    // TLE
    pub fn sort_array_3(nums: Vec<i32>) -> Vec<i32> {
        let mut result = nums;
        let n = result.len();
        Self::i_sort(&mut result);

        result
    }

    fn m_sort(nums: &mut Vec<i32>, left: usize, right: usize) {
        if right - left == 1 {
            return;
        }

        let mid = left + (right - left) / 2;
        // 左側
        Self::m_sort(nums, left, mid);
        // 右側
        Self::m_sort(nums, mid, right);

        // バッファにコピー
        let mut buff = vec![];
        for i in left..mid {
            buff.push(nums[i])
        }
        for i in (mid..=right - 1).rev() {
            buff.push(nums[i]);
        }

        // マージ
        let (mut idx_left, mut idx_right) = (0, buff.len() - 1);
        for i in left..right {
            if buff[idx_left] <= buff[idx_right] {
                nums[i] = buff[idx_left];
                idx_left += 1;
            } else {
                nums[i] = buff[idx_right];
                idx_right -= 1;
            }
        }
    }

    // クイックソート:O(N^2)
    // そしてinplace
    fn q_sort(nums: &mut Vec<i32>, left: usize, right: usize) {
        if right - left <= 1 {
            return;
        }

        // pivot(中点)
        let pivot_idx = left + (right - left) / 2;
        let pivot_value = nums[pivot_idx];

        // pivotと右端をswap
        nums.swap(pivot_idx, right - 1);
        let mut i = left;
        // nums[j] >= pivot_value => j += 1
        // nums[j] < pivot_value => nums[i], nums[j] をswapして i += 1
        for j in left..right - 1 {
            if nums[j] < pivot_value {
                nums.swap(i, j);
                i += 1;
            }
        }

        // 最後にa[i]とpivotを交換
        nums.swap(i, right - 1);

        // 左側、右側をsort
        Self::q_sort(nums, left, i);
        Self::q_sort(nums, i + 1, right);
    }

    // 挿入ソート: O(N^2)
    // 一番シンプル
    fn i_sort(nums: &mut Vec<i32>) {
        let n = nums.len() as i32;

        for i in 1..n {
            let v = nums[i as usize];
            // j: vを挿入する位置
            let mut j = i;
            while j > 0 {
                if nums[j as usize - 1] > v {
                    nums[j as usize] = nums[j as usize - 1];
                } else {
                    break;
                }

                j -= 1;
            }
            nums[j as usize] = v;
        }
    }
}

fn main() {
    let case_1 = vec![5, 2, 3, 1];
    // => [1,2,3,5]
    let case_2 = vec![5, 1, 1, 2, 0, 0];
    // => [0,0,1,1,2,5]

    println!("case_1: {:?}", Solution::sort_array(case_1.clone()));
    println!("case_2: {:?}", Solution::sort_array(case_2.clone()));

    println!("case_1: {:?}", Solution::sort_array_2(case_1.clone()));
    println!("case_2: {:?}", Solution::sort_array_2(case_2.clone()));

    println!("case_1: {:?}", Solution::sort_array_3(case_1.clone()));
    println!("case_2: {:?}", Solution::sort_array_3(case_2.clone()));
}
