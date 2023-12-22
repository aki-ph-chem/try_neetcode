#include <iostream>
#include <unordered_set>
#include <vector>

// 解けなかった
class Solution {
    public:
        bool canPartition(std::vector<int>& nums) {
            int n = nums.size();

            for(long long bit = 0; bit < 1<<n; ++bit) {
                int sum_part_1 = 0, sum_part_2 = 0;
                for(long long i = 0; i < n; ++i) {
                    if(bit & (1 << i)) {
                        ++sum_part_1;
                    } else {
                        ++sum_part_2;
                    }
                }

                if(sum_part_1 == sum_part_2) {
                    return true;
                }
            }

            return false;
        }
};

// 模範解答
class SolutionAns {
    public:
        bool canPartition(std::vector<int>& nums) {
            int target = 0;
            for (int i = 0; i < nums.size(); i++) {
                target += nums[i];
            }
            if (target % 2 != 0) {
                return false;
            }
            target /= 2;

            std::unordered_set<int> dp;
            dp.insert(0);

            for (int i = 0; i < nums.size(); i++) {
                std::unordered_set<int> dpNext;
                for (auto it = dp.begin(); it != dp.end(); it++) {
                    if (*it + nums[i] == target) {
                        return true;
                    }
                    dpNext.insert(*it + nums[i]);
                    dpNext.insert(*it);
                }
                dp = dpNext;
            }

            return false;
        }
};

int main(void) {
    auto case_1 = std::vector{1, 5, 11, 5};
    // true
    auto case_2 = std::vector{1, 2, 3, 5};
    // false
    auto case_3 = std::vector{
        100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100,
        100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100,
        100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100,
        100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100,
        100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100,
        100, 100, 100, 100, 100, 100, 100, 100, 100, 100,
    };
    // true


    Solution s_1;

    std::cout << s_1.canPartition(case_1) << std::endl;
    std::cout << s_1.canPartition(case_2) << std::endl;
    std::cout << s_1.canPartition(case_3) << std::endl;

    SolutionAns s_ans;

    std::cout << s_ans.canPartition(case_1) << std::endl;
    std::cout << s_ans.canPartition(case_2) << std::endl;
    std::cout << s_ans.canPartition(case_3) << std::endl;
}
