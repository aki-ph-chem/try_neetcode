#include <iostream>
#include <vector>
#include <algorithm>

class Solution {
    public:
        // AC
        // ただだし速度はいまいち
        std::vector<int> plus_one(const std::vector<int>& digits) {
            auto res = digits;
            std::reverse(res.begin(), res.end());
            res[0] += 1;

            for(std::size_t i = 0; i < res.size(); ++i) {
                if(res[i] == 10) {
                    res[i] = 0;
                    if(i + 1 < res.size()) {
                        res[i + 1] += 1;
                    } else {
                        res.push_back(1);
                    }
                } 
            }

            std::reverse(res.begin(), res.end());
            return res;
        }

        // reverse()を使わない実装
        // AC
        std::vector<int> plus_one_b(const std::vector<int>& digits) {
            auto res = digits;
            res.back() += 1;

            for(int i = res.size() - 1; i >= 0; --i) {
                if(res[i] == 10) {
                    res[i] = 0;
                    if(i - 1 >= 0) {
                        res[i - 1] += 1;
                    } else {
                        res.insert(res.begin(), 1);
                    }
                }
            }

            return res;
        }
};

// 模範解答
class SolutionAns {
    public:
        std::vector<int> plusOne(std::vector<int>& digits) {
        for (int i = digits.size() - 1; i >= 0; i--) {
            if (digits[i] < 9) {
                digits[i]++;
                return digits;
            }
            digits[i] = 0;
        }
        
        digits[0] = 1;
        digits.push_back(0);
        return digits;
    }
};

int main(void) {
    auto case_1 = std::vector{1, 2, 3}; // [1,2,4]
    auto case_2 = std::vector{4, 3, 2, 1}; // [4,3,2,1}
    auto case_3 = std::vector{9}; // [1,0}
    auto case_4 = std::vector{9, 8, 7, 6, 5, 4, 3, 2, 1, 0}; // [9,8,7,6,5,4,3,2,1,1}
    auto case_5 = std::vector{4, 3, 2, 1, 3, 0};
    auto case_6 = std::vector{9, 9};

   Solution s_1;

   // plus_one()
   auto res_1 = s_1.plus_one(case_1);
   for(const auto& x: res_1) {
       std::cout << x << " ";
   }
   std::cout << std::endl;

   auto res_2 = s_1.plus_one(case_2);
   for(const auto& x: res_2) {
       std::cout << x << " ";
   }
   std::cout << std::endl;

   auto res_3 = s_1.plus_one(case_3);
   for(const auto& x: res_3) {
       std::cout << x << " ";
   }
   std::cout << std::endl;

   auto res_4 = s_1.plus_one(case_4);
   for(const auto& x: res_4) {
       std::cout << x << " ";
   }
   std::cout << std::endl;

   auto res_5 = s_1.plus_one(case_5);
   for(const auto& x: res_5) {
       std::cout << x << " ";
   }
   std::cout << std::endl;

   auto res_6 = s_1.plus_one(case_6);
   for(const auto& x: res_6) {
       std::cout << x << " ";
   }
   std::cout << std::endl;

   // plus_one_b()
   auto res_1b = s_1.plus_one_b(case_1);
   for(const auto& x: res_1b) {
       std::cout << x << " ";
   }
   std::cout << std::endl;

   auto res_2b = s_1.plus_one_b(case_2);
   for(const auto& x: res_2b) {
       std::cout << x << " ";
   }
   std::cout << std::endl;

   auto res_6b = s_1.plus_one_b(case_6);
   for(const auto& x: res_6b) {
       std::cout << x << " ";
   }
   std::cout << std::endl;

   // plusOne()
   SolutionAns s_1_ans;
   auto res_1_ans = s_1_ans.plusOne(case_1);
   for(const auto& x: res_1_ans) {
       std::cout << x << " ";
   }
   std::cout << std::endl;

   auto res_2_ans = s_1_ans.plusOne(case_2);
   for(const auto& x: res_2_ans) {
       std::cout << x << " ";
   }
   std::cout << std::endl;

   auto res_6_ans = s_1_ans.plusOne(case_6);
   for(const auto& x: res_6_ans) {
       std::cout << x << " ";
   }
   std::cout << std::endl;

}
