use std::collections::HashMap;

// 間に合ってしまったのだが...
struct StockSpanner {
    stack: Vec<i32>,
}
impl StockSpanner {
    pub fn new() -> Self {
        Self { stack: vec![] }
    }

    // O(N)
    pub fn next(&mut self, price: i32) -> i32 {
        self.stack.push(price);
        let mut result = 0;
        for p in self.stack.iter().rev() {
            if *p > price {
                break;
            }
            result += 1;
        }

        result
    }
}

// nexをO(N)より軽くできないかな?
//  => できなかった...
struct StockSpanner2 {
    stack: Vec<(i32, i32)>,
}
impl StockSpanner2 {
    pub fn new() -> Self {
        Self { stack: vec![] }
    }

    pub fn next(&mut self, price: i32) -> i32 {
        if self.stack.is_empty() {
            self.stack.push((price, 1));
            return 1;
        }

        let mut result = 0;
        if let Some(stack_last) = self.stack.last() {
            if stack_last.0 == price {
                result += stack_last.1;
            } else if stack_last.0 < price {
                result += stack_last.1;

                let idx = self.stack.len() as i32 - (stack_last.1 - 0);
                println!("self.stack.len(), idx: {},{}", self.stack.len(), idx);

                if self.stack[idx as usize].0 <= price {
                    result += self.stack[idx as usize].1;
                }
            }
        }

        result += 1;
        self.stack.push((price, result));

        result
    }
}

// 模範解答
struct SolutoinAns {
    stack: Vec<(i32, i32)>,
}
impl SolutoinAns {
    pub fn new() -> Self {
        Self { stack: vec![] }
    }

    pub fn next(&mut self, price: i32) -> i32 {
        let mut span = 1;

        while !self.stack.is_empty() && self.stack.last().unwrap().0 <= price {
            span += self.stack.pop().unwrap().1;
        }
        self.stack.push((price, span));

        span
    }
}

// C++の模範解答より
// AC
struct SolutoinAnsCpp {
    stack: Vec<(i32, i32)>,
    current_price: i32,
    current_span: i32,
}
impl SolutoinAnsCpp {
    pub fn new() -> Self {
        Self {
            stack: vec![],
            current_price: 0,
            current_span: 0,
        }
    }

    pub fn next(&mut self, price: i32) -> i32 {
        let mut result = 1;

        while !self.stack.is_empty() && price >= self.current_price {
            result += self.current_span;
            self.stack.pop();
            if !self.stack.is_empty() {
                self.current_price = self.stack.last().unwrap().0;
                self.current_span = self.stack.last().unwrap().1;
            }
        }

        self.stack.push((price, result));
        self.current_price = self.stack.last().unwrap().0;
        self.current_span = self.stack.last().unwrap().1;

        result
    }
}

fn main() {
    let mut stock_sp = StockSpanner::new();
    println!("100: {}", stock_sp.next(100));
    // 1
    println!("80: {}", stock_sp.next(80));
    // 1
    println!("60: {}", stock_sp.next(60));
    // 1
    println!("70: {}", stock_sp.next(70));
    // 2
    println!("60: {}", stock_sp.next(60));
    // 1
    println!("75: {}", stock_sp.next(75));
    // 4
    println!("85: {}", stock_sp.next(85));
    // 6

    /*
    let mut stock_sp_2 = StockSpanner2::new();
    println!("100: {}", stock_sp_2.next(100));
    // 1
    println!("80: {}", stock_sp_2.next(80));
    // 1
    println!("60: {}", stock_sp_2.next(60));
    // 1
    println!("70: {}", stock_sp_2.next(70));
    // 2
    println!("60: {}", stock_sp_2.next(60));
    // 1
    println!("75: {}", stock_sp_2.next(75));
    // 4
    println!("85: {}", stock_sp_2.next(85));
    // 6

    let mut stock_sp_3 = StockSpanner2::new();
    println!("31: {}", stock_sp_3.next(31));
    // 1
    println!("41: {}", stock_sp_3.next(41));
    // 2
    println!("48: {}", stock_sp_3.next(48));
    // 3
    println!("59: {}", stock_sp_3.next(59));
    // 4
    println!("79: {}", stock_sp_3.next(79));
    // 5
    println!("{:?}", stock_sp_3.stack);
    */

    let mut stock_sp_ans = SolutoinAns::new();
    println!("100: {}", stock_sp_ans.next(100));
    // 1
    println!("80: {}", stock_sp_ans.next(80));
    // 1
    println!("60: {}", stock_sp_ans.next(60));
    // 1
    println!("70: {}", stock_sp_ans.next(70));
    // 2
    println!("60: {}", stock_sp_ans.next(60));
    // 1
    println!("75: {}", stock_sp_ans.next(75));
    // 4
    println!("85: {}", stock_sp_ans.next(85));
    // 6

    let mut stock_sp_ans_cpp = SolutoinAnsCpp::new();
    println!("100: {}", stock_sp_ans_cpp.next(100));
    // 1
    println!("80: {}", stock_sp_ans_cpp.next(80));
    // 1
    println!("60: {}", stock_sp_ans_cpp.next(60));
    // 1
    println!("70: {}", stock_sp_ans_cpp.next(70));
    // 2
    println!("60: {}", stock_sp_ans_cpp.next(60));
    // 1
    println!("75: {}", stock_sp_ans_cpp.next(75));
    // 4
    println!("85: {}", stock_sp_ans_cpp.next(85));
    // 6
}
