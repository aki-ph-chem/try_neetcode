#include <iostream>
#include <vector>

class Solution {
    public:
        // AC
        int climbStairs(int n) {
            std::vector<int> table(n + 1);
            if(n == 1){
                return 1;
            }

            if(n == 2) {
                return 2;
            }

            if(2 < n) {
                table[1] = 1;
                table[2] = 2;
                for(int i = 3; i < n + 1; ++i) {
                    table[i] = table[i - 1] + table[i - 2];
                }
            }

            return table[n];
        }

        // TLE
        int starir_rec(int n) {
            if(n == 1) {
                return 1;
            } else if(n == 2) {
                return 2;
            } else {
                return starir_rec(n - 1) + starir_rec(n - 2);
            }
        }
};

int main(void) {
    /*
    Solution s_1;
    // 単純な再帰
    std::cout << s_1.starir_rec(1) << std::endl;
    std::cout << s_1.starir_rec(2) << std::endl;
    std::cout << s_1.starir_rec(3) << std::endl;
    std::cout << s_1.starir_rec(4) << std::endl;

    //dp
    std::cout << s_1.climbStairs(1) << std::endl;
    std::cout << s_1.climbStairs(2) << std::endl;
    std::cout << s_1.climbStairs(3) << std::endl;
    std::cout << s_1.climbStairs(4) << std::endl;
    std::cout << s_1.climbStairs(45) << std::endl;
    */

    Solution s_t;
    for(int i = 1; i <= 45; ++i) {
        std::cout << "i = " << i << " ";
        std::cout << s_t.climbStairs(i) << std::endl;;
    }
}
