#include <vector>
#include <iostream>

// O(log(n))
// AC
class Solution {
    public:
        int search(const std::vector<int>& nums, int target) {
            int left = 0, right = nums.size() - 1;
            while(left <= right) {
                int mid = (left + right) / 2;

                if(nums[mid] == target) {
                    return mid;
                }

                if(nums[left] <= nums[mid]) {
                    if(nums[left] <= target && target <= nums[mid]) {
                        right = mid - 1;
                    } else {
                        left = mid + 1;
                    }
                } else {
                    if(nums[mid] <= target && target <= nums[right]) {
                        left = mid + 1;
                    } else {
                        right = mid - 1;
                    }
                }
            }

            return -1;
        }
};

int main(void) {
    auto case_1 = std::vector{4, 5, 6, 7, 0, 1, 2};
    int case_1_target = 0; // 4
                           
    auto case_2 = std::vector{4, 5, 6, 7, 0, 1, 2};
    int case_2_target = 3; // -1
                           
    auto case_3 = std::vector{1};
    int case_3_target = 0; // -1
                           
    auto case_4 = std::vector{3, 4, 5, 1, 2};
    int case_4_target = 2; // 4
                          
    // 普通の二分探索
    auto case_5 = std::vector{3, 4, 5};
    int case_5_target = 4; // 1
                           
    auto case_6 = std::vector{2, 3, 4, 5, 7};
    int case_6_target = 4; // 2

    Solution s_1;
    std::cout << "case_1: " << s_1.search(case_1, case_1_target) << std::endl;
    std::cout << "case_2: " << s_1.search(case_2, case_2_target) << std::endl;
    std::cout << "case_3: " << s_1.search(case_3, case_3_target) << std::endl;
    std::cout << "case_4: " << s_1.search(case_4, case_4_target) << std::endl;
    std::cout << "case_5: " << s_1.search(case_5, case_5_target) << std::endl;
    std::cout << "case_6: " << s_1.search(case_6, case_6_target) << std::endl;
}
