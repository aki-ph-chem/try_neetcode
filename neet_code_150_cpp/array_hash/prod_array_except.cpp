#include <cstddef>
#include <iostream>
#include <vector>
//#define DEBUG_SOLUTION

class Solution {
    public:
        // O(N^2)
        std::vector<int> product_except_self_sq(const std::vector<int>& nums) {
            auto nums_size = nums.size();
            std::vector<int> result;
            for(std::size_t i = 0; i < nums_size; ++i) {
                int tmp = 1;
                for(std::size_t j = 0; j < nums_size; ++j) {
                    if(i == j) {
                        continue;
                    }
                    tmp *= nums[j];
                }
                result.push_back(tmp);
            }
            return result;
        }

        std::vector<int> product_except_self(const std::vector<int>& nums) {
            auto nums_size = nums.size();
            std::vector<int> result(nums_size, 1);

            // i-1までの積
            for(std::size_t i = 1; i < nums_size; ++i) {
                result[i] = result[i-1] * nums[i - 1];
            }
#ifdef DEBUG_SOLUTION
            std::cout << "result" << std::endl;
            for(const auto n: result) {
                std::cout << n << std::endl;
            }
#endif
            int right = 1;
            for(int i = nums_size - 1; i >= 0; --i) {
#ifdef DEBUG_SOLUTION
                std::cout << "i: " << i << std::endl;
#endif
                result[i] = right * result[i];
                right *= nums[i]; 
            }
            return result;
        }
};

// 後で時間を置いて解いたときの別解
class SolutionLatter {
    public:
        // AC
        std::vector<int> prod_except_self(std::vector<int>& nums) {
            auto n = nums.size();
            std::vector<int> prefix(n + 1, 1);
            for(int i = 0; i < n; ++i) {
                prefix[i + 1] = nums[i] * prefix[i];
            }

            std::vector<int> result(n, 1);
            int acc = 1;
            for(int i = n - 1; i >= 0; --i) {
                result[i] = acc * prefix[i];
                acc *= nums[i];
            }

            return result;
        }
};

int main(void) {
    auto case_1 = std::vector{1,2,3,4};
    auto case_2 = std::vector{-1,1,0,-3,3};

    Solution s_1;
    // O(N^2)
    for(const auto &v: s_1.product_except_self_sq(case_1)) {
        std::cout << v << " ";
    }
    std::cout << std::endl;
    for(const auto &v: s_1.product_except_self_sq(case_2)) {
        std::cout << v << " ";
    }
    std::cout << std::endl;

    // O(kN)
    for(const auto &v: s_1.product_except_self(case_1)) {
        std::cout << v << " ";
    }
    std::cout << std::endl;
    for(const auto &v: s_1.product_except_self(case_2)) {
        std::cout << v << " ";
    }
    std::cout << std::endl;

    SolutionLatter s_ans_latter;
    auto res_1_latter = s_ans_latter.prod_except_self(case_1);
    auto res_2_latter = s_ans_latter.prod_except_self(case_2);

    for(auto &v: res_1_latter) {
        std::cout << v << " ";
    }
    std::cout << std::endl;
    for(auto &v: res_2_latter) {
        std::cout << v << " ";
    }
    std::cout << std::endl;
}
