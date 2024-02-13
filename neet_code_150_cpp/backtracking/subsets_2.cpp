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

    Solution s_1;

    auto res_s_1 = s_1.subsetsWithDup(case_1);
    auto res_s_2 = s_1.subsetsWithDup(case_2);

    show_result(res_s_1);
    show_result(res_s_2);
}
