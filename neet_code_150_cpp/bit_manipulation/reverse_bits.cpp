#include <cstdint>
#include <iostream>
#include <stack>
#include <system_error>

class Solution {
    public:
        // AC
        uint32_t reverseBits(uint32_t n) {
            uint32_t result = 0;
            std::stack<int> bit_list;

            for(int i = 0; i < 32; ++i) {
                if(n & (1 << i)) {
                    bit_list.push(1);
                } else {
                    bit_list.push(0);
                }
            }

            int i = 0;
            while(!bit_list.empty()) {
                result += bit_list.top() * (1 << i);
                bit_list.pop();
                ++i;
            }

            return result;
        }

        // AC
        uint32_t reverseBits2(uint32_t n) {
            uint32_t result = 0;

            int b = 0;
            for(int a = 31; a >= 0; --a) {
                if(n & (1 << a)) {
                    result += 1 << b;
                }
                ++b;
            }

            return result;
        }
};

// 模範解答
class SolutionAns {
public:
    uint32_t reverseBits(uint32_t n) {
        uint32_t result = 0;
        
        for (int i = 0; i < 32; i++) {
            result <<= 1;
            result |= n & 1;
            n >>= 1;
        }
        
        return result;
    }
};

int main(void) {
    uint32_t case_1 = 0b00000010100101000001111010011100;
    // => 0b00111001011110000010100101000000 (964176192)
    uint32_t case_2 = 0b11111111111111111111111111111101;
    // => 0b10111111111111111111111111111111 (3221225471)

    Solution s_1;
    std::cout << s_1.reverseBits(case_1) << std::endl;
    std::cout << s_1.reverseBits(case_2) << std::endl;

    std::cout << s_1.reverseBits2(case_1) << std::endl;
    std::cout << s_1.reverseBits2(case_2) << std::endl;

    SolutionAns s_ans;
    std::cout << s_ans.reverseBits(case_1) << std::endl;
    std::cout << s_ans.reverseBits(case_2) << std::endl;
}
