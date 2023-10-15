#include <iostream>
#include <vector>

std::size_t bin_search(const std::vector<int> nums, int target) {
    std::size_t idx_l = 0;
    std::size_t idx_r = nums.size() - 1;

    while(idx_l <= idx_r) {
        std::size_t mid = (idx_l + idx_r) / 2;

        if(nums[mid] == target) {
            return mid;
        }

        if(target < nums[mid]) {
            idx_r = mid - 1;
        } else {
            idx_l = mid + 1;
        }
    }
    return -1;
}

int main(void) {
    auto case_1_array = std::vector{1,3,6,8,10,12};
    std::cout << bin_search(case_1_array, 6) << std::endl;
    std::cout << bin_search(case_1_array, 8) << std::endl;
    std::cout << bin_search(case_1_array, 10) << std::endl;
    std::cout << bin_search(case_1_array, 1) << std::endl;
    std::cout << bin_search(case_1_array, 12) << std::endl;
}
