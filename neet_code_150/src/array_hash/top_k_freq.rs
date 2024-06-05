use std::cmp::Ordering;
use std::collections::HashMap;

// 出来なかった😭
/*
struct Solution {}
impl Solution {
    pub fn top_k_freqent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut set = HashSet::new();
        let mut map:HashMap<i32,i32> = HashMap::new();

        for v in &nums {
            if set.contains(v) {
            }

            set.insert(v);
        }

        vec![]
    }
}
*/

struct SolutionAns {}
impl SolutionAns {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut map: HashMap<i32, i32> = HashMap::new();

        for n in nums {
            *map.entry(n).or_default() += 1;
        }

        let mut freq: Vec<(i32, i32)> = map.into_iter().collect();

        let res = if k == freq.len() as i32 {
            &freq
        } else {
            Self::quick_select(&mut freq, k)
        };

        res.into_iter().map(|&(n, _)| n).collect()
    }

    pub fn quick_select(slice: &mut [(i32, i32)], k: i32) -> &[(i32, i32)] {
        let (mut pivot, mut i, mut j) = (0, 1, 1);

        for index in 1..slice.len() {
            if slice[index].1 >= slice[pivot].1 {
                slice.swap(index, j);
                j += 1;
            } else {
                slice.swap(index, i);
                i += 1;
                j += 1;
            }
        }

        slice.swap(pivot, i - 1);
        pivot = i - 1;

        let larger_items = (j - pivot) as i32;

        match larger_items.cmp(&k) {
            Ordering::Less => Self::quick_select(&mut slice[0..j], k),
            Ordering::Greater => Self::quick_select(&mut slice[pivot + 1..j], k),
            Ordering::Equal => &slice[pivot..j],
        }
    }
}

// C++ の模範解答より
struct SolutionAnsCpp {}
impl SolutionAnsCpp {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let n = nums.len();
        // 要素 -> その個数 の対応をmapに作る
        let mut map: HashMap<i32, i32> = HashMap::new();
        for v in nums {
            *map.entry(v).or_default() += 1;
        }

        // mapと逆の対応関係(個数 -> 要素)bucketを作る
        // bucketを作ると要素が少ない順に並ぶ(バケットソートに似てる)
        let mut bucket = vec![vec![]; n + 1];
        for m in map {
            bucket[m.1 as usize].push(m.0);
        }
        //println!("bucket: {:?}", bucket);

        let mut result = vec![];
        for i in (0..=n).rev() {
            if result.len() >= k as usize {
                break;
            }

            if !bucket[i].is_empty() {
                for v in &bucket[i] {
                    result.push(*v as i32);
                }
            }
        }

        result
    }

    // 時間を開けて解いたときに考えた別解(上とほぼ同じ)
    fn top_k_frequent_2(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut map: HashMap<i32, i32> = HashMap::new();
        for v in nums {
            *map.entry(v).or_default() += 1;
        }

        let mut map_2 = vec![];
        for (k, v) in map {
            map_2.push((v, k));
        }
        map_2.sort_by(|a, b| a.0.cmp(&b.0));

        let mut result = vec![];
        let mut k = k;
        while !map_2.is_empty() && k > 0 {
            result.push(map_2.pop().unwrap().1);
            k -= 1;
        }

        result
    }
}

fn main() {
    let case_1 = vec![1, 1, 1, 2, 2, 3];
    let k_case_1 = 2;
    let res_1 = SolutionAns::top_k_frequent(case_1.clone(), k_case_1);
    println!("case_1: {:?}", res_1);

    let case_2 = vec![1];
    let k_case_2 = 1;
    let res_2 = SolutionAns::top_k_frequent(case_2.clone(), k_case_2);
    println!("case_1: {:?}", res_2);

    let res_cpp_1 = SolutionAnsCpp::top_k_frequent(case_1.clone(), k_case_1);
    let res_cpp_2 = SolutionAnsCpp::top_k_frequent(case_2.clone(), k_case_2);
    println!("case_1: {:?}", res_cpp_1);
    println!("case_2: {:?}", res_cpp_2);

    println!(
        "case_1: {:?}",
        SolutionAnsCpp::top_k_frequent_2(case_1.clone(), k_case_1)
    );
    println!(
        "case_2: {:?}",
        SolutionAnsCpp::top_k_frequent_2(case_2.clone(), k_case_2)
    );
}
