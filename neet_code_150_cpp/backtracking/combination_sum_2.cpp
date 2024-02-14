#include <iostream>
#include <vector>
#include <set>
#include <algorithm>

class Solution {
    public:
        // TLE
        // ビット演算で解く
        // O(N 2^N)
        std::vector<std::vector<int>> combinationSum2Bit(std::vector<int>& candidates, int target) {
            auto n = (int)candidates.size();
            std::vector<std::vector<int>> result;
            std::set<std::vector<int>> set;

            for(int bit = 0; bit < (1 << n); ++bit) {
                int sum = 0;
                std::vector<int> sub_set = {};
                for(int i = 0; i < n; ++i) {
                    if(bit & (1 << i)) {
                        sum += candidates[i];
                        sub_set.push_back(candidates[i]);
                    }
                }

                std::sort(sub_set.begin(), sub_set.end());
                if(sum == target && set.find(sub_set) == set.end()) {
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
        std::vector<std::vector<int>> combinationSum2(std::vector<int>& candidates, int target) {
            std::sort(candidates.begin(), candidates.end());
            std::vector<int> current;
            std::vector<std::vector<int>> result;

            dfs(candidates, target, 0, 0, current, result);

            return result;
        }

    private:
        void dfs(
                std::vector<int>& candidates, 
                int target, 
                int sum, 
                int start, 
                std::vector<int>& current, 
                std::vector<std::vector<int>>& result
                ) {

            if(sum > target) {
                return;
            }
            if(sum == target) {
                result.push_back(current);
                return;
            }

            for(int i = start; i < candidates.size(); ++i) {
                if(i > start && candidates[i] == candidates[i - 1]) {
                    continue;
                }

                current.push_back(candidates[i]);
                dfs(candidates, target, sum + candidates[i], i + 1, current, result);
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
    std::pair<std::vector<int>, int> case_1 = {{10, 1, 2, 7, 6, 1, 5}, 8};
    std::pair<std::vector<int>, int> case_2 = {{2, 5, 2, 1, 2}, 5};

    Solution s_1;

    auto res_1 = s_1.combinationSum2Bit(case_1.first, case_1.second);
    auto res_2 = s_1.combinationSum2Bit(case_2.first, case_2.second);
    show_result(res_1);
    show_result(res_2);
}
