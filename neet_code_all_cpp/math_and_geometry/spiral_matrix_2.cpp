#include <cwchar>
#include <iostream>
#include <vector>

class Solution {
    public:
        // AC
        std::vector<std::vector<int>> generateMatrix(int n) {
            std::vector<std::vector<int>> result(n, std::vector<int>(n));
            int left = 0, right = n - 1;
            int top = 0, bottom = n - 1;
            int idx = 1;

            while(top <= bottom && left <= right) {
                for(int i = left; i <= right; ++i) {
                    result[top][i] = idx;
                    ++idx;
                }
                ++top;

                for(int i = top; i <= bottom; ++i) {
                    result[i][right] = idx;
                    ++idx;
                }
                --right;

                if(top <= bottom) {
                    for(int i = right; i >= left; --i) {
                        result[bottom][i] = idx;
                        ++idx;
                    }
                }
                --bottom;

                if(left <= right) {
                    for(int i = bottom; i >= top; --i) {
                        result[i][left] = idx;
                        ++idx;
                    }
                }
                ++left;
            }

            return result;
        }
};

class SolutionAnsKotlin {
    public:
        // AC
        std::vector<std::vector<int>> generateMatrix(int n) {
            std::vector<std::vector<int>> result(n, std::vector<int>(n));
            int left = 0, right = n - 1;
            int top = 0, bottom = n - 1;
            int count = 1;

            while(left <= right && top <= bottom) {
                for(int i = left; i <= right; ++i) {
                    result[top][i] = count;
                    ++count;
                }
                ++top;

                for(int i = top; i <= bottom; ++i) {
                    result[i][right] = count;
                    ++count;
                }
                --right;

                if(left > right || top > bottom) {
                    break;
                }

                for(int i = right; i >= left; --i) {
                    result[bottom][i] = count;
                    ++count;
                }
                --bottom;

                for(int i = bottom; i >= top; --i) {
                    result[i][left] = count;
                    ++count;
                }
                ++left;
            }

            return result;
        }
};

void show_result(std::vector<std::vector<int>>& result) {
    for(auto& v: result) {
        for(auto& w: v) {
            std::cout << w << " ";
        }
        std::cout << '\n';
    }
}

int main(void) {
    Solution s_1;

    int case_1 = 3;
    int case_2 = 1;

    auto res_1 = s_1.generateMatrix(case_1);
    auto res_2 = s_1.generateMatrix(case_2);

    show_result(res_1);
    show_result(res_2);

    SolutionAnsKotlin s_ans_kt;

    auto res_1_kt = s_ans_kt.generateMatrix(case_1);
    auto res_2_kt = s_ans_kt.generateMatrix(case_2);

    show_result(res_1_kt);
    show_result(res_2_kt);
}
