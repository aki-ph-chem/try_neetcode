#include <iostream>
#include <vector>

// AC
class Solution {
    public:
        int searchInsert(std::vector<int>& nums, int target) {
            auto [l, r] = std::pair(0, (int)nums.size() - 1);
            auto mid = 0;

            while(l <= r) {
                mid = l + (r - l) / 2;
                if(target == nums[mid]) {
                    return mid;
                }

                if(nums[mid] < target) {
                    l = mid + 1;
                } else {
                    r = mid - 1;
                }
            }

            if(nums[mid] < target) {
                ++mid;
            }

            return mid;
        }
};

// AC
// 模範解答
class SolutionAns {
    public:
        int searchInsert(std::vector<int>& nums, int target) {
            int left = 0;
            int right = nums.size() - 1;

            while(left <= right) {
                int mid = left + (right - left) / 2;

                if(nums[mid] < target) {
                    left = mid + 1;
                }else {
                    right = mid - 1;
                }
            }

            return left;
        }
};

int main(void) {
    using Case = std::pair<std::vector<int>, int>;

    Case case_1 = {{1, 3, 5, 6}, 5};
    // => 2
    Case case_2 = {{1, 3, 5, 6}, 2};
    // => 1
    Case case_3 = {{1, 3, 5, 6}, 7};
    // => 4
    Case case_4 = {{1, 3, 5, 6}, 0};
    // => 0
    Case case_5 = {{3, 4, 5, 6, 7, 8}, 6};
    // => 3

    Solution s_1;
    std::cout << s_1.searchInsert(case_1.first, case_1.second) << std::endl;
    std::cout << s_1.searchInsert(case_2.first, case_2.second) << std::endl;
    std::cout << s_1.searchInsert(case_3.first, case_3.second) << std::endl;
    std::cout << s_1.searchInsert(case_4.first, case_4.second) << std::endl;
    std::cout << s_1.searchInsert(case_5.first, case_5.second) << std::endl;

    SolutionAns s_ans;
    std::cout << s_ans.searchInsert(case_1.first, case_1.second) << std::endl;
    std::cout << s_ans.searchInsert(case_2.first, case_2.second) << std::endl;
    std::cout << s_ans.searchInsert(case_3.first, case_3.second) << std::endl;
    std::cout << s_ans.searchInsert(case_4.first, case_4.second) << std::endl;
    std::cout << s_ans.searchInsert(case_5.first, case_5.second) << std::endl;
}
