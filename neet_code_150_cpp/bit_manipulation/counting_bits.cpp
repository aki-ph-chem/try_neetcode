#include <iostream>
#include <vector>

class Solution {
    public:
        std::vector<int> countBits(int n) {
            std::vector<int> result(n + 1, 0);
            for(int i = 0; i <= n; ++i) {
                result[i] = count(i);
            }

            return result;
        }

    private:
        int count(int n) {
            int res = 0;
            while(n != 0) {
                int bit = n & 1;
                if(bit == 1) {
                    ++res;
                }
                n = n >> 1;
            }

            return res;
        }
};

int main(void) {
    int case_1 = 2;
    // => [0,1,1]
    int case_2 = 5;
    // => [0,1,1,2,1,2]

    Solution s_1;
    auto res_1 = s_1.countBits(case_1);
    auto res_2 = s_1.countBits(case_2);

    for(auto& v: res_1) {
        std::cout << v << " ";
    }
    std::cout << std::endl;
    for(auto& v: res_2) {
        std::cout << v << " ";
    }
    std::cout << std::endl;
}
