#include <climits>
#include <iostream>
#include <system_error>
#include <vector>

class SolutionAns {
    public:
        double findMedianSortedArrays(std::vector<int>& nums1, std::vector<int>& nums2) {
            int m = nums1.size();
            int n = nums2.size();

            if(m > n) {
                return findMedianSortedArrays(nums2, nums1);
            }

            int total = m + n;
            int low = 0; 
            int high = m;
            double result = 0.0;

            while(low <= high) {
                // nums1
                int i = low + (high - low) / 2;
                // nums2
                int j = (total + 1) / 2 - i;

                int left_1 = (i > 0)? nums1[i - 1]: INT_MIN;
                int right_1 = (i < m)? nums1[i] : INT_MAX; 
                int left_2 = (j > 0)? nums2[j - 1] : INT_MIN;
                int right_2 = (j < n)? nums2[j]: INT_MAX;

                // partition is correct
                if(left_1 <= right_2 && left_2 <= right_1) {
                    // even
                    if(total % 2 == 0) {
                        result = (std::max(left_1, left_2) + std::min(right_1, right_2)) / 2.0;
                        // odd
                    } else {
                        result = std::max(left_1, left_2);
                    }
                    break;
                }  else if(right_2 < left_1) {
                    high = i - 1;
                } else {
                    low = i + 1;
                }
            }

            return result;
        }
};

int main(void) {
    std::pair<std::vector<int>, std::vector<int>> case_1 = {{1, 3},{2}};
    // => 2
    std::pair<std::vector<int>, std::vector<int>> case_2 = {{1, 2},{3, 4}};
    // => 2.5

    SolutionAns s_ans;
    std::cout << s_ans.findMedianSortedArrays(case_1.first, case_1.second) << std::endl; 
    std::cout << s_ans.findMedianSortedArrays(case_2.first, case_2.second) << std::endl; 
}
