#include <iostream>
#include <vector>

// 模範解答
class SolutionAns {
    public:
        // AC
        int findPeakElement(std::vector<int>& nums) {
            int n = nums.size();
            if(n == 1) {
                return 0;
            }

            auto [left, right] = std::pair(0, n - 1);
            while(left <= right) {
                int mid = left + (right - left) / 2;

                if(mid > 0 && nums[mid] < nums[mid - 1]) {
                    right = mid - 1;
                } else if (mid < n - 1 && nums[mid] < nums[mid + 1]) {
                    left = mid + 1;
                } else {
                    return mid;
                }
            }

            return -1;
        }

        // AC
        int findPeakElement2(std::vector<int>& nums) {
            int n = nums.size();
            if(n == 1) {
                return 0;
            }

            auto [left, right] = std::pair(0, n - 1);
            while(left <= right) {
                int mid = left + (right - left) / 2;

                if((mid == 0 && nums[mid] > nums[mid + 1] )|| 
                   (mid == n - 1 && nums[mid - 1] < nums[mid])|| 
                   (mid > 0 
                    && mid < n - 1 
                    && nums[mid - 1] < nums[mid]
                    && nums[mid] > nums[mid + 1])) {
                    return mid;
                }

                if(mid > 0 && nums[mid] < nums[mid - 1]) {
                    right = mid - 1;
                } else if (mid < n - 1 && nums[mid] < nums[mid + 1]) {
                    left = mid + 1;
                }             
            }

            return -1;
        }
};

int main(void) {
    std::vector<int> case_1 = {1, 2, 3, 1};
    // => 2
    std::vector<int> case_2 = {1, 2, 1, 3, 5, 6, 4};
    // => 5

    SolutionAns s_ans;

    std::cout << s_ans.findPeakElement(case_1) << std::endl;
    std::cout << s_ans.findPeakElement(case_2) << std::endl;

    std::cout << s_ans.findPeakElement2(case_1) << std::endl;
    std::cout << s_ans.findPeakElement2(case_2) << std::endl;
}
