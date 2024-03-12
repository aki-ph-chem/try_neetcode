// 縛り: 組み込みのsortを使うな, inplaceで実装せよ
struct Solution {}
impl Solution {
    // AC
    pub fn sort_colors(nums: &mut Vec<i32>) {
        let n = nums.len();
        Self::q_sort(nums, 0, n);
    }

    // クイックソートはinplaceなのでそれを実装する
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
}

// 模範解答
struct SolutionAns {}
impl SolutionAns {
    // ラディックスソート
    pub fn sort_colors(nums: &mut Vec<i32>) {
        let mut count = [0; 3];
        for num in nums.iter() {
            count[*num as usize] += 1;
        }

        let mut i = 0;
        for (num, c) in count.iter().enumerate() {
            for _ in 0..*c {
                nums[i] = num as i32;
                i += 1;
            }
        }
    }
}

// AC
// C++の模範解答より
struct SolutionAnsCpp {}
impl SolutionAnsCpp {
    pub fn sort_colors(nums: &mut Vec<i32>) {
        let (mut p_1, mut p_2) = (0_i32, nums.len() as i32 - 1);
        let mut i = p_1;

        while i <= p_2 {
            // 0だったら左端に詰める
            if nums[i as usize] == 0 {
                nums.swap(i as usize, p_1 as usize);
                p_1 += 1;
            }

            // 2だったら右端に詰める
            if nums[i as usize] == 2 {
                nums.swap(i as usize, p_2 as usize);
                p_2 -= 1;
                i -= 1;
            }

            i += 1;
        }
    }
}

fn main() {
    let case_1 = vec![2, 0, 2, 1, 1, 0];
    // => [0,0,1,1,2,2]
    let case_2 = vec![2, 0, 1];
    // => [0,1,2]

    let mut res_1 = case_1.clone();
    Solution::sort_colors(&mut res_1);
    println!("res_1: {:?}", res_1);

    let mut res_2 = case_2.clone();
    Solution::sort_colors(&mut res_2);
    println!("res_2: {:?}", res_2);

    let mut res_1 = case_1.clone();
    SolutionAns::sort_colors(&mut res_1);
    println!("res_1: {:?}", res_1);

    let mut res_2 = case_2.clone();
    SolutionAns::sort_colors(&mut res_2);
    println!("res_2: {:?}", res_2);

    let mut res_1 = case_1.clone();
    SolutionAnsCpp::sort_colors(&mut res_1);
    println!("res_1: {:?}", res_1);

    let mut res_2 = case_2.clone();
    SolutionAnsCpp::sort_colors(&mut res_2);
    println!("res_2: {:?}", res_2);
}
