use std::cmp::Reverse;
use std::collections::BinaryHeap;

// 解けなかった
struct MedianFinder {
    heap: BinaryHeap<i32>,
}

impl MedianFinder {
    fn new() -> Self {
        Self {
            heap: BinaryHeap::new(),
        }
    }

    fn add_num(&mut self, num: i32) {
        self.heap.push(num);
    }

    fn find_median(&self) -> f64 {
        1.2
    }
}

// AC
// C++の模範解答より
struct SolutionAnsCpp {
    lower: BinaryHeap<i32>,
    higher: BinaryHeap<Reverse<i32>>,
}

impl SolutionAnsCpp {
    fn new() -> Self {
        Self {
            lower: BinaryHeap::new(),
            higher: BinaryHeap::new(),
        }
    }

    fn add_num(&mut self, num: i32) {
        if self.lower.is_empty() {
            self.lower.push(num);
            return;
        }

        if self.lower.len() > self.higher.len() {
            if self.lower.peek().unwrap() > &num {
                self.higher.push(Reverse(*self.lower.peek().unwrap()));
                self.lower.pop();
                self.lower.push(num);
            } else {
                self.higher.push(Reverse(num));
            }
        } else {
            if num > self.higher.peek().unwrap().0 {
                self.lower.push(self.higher.peek().unwrap().0);
                self.higher.pop();
                self.higher.push(Reverse(num));
            } else {
                self.lower.push(num);
            }
        }
    }

    fn find_median(&self) -> f64 {
        let mut result = 0.0;

        if self.lower.len() == self.higher.len() {
            result = *self.lower.peek().unwrap() as f64
                + (self.higher.peek().unwrap().0 - *self.lower.peek().unwrap()) as f64 / 2_f64;
        } else {
            if self.lower.len() > self.higher.len() {
                result = *self.lower.peek().unwrap() as f64;
            } else {
                result = self.higher.peek().unwrap().0 as f64;
            }
        }

        result
    }
}

fn main() {
    let mut median_ans = SolutionAnsCpp::new();
    median_ans.add_num(1);
    median_ans.add_num(2);
    println!("{}", median_ans.find_median());
    median_ans.add_num(3);
    println!("{}", median_ans.find_median());
}
