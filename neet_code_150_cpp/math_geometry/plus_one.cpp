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
};

int main(void) {
    auto case_1 = std::vector{1, 2, 3}; // [1,2,4]
    auto case_2 = std::vector{4, 3, 2, 1}; // [4,3,2,1}
    auto case_3 = std::vector{9}; // [1,0}
    auto case_4 = std::vector{9, 8, 7, 6, 5, 4, 3, 2, 1, 0}; // [9,8,7,6,5,4,3,2,1,1}
    auto case_5 = std::vector{4, 3, 2, 1, 3, 0};
    auto case_6 = std::vector{9, 9};

   Solution s_1;

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
}
