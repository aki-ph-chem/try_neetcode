#include <algorithm>
#include <vector>
#include <iostream>

// AC
class Solution {
    public:
        int find_min(std::vector<int>& nums) {
            std::size_t left = 0, right = nums.size() - 1;

            std::size_t mid = (left + right) / 2;
            while(left < right) {
                if(nums[mid] > nums[right]) {
                    left = mid + 1;
                } else  {
                    right = mid;
                }

                mid = (left + right) / 2;
            }

            return nums[mid];
        }
};

// 模範解答
class SolutionAns {
    public:
        int find_min(std::vector<int>& nums) {
// Neetcode solution Ologn time O1 space binary search
        int res = nums[0];
        int l = 0;
        int r = nums.size() - 1;

        while (l <= r)
        {
            if (nums[l] < nums[r])
            {
                res = std::min(res, nums[l]);
                break;
            }
            int mid = l + (r - l) / 2;
            res = std::min(res, nums[mid]);

            if (nums[mid] >= nums[l]) // mid present in left sorted array
            {
                l = mid + 1; // try to move closer to right sorted array
            }
            else
            {
                r = mid - 1;
            }
        }

        return res;
        }
};

int main(void) {
    auto case_1 = std::vector{3, 4, 5, 1, 2};
    auto case_2 = std::vector{4, 5, 6, 7, 0, 1, 2};
    auto case_3 = std::vector{11, 13, 15, 17};
    auto case_4 = std::vector{2, 3, 4, 5, 1, 2};
    auto case_5 = std::vector{2, 3};
    auto case_6 = std::vector{3, 2};
    auto case_7 = std::vector{3};
    auto case_8 = std::vector{3, 4, 5, 1, 1, 2};

    Solution s_1;

    std::cout << "Solution" << std::endl;
    std::cout << "case_1: " << s_1.find_min(case_1) << std::endl;
    std::cout << "case_2: " << s_1.find_min(case_2) << std::endl;
    std::cout << "case_3: " << s_1.find_min(case_3) << std::endl;
    std::cout << "case_4: " << s_1.find_min(case_4) << std::endl;
    std::cout << "case_5: " << s_1.find_min(case_5) << std::endl;
    std::cout << "case_6: " << s_1.find_min(case_6) << std::endl;
    std::cout << "case_7: " << s_1.find_min(case_7) << std::endl;
    std::cout << "case_8: " << s_1.find_min(case_8) << std::endl;

    SolutionAns s_2;
    std::cout << "SolutionAns" << std::endl;
    std::cout << "case_1: " << s_2.find_min(case_1) << std::endl;
    std::cout << "case_2: " << s_2.find_min(case_2) << std::endl;
    std::cout << "case_3: " << s_2.find_min(case_3) << std::endl;
    std::cout << "case_4: " << s_2.find_min(case_4) << std::endl;
    std::cout << "case_5: " << s_2.find_min(case_5) << std::endl;
    std::cout << "case_6: " << s_2.find_min(case_6) << std::endl;
    std::cout << "case_7: " << s_2.find_min(case_7) << std::endl;
    std::cout << "case_8: " << s_2.find_min(case_8) << std::endl;
}
