#include <cwctype>
#include <iostream>
#include <vector>
#include <algorithm>
#include <set>

// AC
class Solution {
    public:
        std::vector<std::vector<int>> subsetsWithDup(std::vector<int>& nums) {
            auto n = (int)nums.size();
            std::vector<std::vector<int>> result;
            std::set<std::vector<int>> set;

            for(int i = 0; i < (1 << n); ++i) {
                std::vector<int> sub_set;
                for(int j = 0; j < n; ++j) {
                    if(i & (1 << j)) {
                        sub_set.push_back(nums[j]);
                    }
                }
                std::sort(sub_set.begin(), sub_set.end());
                if(set.find(sub_set) == set.end()) {
                    set.insert(sub_set);
                    result.push_back(sub_set);
                }
            }

            return result;
        }
};

// 模範解答
class SolutionAns {
    public:
        std::vector<std::vector<int>> subsetsWithDup(std::vector<int>& nums) {
            std::sort(nums.begin(), nums.end());
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
                if(i > start && nums[i] == nums[i - 1]) {
                    continue;
                }

                current.push_back(nums[i]);
                dfs(nums, i + 1, current, result);
                current.pop_back();
            }
        }
};

// AC
// Rustの模範解答
class SolutionAnsRust {
    public:
        std::vector<std::vector<int>> subsetsWithDup(std::vector<int>& nums) {
            std::sort(nums.begin(), nums.end());
            std::vector<std::vector<int>> result;
            std::vector<int> subsets;
            backtrack(0, result, nums, subsets);

            return result;
        }
        
    private:
        void backtrack(int i, std::vector<std::vector<int>>& result, std::vector<int>& nums, std::vector<int>& subsets) {
            if(i == nums.size()) {
                result.push_back(subsets);
                return;
            }

            subsets.push_back(nums[i]);
            backtrack(i + 1, result, nums, subsets);
            subsets.pop_back();

            while(i + 1 < nums.size() && nums[i] == nums[i + 1]) {
                ++i;
            }
            backtrack(i + 1, result, nums, subsets);
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
    auto case_1 = std::vector{1, 2, 2};
    auto case_2 = std::vector{0};
    auto case_3 = std::vector{4, 1, 0};

    Solution s_1;

    auto res_s_1 = s_1.subsetsWithDup(case_1);
    auto res_s_2 = s_1.subsetsWithDup(case_2);

    show_result(res_s_1);
    show_result(res_s_2);

    SolutionAns s_ans;

    auto res_s_1_ans = s_ans.subsetsWithDup(case_1);
    auto res_s_2_ans = s_ans.subsetsWithDup(case_2);
    auto res_s_3_ans = s_ans.subsetsWithDup(case_3);
    show_result(res_s_1_ans);
    show_result(res_s_2_ans);
    show_result(res_s_3_ans);

    SolutionAnsRust s_ans_rs;
    auto res_s_1_ans_rs = s_ans_rs.subsetsWithDup(case_1);
    auto res_s_2_ans_rs = s_ans_rs.subsetsWithDup(case_2);
    auto res_s_3_ans_rs = s_ans_rs.subsetsWithDup(case_3);
    show_result(res_s_1_ans_rs);
    show_result(res_s_2_ans_rs);
    show_result(res_s_3_ans_rs);
}
