#include <iostream>
#include <vector>

// AC
// ビット演算で部分集合を数え上げる
class Solution {
    public:
        std::vector<std::vector<int>> subsets(std::vector<int>& nums) {
            auto n = (int)nums.size();
            std::vector<std::vector<int>> result;

            for(int i = 0; i < (1 << n); ++i) {
                std::vector<int> sub_set;
                for(int j = 0; j < n; ++j) {
                    if(i & (1 << j)) {
                        sub_set.push_back(nums[j]);
                    }
                }
                result.push_back(sub_set);
            }

            return result;
        }
};

// 模範解答
// DFS
class SolutionAns {
    public:
        std::vector<std::vector<int>> subsets(std::vector<int>& nums) {
            std::vector<int> current;
            std::vector<std::vector<int>> result;
            dfs(nums, 0, current, result);

            return result;
        }

    private:
        void dfs(
                std::vector<int>& nums, 
                int start, 
                std::vector<int>& current, 
                std::vector<std::vector<int>>& result
                ) {
            result.push_back(current);

            for(int i = start; i < nums.size(); ++i) {
                current.push_back(nums[i]);
                dfs(nums, i + 1, current, result);
                current.pop_back();
            }
        }
};

void show_result(const std::vector<std::vector<int>>& result) {
    for(auto& v: result) {
        for(auto& w: v) {
            std::cout << w << " ";
        }
        std::cout << '\n';
    }
}

int main(void) {
    auto case_1 = std::vector{1, 2, 3};
    auto case_2 = std::vector{0};

    Solution s_1;

    auto res_1 = s_1.subsets(case_1);
    auto res_2 = s_1.subsets(case_2);
    show_result(res_1);
    show_result(res_2);

    SolutionAns s_ans;
    auto res_1_ans = s_ans.subsets(case_1);
    auto res_2_ans = s_ans.subsets(case_2);
    show_result(res_1_ans);
    show_result(res_2_ans);
}
