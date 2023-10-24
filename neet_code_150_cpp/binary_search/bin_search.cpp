#include <vector>
#include <iostream>

class Solution {
    public:
        int search(std::vector<int> &nums, int target) {
            int left = 0, right = nums.size() - 1;
            while(left <= right) {
                int mid = (left + right) / 2;
                if(nums[mid] == target) {
                    return mid;
                }

                if(nums[mid] < target) {
                    left = mid + 1;
                } else {
                    right = mid - 1;
                }
            }

            return -1;
        }

        int search_size_t(std::vector<int> &nums, int target) {
            std::size_t left = 0, right = nums.size();

            while(left < right) {
                std::size_t mid = (right + left) / 2;

                if(nums[mid] == target) {
                    return mid;
                }

                if(nums[mid] < target) {
                    left = mid + 1;
                } else {
                    right = mid;
                }
            }

            return -1;
        }
};

int main(void) {
    Solution s_1;

    auto case_1_nums = std::vector{-1, 0, 3, 5, 9, 12};
    auto case_1_target = 9;

    auto case_2_nums = std::vector{-1, 0, 3, 5, 9, 12};
    auto case_2_target = 2;

    auto case_3_nums = std::vector{2, 5};
    auto case_3_target = 0;

    std::cout << "index: int" << std::endl;
    std::cout << s_1.search(case_1_nums, case_1_target) << std::endl;
    std::cout << s_1.search(case_2_nums, case_2_target) << std::endl;
    std::cout << s_1.search(case_3_nums, case_3_target) << std::endl;

    std::cout << "index: std::size_t" << std::endl;
    std::cout << s_1.search_size_t(case_1_nums, case_1_target) << std::endl;
    std::cout << s_1.search_size_t(case_2_nums, case_2_target) << std::endl;
    std::cout << s_1.search_size_t(case_3_nums, case_3_target) << std::endl;

}
