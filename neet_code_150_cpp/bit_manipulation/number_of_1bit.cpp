#include <cstdint>
#include <iostream>

// AC
class Solution {
    public:
        int hammingWeight(uint32_t n) {
            int sum = 0;
            for(int i = 0; i < 32; ++i) {
                if(n & (1 << i)) {
                    ++sum;
                }
            }

            return sum;
        }
};

// 模範解答
class SolutionAns {
public:
    int hammingWeight(uint32_t n) {
        int bit = 0;
        int result = 0;
        
        while (n != 0) {
            bit = n & 1;
            if (bit == 1) {
                result++;
            }
            n = n >> 1;
        }
        
        return result;
    }
};

int main(void) {
    uint32_t case_1 = 0b00000000000000000000000000001011;
    // => 3
    uint32_t case_2 = 0b00000000000000000000000010000000;
    // => 1
    uint32_t case_3 = 0b11111111111111111111111111111101;
    // => 32

    Solution s_1;
    std::cout << "case_1: " << s_1.hammingWeight(case_1) << std::endl;
    std::cout << "case_2: " << s_1.hammingWeight(case_2) << std::endl;
    std::cout << "case_3: " << s_1.hammingWeight(case_3) << std::endl;

    SolutionAns s_ans;
    std::cout << "case_1: " << s_ans.hammingWeight(case_1) << std::endl;
    std::cout << "case_2: " << s_ans.hammingWeight(case_2) << std::endl;
    std::cout << "case_3: " << s_ans.hammingWeight(case_3) << std::endl;
}
