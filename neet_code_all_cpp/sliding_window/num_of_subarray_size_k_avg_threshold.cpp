#include <iostream>
#include <vector>
#include <deque>

class Solution {
    public:
        // TLE(RustだとACした)
        int numOfSubarrays(std::vector<int>& arr, int k, int threshold) {
            int result = 0;

            for(int i = 0; i < arr.size() - k + 1; ++i) {
                int sub_array_sum = 0;
                for(int j = i; j < i + k; ++j) {
                    sub_array_sum += arr[j];
                }

                if(sub_array_sum / k >= threshold) {
                    ++result;
                }
            } 

            return result;
        }

        // AC
        // O(N)
        int numOfSubarrays2(std::vector<int>& arr, int k, int threshold) {
            int result = 0;
            int sub_array_sum = 0;

            int i = 0;
            while(i < k) {
                sub_array_sum += arr[i];
                ++i;
            }
            if(sub_array_sum / k >= threshold) {
                ++result;
            }

            while(i < arr.size()) {
                sub_array_sum -= arr[i - k];
                sub_array_sum += arr[i];

                if(sub_array_sum / k >= threshold) {
                    ++result;
                }
                ++i;
            }

            return result;
        }
};

// 模範解答
class SolutionAns {
    public:
        int numOfSubarrays(std::vector<int>& arr, int k, int threshold) {
            int sum = 0;
            int n = (int)arr.size();
            for(int i = 0; i < k; ++i) {
                sum += arr[i];
            }

            int left = 0;
            int right = k;
            int result = 0;
            if(sum / k >= threshold) ++result;
            while(right < n) {
                sum -= arr[left++];
                sum += arr[right++];
                if(sum /k >= threshold) ++result;
            }

            return result;
        }
};

int main(void) {
    using Case = std::tuple<std::vector<int>, int, int>;

    Case case_1 = {{2, 2, 2, 2, 5, 5, 5, 8}, 3, 4};
    // => 3
    Case case_2 = {{11, 13, 17, 23, 29, 31, 7, 5, 2, 3}, 3, 6};
    // => 6

    Solution s_1;

    std::cout << s_1.numOfSubarrays(std::get<0>(case_1), std::get<1>(case_1), std::get<2>(case_1)) << std::endl;
    std::cout << s_1.numOfSubarrays(std::get<0>(case_2), std::get<1>(case_2), std::get<2>(case_2)) << std::endl;

    std::cout << s_1.numOfSubarrays2(std::get<0>(case_1), std::get<1>(case_1), std::get<2>(case_1)) << std::endl;
    std::cout << s_1.numOfSubarrays2(std::get<0>(case_2), std::get<1>(case_2), std::get<2>(case_2)) << std::endl;

    SolutionAns s_ans;

    std::cout << s_ans.numOfSubarrays(std::get<0>(case_1), std::get<1>(case_1), std::get<2>(case_1)) << std::endl;
    std::cout << s_ans.numOfSubarrays(std::get<0>(case_2), std::get<1>(case_2), std::get<2>(case_2)) << std::endl;
}
