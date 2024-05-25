#include <iostream>
#include <vector>

// 模範解答
class Solution {
    public:
        int firstMissingPositive(std::vector<int> nums) {
            for(int i = 0; i < nums.size(); ++i) {
                if(i + 1 == nums[i]) continue;
                int x = nums[i];
                while(x >= 1 && x <= nums.size() && x != nums[x - 1]) {
                    std::swap(x, nums[x - 1]);
                }
            }
            for(int i = 0; i < nums.size(); ++i) {
                if(i + 1 != nums[i]) {
                    return i + 1;
                }
            }

            return nums.size() + 1;
        }
};

int main(void) {
    std::vector<int> case_1 = {1, 2, 0};
    // => 3
    std::vector<int> case_2 = {3, 4, -1, 1};
    // => 2
    std::vector<int> case_3 = {7, 8, 9, 11, 12};
    // => 1

    Solution s_1;

    std::cout << s_1.firstMissingPositive(case_1) << std::endl;
    std::cout << s_1.firstMissingPositive(case_2) << std::endl;
    std::cout << s_1.firstMissingPositive(case_3) << std::endl;
}
