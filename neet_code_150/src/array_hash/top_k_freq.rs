use std::collections::HashMap;
use std::cmp::Ordering;

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

        for n in nums{
            *map.entry(n).or_default() +=1;
        }

        let mut freq: Vec<(i32, i32)> = map.into_iter().collect();

        let res = if k == freq.len() as i32{
            &freq
        }else{
            quick_select(&mut freq, k)
        };

        res.into_iter()
            .map(|&(n, _)| n)
            .collect()
    }
}

pub fn quick_select(slice: &mut [(i32, i32)], k: i32) -> &[(i32, i32)]{
    let (mut pivot, mut i, mut j) = (0, 1, 1);

    for index in 1..slice.len(){
        if slice[index].1 >= slice[pivot].1{
            slice.swap(index, j);
            j+=1;
        }else{
            slice.swap(index, i);
            i+=1;
            j+=1;
        }
    }

    slice.swap(pivot, i - 1);
    pivot = i - 1;

    let larger_items = (j - pivot) as i32;

    match larger_items.cmp(&k) {
        Ordering::Less => quick_select(&mut slice[0..j], k),
        Ordering::Greater => quick_select(&mut slice[pivot + 1..j], k),
        Ordering::Equal => &slice[pivot..j],
    }
}

fn main() {
    let case_1 = vec![1,1,1,2,2,3];
    let k_case_1 = 2; 
    let res_1 = SolutionAns::top_k_frequent(case_1, k_case_1);
    println!("case_1: {:?}", res_1);

    let case_2 = vec![1];
    let k_case_2 = 1; 
    let res_2 = SolutionAns::top_k_frequent(case_2, k_case_2);
    println!("case_1: {:?}", res_2);
}
