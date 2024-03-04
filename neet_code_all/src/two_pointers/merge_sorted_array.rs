struct Solution {}
impl Solution {
    // AC
    // Merge sortの最後のマージを実装した
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        let mut buff = nums1.clone();
        for (i, num) in (m as usize..buff.len()).zip(nums2.iter().rev()) {
            buff[i] = *num;
        }
        //println!("buff: {:?}", buff);

        let (mut l, mut r) = (0, nums1.len() - 1);
        for i in 0..nums1.len() {
            if buff[l] <= buff[r] {
                nums1[i] = buff[l];
                l += 1;
            } else {
                nums1[i] = buff[r];
                r -= 1;
            }
        }

        //println!("num1: {:?}", num1);
    }
}

// 模範解答
struct SolutionAns {}
impl SolutionAns {
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        let (mut m, mut n) = (m as usize, n as usize);
        // Last index nums1
        let mut last = m + n - 1;

        // Merge in reverse order
        // num1, num2の後ろの要素から比較して大きい方をnum1の後ろに入れる
        while m > 0 && n > 0 {
            if nums1[m - 1] > nums2[n as usize - 1] {
                nums1[last] = nums1[m - 1];
                m -= 1;
            } else {
                nums1[last] = nums2[n - 1];
                n -= 1;
            }
            last -= 1;
        }

        // num2の残りもnum1にマージ
        while n > 0 {
            nums1[last] = nums2[n - 1];
            n -= 1;
            last -= 1;
        }
    }
}

fn main() {
    let case_1 = (vec![1, 2, 3, 0, 0, 0], 3, vec![2, 5, 6], 3);
    // [1,2,2,3,5,6]
    let case_2 = (vec![1], 1, Vec::<i32>::new(), 0);
    // [1]
    let case_3 = (vec![0], 0, vec![1], 1);
    // [1]
    let case_4 = (vec![1, 5, 10, 0, 0, 0], 3, vec![2, 7, 12], 3);

    let mut res_1 = case_1.clone();
    Solution::merge(&mut res_1.0, res_1.1, &mut res_1.2, res_1.3);
    println!("case_1: {:?}", res_1.0);

    let mut res_2 = case_2.clone();
    Solution::merge(&mut res_2.0, res_2.1, &mut res_2.2, res_2.3);
    println!("case_2: {:?}", res_2.0);

    let mut res_4 = case_4.clone();
    Solution::merge(&mut res_4.0, res_4.1, &mut res_4.2, res_4.3);
    println!("case_4: {:?}", res_4.0);

    let mut res_1 = case_1.clone();
    SolutionAns::merge(&mut res_1.0, res_1.1, &mut res_1.2, res_1.3);
    println!("case_1: {:?}", res_1.0);

    let mut res_2 = case_2.clone();
    SolutionAns::merge(&mut res_2.0, res_2.1, &mut res_2.2, res_2.3);
    println!("case_2: {:?}", res_2.0);

    let mut res_4 = case_4.clone();
    SolutionAns::merge(&mut res_4.0, res_4.1, &mut res_4.2, res_4.3);
    println!("case_4: {:?}", res_4.0);
}
