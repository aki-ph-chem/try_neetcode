use std::collections::HashSet;

// RustだとHashSetでなくて単に条件分岐で母音か否かの判定をしたほうが高速だった(C++はあまり変わらなかった)
struct Solution {}
impl Solution {
    // TLE
    // O(kN)
    pub fn max_vowels(s: String, k: i32) -> i32 {
        let s: Vec<char> = s.chars().collect();
        let k = k as usize;

        let vowel_set = HashSet::from(['a', 'e', 'i', 'o', 'u']);
        let mut result = 0;

        for i in 0..s.len() {
            let mut tmp_num = 0;

            let mut j = i;
            while j < s.len() && j < i + k {
                if vowel_set.contains(&s[j]) {
                    tmp_num += 1;
                }
                j += 1;
            }

            result = result.max(tmp_num);
        }

        result
    }

    // AC
    // O(N)
    pub fn max_vowels_2(s: String, k: i32) -> i32 {
        let s: Vec<char> = s.chars().collect();
        let k = k as usize;

        let vowel_set = HashSet::from(['a', 'e', 'i', 'o', 'u']);
        let (mut left, mut total) = (0, 0);
        let mut result = 0;

        for i in 0..k {
            if vowel_set.contains(&s[i]) {
                total += 1;
            }
            //println!("total(0): {}", total);
            result = total;
        }

        for j in k..s.len() {
            if vowel_set.contains(&s[j]) {
                total += 1;
            }
            //println!("total(1): {}", total);

            if vowel_set.contains(&s[left]) {
                total -= 1;
            }
            //println!("total(2): {}", total);

            left += 1;
            result = result.max(total);
        }

        result
    }

    // O(N): HashSetを使わない実装
    // AC
    pub fn max_vowels_3(s: String, k: i32) -> i32 {
        let s: Vec<char> = s.chars().collect();
        let k = k as usize;

        let is_vowels = |&c| match c {
            'a' | 'e' | 'i' | 'o' | 'u' => true,
            _ => false,
        };

        let (mut left, mut total) = (0, 0);
        let mut result = 0;

        for i in 0..k {
            if is_vowels(&s[i]) {
                total += 1;
            }
            //println!("total(0): {}", total);
            result = total;
        }

        for j in k..s.len() {
            if is_vowels(&s[j]) {
                total += 1;
            }
            //println!("total(1): {}", total);

            if is_vowels(&s[left]) {
                total -= 1;
            }
            //println!("total(2): {}", total);

            left += 1;
            result = result.max(total);
        }

        result
    }
}

// AC
// C++の模範解答より
struct SolutionAnsCpp {}
impl SolutionAnsCpp {
    pub fn max_vowels(s: String, k: i32) -> i32 {
        let is_vowel = |c| match c {
            'a' | 'e' | 'i' | 'o' | 'u' => true,

            _ => false,
        };

        let k = k as usize;
        let s: Vec<char> = s.chars().collect();

        let (mut part_1, mut part_2) = (0, 0);
        let (mut count, mut max_i) = (0, std::usize::MIN);

        while part_2 < s.len() {
            if part_2 - part_1 == k {
                max_i = max_i.max(count);

                if max_i == k {
                    return count as i32;
                }

                if is_vowel(s[part_1]) {
                    count -= 1;
                }
                part_1 += 1;

                if is_vowel(s[part_2]) {
                    count += 1;
                }
                part_2 += 1;
            } else {
                if is_vowel(s[part_2]) {
                    count += 1;
                }

                part_2 += 1;
            }
        }

        max_i = max_i.max(count);
        max_i as i32
    }
}

fn main() {
    let case_1 = ("abciiidef".to_string(), 3);
    // => 3
    let case_2 = ("aeiou".to_string(), 2);
    // => 2
    let case_3 = ("leetcode".to_string(), 3);
    // => 2
    let case_4 = ("tryhard".to_string(), 4);
    // => 1

    println!(
        "case_1: {}",
        Solution::max_vowels(case_1.0.clone(), case_1.1)
    );
    println!(
        "case_2: {}",
        Solution::max_vowels(case_2.0.clone(), case_2.1)
    );
    println!(
        "case_3: {}",
        Solution::max_vowels(case_3.0.clone(), case_3.1)
    );
    println!(
        "case_4: {}",
        Solution::max_vowels(case_4.0.clone(), case_4.1)
    );

    println!(
        "case_1: {}",
        Solution::max_vowels_2(case_1.0.clone(), case_1.1)
    );
    println!(
        "case_2: {}",
        Solution::max_vowels_2(case_2.0.clone(), case_2.1)
    );
    println!(
        "case_3: {}",
        Solution::max_vowels_2(case_3.0.clone(), case_3.1)
    );
    println!(
        "case_4: {}",
        Solution::max_vowels_2(case_4.0.clone(), case_4.1)
    );

    println!(
        "case_1: {}",
        Solution::max_vowels_3(case_1.0.clone(), case_1.1)
    );
    println!(
        "case_2: {}",
        Solution::max_vowels_3(case_2.0.clone(), case_2.1)
    );
    println!(
        "case_3: {}",
        Solution::max_vowels_3(case_3.0.clone(), case_3.1)
    );
    println!(
        "case_4: {}",
        Solution::max_vowels_3(case_4.0.clone(), case_4.1)
    );

    println!(
        "case_1: {}",
        SolutionAnsCpp::max_vowels(case_1.0.clone(), case_1.1)
    );
    println!(
        "case_2: {}",
        SolutionAnsCpp::max_vowels(case_2.0.clone(), case_2.1)
    );
    println!(
        "case_3: {}",
        SolutionAnsCpp::max_vowels(case_3.0.clone(), case_3.1)
    );
    println!(
        "case_4: {}",
        SolutionAnsCpp::max_vowels(case_4.0.clone(), case_4.1)
    );
}
