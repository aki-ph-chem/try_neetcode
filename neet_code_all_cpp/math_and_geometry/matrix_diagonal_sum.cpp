#include <iostream>
#include <vector>

class Solution {
    public:
        // AC
        int diagonalSum(std::vector<std::vector<int>>& mat) {
            int n = mat.size();
            int result = 0;
            for(int i = 0; i < n; ++i) {
                result += mat[i][i];
                if(i != n - 1 - i) {
                    result += mat[i][n - 1 - i];
                }
            }

            return result;
        }
};

// Kotlinの模範解答
class SolutionAnsKotlin {
    public:
        int diagonalSum(std::vector<std::vector<int>>& mat) {
            int n = mat.size() - 1;
            int result = 0;

            for(int i = 0; i <= n; ++i) {
                result += mat[i][i] + mat[n - i][i];
            }

            if (n % 2 == 0){
                result -= mat[n/2][n/2];
            }

            return result;
        }
};

int main(void) {
    std::vector<std::vector<int>> case_1 = {{1,2,3},
        {4,5,6},
        {7,8,9}};
    // => 25
    std::vector<std::vector<int>> case_2 = {{1,1,1,1},
        {1,1,1,1},
        {1,1,1,1},
        {1,1,1,1}};
    // => 8
    std::vector<std::vector<int>> case_3 = {{5}};
    // => 5

    Solution s_1;

    std::cout << s_1.diagonalSum(case_1) << std::endl;
    std::cout << s_1.diagonalSum(case_2) << std::endl;
    std::cout << s_1.diagonalSum(case_3) << std::endl;

    SolutionAnsKotlin s_ans_kt;

    std::cout << s_ans_kt.diagonalSum(case_1) << std::endl;
    std::cout << s_ans_kt.diagonalSum(case_2) << std::endl;
    std::cout << s_ans_kt.diagonalSum(case_3) << std::endl;
}
