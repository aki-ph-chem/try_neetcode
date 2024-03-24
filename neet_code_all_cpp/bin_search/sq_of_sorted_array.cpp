#include <iostream>
#include <cmath>
#include <vector>

// C++の模範解答はつまらないやつだったので..
// Rustの模範解答
// 二分探索というよりもしゃくとり法では..
class SolutionAnsRust {
    public:
        std::vector<int> sortedSquares(std::vector<int>& nums) {
            std::vector<int> result(nums.size());
            int idx = nums.size() - 1;
            int left = 0;
            int right = nums.size() - 1;

            while(left <= right) {
                if(std::abs(nums[left]) > std::abs(nums[right])) {
                    result[idx] = std::pow(nums[left], 2);
                    ++left;
                } else {
                    result[idx] = std::pow(nums[right], 2);
                    --right;
                }
                --idx;
            }

            return result;
        }
};

void show_result(std::vector<int>& result) {
    for(auto& v: result) {
        std::cout << v << " ";
    }
    std::cout << std::endl;
}

int main(void) {
    std::vector<int> case_1 = {-4, -1, 0, 3, 10};
    // => [0,1,9,16,100}
    std::vector<int> case_2 = {-7, -3, 2, 3, 11};
    // =>  [4,9,9,49,121}
    std::vector<int> case_3 = {-5, -3, 2, 10};
    // => [0, 1, 9, 100]
    
    SolutionAnsRust s_ans_rs;

    auto res_1 = s_ans_rs.sortedSquares(case_1);
    show_result(res_1);
    auto res_2 = s_ans_rs.sortedSquares(case_2);
    show_result(res_2);
    auto res_3 = s_ans_rs.sortedSquares(case_3);
    show_result(res_3);
}
