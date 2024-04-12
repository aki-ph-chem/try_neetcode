#include <cwchar>
#include <iostream>
#include <vector>

class Solution {
    public:
        // AC
        std::vector<int> searchRange(std::vector<int>& nums, int target) {
            auto [left, right] = std::pair(0, (int)nums.size() - 1);
            while(left <= right) {
                int mid = left + (right - left) / 2;

                if(nums[mid] == target) {
                    auto [l_mid, r_mid] = std::pair(mid, mid);

                    while(l_mid - 1 >= 0 && nums[l_mid - 1] == target) {
                        --l_mid;
                    }
                    while(r_mid + 1 < (int)nums.size() && nums[r_mid + 1] == target) {
                        ++r_mid;
                    }

                    return std::vector{l_mid, r_mid};

                }

                if(nums[mid] < target) {
                    left = mid + 1;
                } else if(nums[mid] > target) {
                    right = mid - 1;
                }
            }

            return std::vector{-1, -1};
        }
};

// 模範解答
class SolutionAns {
    public:
        std::vector<int> searchRange(std::vector<int>& nums, int target) {
            if(nums.empty()) {
                return {-1, -1};
            }
            auto [left, right] = std::pair(0, (int)nums.size() - 1);
            while(left < right) {
                int mid = left + (right - left) / 2;
                if(nums[mid] >= target) {
                    right = mid;
                } else {
                    left = mid + 1;
                }
            }
            if(nums[left] != target) {
                return {-1, -1};
            }
            int left_result = left;
            left = 0;
            right = nums.size() - 1;

            while(left < right) {
                int mid = left + (right - left + 1) / 2;
                if(nums[mid] <= target) {
                    left = mid;
                } else {
                    right = mid - 1;
                }
            }

            return {left_result, right};
        }
};

void show_res(std::vector<int>&& res) {
    std::cout << res[0] << " " << res[1] << std::endl;
}

int main(void) {
    using Case = std::pair<std::vector<int>, int>;
    Case case_1 = {{5, 7, 7, 8, 8, 10}, 8};
    // => [3, 4]
    Case case_2 = {{5, 7, 7, 8, 8, 10}, 6};
    // => [-1, -1]
    Case case_3 = {{}, 0};
    // => [-1, -1]

    Solution s_1;
    show_res(s_1.searchRange(case_1.first, case_1.second));
    show_res(s_1.searchRange(case_2.first, case_2.second));
    show_res(s_1.searchRange(case_3.first, case_3.second));

    SolutionAns s_ans;
    show_res(s_ans.searchRange(case_1.first, case_1.second));
    show_res(s_ans.searchRange(case_2.first, case_2.second));
    show_res(s_ans.searchRange(case_3.first, case_3.second));
}
