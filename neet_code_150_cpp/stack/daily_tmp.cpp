#include <iostream>
#include <vector>
#include <stack>

class Solution {
    public:
        std::vector<int> daily_tmp(const std::vector<int>& temperatures) {
            auto len_tmp = temperatures.size();
            std::vector<int> result(len_tmp,0);
            std::stack<std::pair<int,std::size_t>> stack;

            for(std::size_t i = 0; i < len_tmp; ++i) {
                while(!stack.empty() && temperatures[i] > stack.top().first) {
                    auto stack_idx = stack.top().second;
                    stack.pop();
                    result[stack_idx] = static_cast<int>(i - stack_idx);
                }

                stack.push({temperatures[i], i});
            }

            return result;
        }
};

int main(void) {
    const auto case_1 = std::vector{73, 74, 75, 71, 69, 72, 76, 73};
    const auto case_2 = std::vector{30, 40, 50, 60};
    const auto case_3 = std::vector{30, 60, 90};

    Solution s_1;

    auto res_1 = s_1.daily_tmp(case_1);
    for(const auto& v: res_1) {
        std::cout << v << " ";
    }
    std::cout << std::endl;

    auto res_2 = s_1.daily_tmp(case_2);
    for(const auto& v: res_2) {
        std::cout << v << " ";
    }
    std::cout << std::endl;
    auto res_3 = s_1.daily_tmp(case_3);
    for(const auto& v: res_3) {
        std::cout << v << " ";
    }
    std::cout << std::endl;
}
