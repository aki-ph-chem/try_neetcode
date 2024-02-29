#include <iostream>
#include <vector>

//AC
class Solution {
    public:
        std::vector<int> getConcatenation(std::vector<int>& nums) {
            auto n = nums.size();
            std::vector<int> result(2*n);

            for(int i = 0; i < n; ++i) {
                result[i] = nums[i];
                result[i + n] = nums[i];
            }

            return result;
        }
};

// 模範解答
class SolutionAns {
    public:
        std::vector<int> getConcatenation(std::vector<int>& nums) {
            std::vector<int> ans;
            auto len = nums.size();
            for(int i = 0; i < 2 *len; ++i) {
                ans.push_back(nums[i % len]);
            }

            return ans;
        }
};

void show_result(std::vector<int>& result) {
    for(auto& v: result) {
        std::cout << v << " ";
    }
    std::cout << std::endl;
}

int main(void) {
    auto case_1 = std::vector{1, 2, 1};
    auto case_2 = std::vector{1, 3, 2, 1};

    Solution s_1;
    auto res_1 = s_1.getConcatenation(case_1);
    auto res_2 = s_1.getConcatenation(case_2);
    show_result(res_1);
    show_result(res_2);
}
