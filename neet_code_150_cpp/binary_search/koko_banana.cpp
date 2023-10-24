#include <algorithm>
#include <cmath>
#include <vector>
#include <iostream>

class Solution {
    public:
        int min_eating_speed(std::vector<int>& piles, int h) {
            int n = piles.size();

            int low = 1;
            int high = 0;
            for (int i = 0; i < n; i++) {
                high = std::max(high, piles[i]);
            }

            int result = high;

            while (low <= high) {
                int k = low + (high - low) / 2;
                long int hours = 0;
                for (int i = 0; i < n; i++) {
                    hours += ceil((double) piles[i] / k);
                }
                if (hours <= h) {
                    result = std::min(result, k);
                    high = k - 1;
                } else {
                    low = k + 1;
                }
            }

            return result;
        }
};

int main(void) {
    auto case_1_array = std::vector{3, 6, 7, 11};
    auto case_1_h = 8;

    auto case_2_array = std::vector{30, 11, 23, 4, 20};
    auto case_2_h = 5;

    auto case_3_array = std::vector{30, 11, 23, 4, 20};
    auto case_3_h = 6;

    Solution s_1;

    std::cout << s_1.min_eating_speed(case_1_array, case_1_h) << std::endl;
    std::cout << s_1.min_eating_speed(case_2_array, case_2_h) << std::endl;
    std::cout << s_1.min_eating_speed(case_3_array, case_3_h) << std::endl;
}
