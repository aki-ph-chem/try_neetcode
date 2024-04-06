#include <iostream>
#include <vector>

// Kotlinの模範解答より
class SolutionAnsKotlin {
    public:
        // AC
        bool search(std::vector<int>& nums, int target) {
            int left = 0, right = nums.size() - 1;
            while(left <= right) {
                int mid = left + (right - left) / 2;
                if(nums[mid] == target) {
                    return true;
                }

                if(nums[left] < nums[mid]) {
                    if(nums[left] <= target && nums[mid] > target) {
                        right = mid - 1;
                    } else {
                        left = mid + 1;
                    }
                } else if(nums[left] > nums[mid]){
                    if(nums[right] >= target && nums[mid] < target) {
                        left = mid + 1;
                    } else {
                        right = mid - 1;
                    }
                } else {
                    ++left;
                }
            }

            return false;
        }
};

int main(void) {
    using Case = std::pair<std::vector<int>, int>;
    Case case_1 = {{2, 5, 6, 0, 0, 1, 2}, 0};
    // => true
    Case case_2 = {{2, 5, 6, 0, 0, 1, 2}, 3};
    // => false
    Case case_3 = {{1, 0, 1, 1, 1}, 0};
    // => true

    SolutionAnsKotlin s_ans_kt;

    std::cout << s_ans_kt.search(case_1.first, case_1.second) << std::endl;
    std::cout << s_ans_kt.search(case_2.first, case_2.second) << std::endl;
    std::cout << s_ans_kt.search(case_3.first, case_3.second) << std::endl;
}
