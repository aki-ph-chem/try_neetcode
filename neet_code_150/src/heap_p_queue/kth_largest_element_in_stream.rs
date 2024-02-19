use std::cmp::Reverse;
use std::collections::BinaryHeap;

// 解けなかった
struct KthLargest {
    k: i32,
    heap: BinaryHeap<i32>,
}

impl KthLargest {
    fn new(k: i32, nums: Vec<i32>) -> Self {
        Self {
            k,
            heap: BinaryHeap::from(nums),
        }
    }

    fn add(&mut self, val: i32) -> i32 {
        self.heap.push(val);

        let mut heap_clone = self.heap.clone();
        for _step in 0..self.k - 1 {
            heap_clone.pop();
        }

        *heap_clone.peek().unwrap()
    }
}

// 模範解答
struct SolutionAns {
    min_heap: BinaryHeap<Reverse<i32>>,
    size: usize,
}

impl SolutionAns {
    fn new(k: i32, nums: Vec<i32>) -> Self {
        let mut kth_largest = Self {
            min_heap: BinaryHeap::new(),
            size: k as usize,
        };

        for n in nums {
            kth_largest.add(n);
        }

        kth_largest
    }

    fn add(&mut self, val: i32) -> i32 {
        self.min_heap.push(Reverse(val));

        if self.min_heap.len() > self.size {
            self.min_heap.pop();
        }

        match self.min_heap.peek() {
            Some(Reverse(min_value)) => *min_value,
            _ => -1,
        }
    }
}

// AC
// C++の模範解答より
struct SolutionAnsCpp {
    k: i32,
    min_heap: BinaryHeap<Reverse<i32>>,
}
impl SolutionAnsCpp {
    fn new(k: i32, nums: Vec<i32>) -> Self {
        let mut k_th_largest = Self {
            k,
            min_heap: BinaryHeap::new(),
        };

        for n in nums {
            k_th_largest.min_heap.push(Reverse(n));
        }

        while k_th_largest.min_heap.len() as i32 > k {
            k_th_largest.min_heap.pop();
        }

        k_th_largest
    }

    fn add(&mut self, val: i32) -> i32 {
        self.min_heap.push(Reverse(val));

        while self.min_heap.len() as i32 > self.k {
            self.min_heap.pop();
        }

        self.min_heap.peek().unwrap().0
    }
}

fn main() {
    let case_1 = (3, vec![4, 5, 8, 2]);
    let case_1_query = vec![3, 5, 10, 9, 4];
    // => 4,5,5,8,8

    /*
    let mut kth_larget_1 = KthLargest::new(case_1.0, case_1.1.clone());
    for v in &case_1_query {
        println!("kth_largest_1.add({}): {}", *v, kth_larget_1.add(*v));
    }
    */

    let mut kth_larget_ans_1 = SolutionAns::new(case_1.0, case_1.1.clone());
    for v in &case_1_query {
        println!(
            "kth_largest_ans_1.add({}): {}",
            *v,
            kth_larget_ans_1.add(*v)
        );
    }

    let mut kth_larget_ans_cpp_1 = SolutionAnsCpp::new(case_1.0, case_1.1.clone());
    for v in &case_1_query {
        println!(
            "kth_largest_ans_cpp_1.add({}): {}",
            *v,
            kth_larget_ans_cpp_1.add(*v)
        );
    }
}
