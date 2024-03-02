#include <climits>
#include <iostream>
#include <utility>
#include <vector>
#include <algorithm>

class Solution {
    public:
        // AC
        int minimumDifference(std::vector<int>& nums, int k) {
            std::sort(nums.begin(), nums.end());
            auto n = (int)nums.size();

            int result = INT_MAX;
            for(int i = 0; i < n; ++i) {
                if(i + k - 1 < n) {
                    result = std::min(result, nums[i + k - 1] - nums[i]);
                } else {
                    break;
                }
            }

            return result;
        }
};

// 模範解答
class SolutionAns {
    public:
        int minimumDifference(std::vector<int>& nums, int k) {
            std::sort(nums.begin(), nums.end());
            auto [l, r] = std::pair(0, k - 1);
            int result = INT_MAX;

            while(r < nums.size()) {
                result = std::min(result, nums[r] - nums[l]);
                ++l;
                ++r;
            }

            return result;
        }
};

int main(void) {
    using Case = std::pair<std::vector<int>, int>;

    Case case_1 = {{90}, 1};
    // => 0
    Case case_2 = {{9, 4, 1, 7}, 2};
    // => 2

    Solution s_1;
    std::cout << s_1.minimumDifference(case_1.first, case_1.second) << std::endl;
    std::cout << s_1.minimumDifference(case_2.first, case_2.second) << std::endl;

    SolutionAns s_ans;
    std::cout << s_ans.minimumDifference(case_1.first, case_1.second) << std::endl;
    std::cout << s_ans.minimumDifference(case_2.first, case_2.second) << std::endl;
}
