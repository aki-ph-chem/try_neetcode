#include <iostream>
#include <vector>

class Solution {
    public:
        bool canPartition(std::vector<int>& nums) {
            int n = nums.size();

            for(long long bit = 0; bit < 1<<n; ++bit) {
                int sum_part_1 = 0, sum_part_2 = 0;
                for(long long i = 0; i < n; ++i) {
                    if(bit & (1 << i)) {
                        ++sum_part_1;
                    } else {
                        ++sum_part_2;
                    }
                }

                if(sum_part_1 == sum_part_2) {
                    return true;
                }
            }

            return false;
        }
};

int main(void) {
    auto case_1 = std::vector{1, 5, 11, 5};
    // true
    auto case_2 = std::vector{1, 2, 3, 5};
    // false
    auto case_3 = std::vector{
        100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100,
        100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100,
        100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100,
        100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100,
        100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100,
        100, 100, 100, 100, 100, 100, 100, 100, 100, 100,
    };
    // true


    Solution s_1;

    std::cout << s_1.canPartition(case_1) << std::endl;
    std::cout << s_1.canPartition(case_2) << std::endl;
    std::cout << s_1.canPartition(case_3) << std::endl;
}
