#include <algorithm>
#include <vector>
#include <iostream>

class Solution {
    public:
        // リーバスしてから転置
        void rotate(std::vector<std::vector<int>>& matrix) {
            auto matrix_size = matrix.size();
            std::reverse(matrix.begin(), matrix.end());
            for(std::size_t i = 0; i < matrix_size; ++i) {
                for(std::size_t j = i; j < matrix_size; ++j) {
                    auto e_ij = matrix[i][j]; 
                    auto e_ji = matrix[j][i]; 
                    matrix[j][i] = e_ij;
                    matrix[i][j] = e_ji;
                }
            }
        }
};

// 模範解答
class SolutionAns {
    public:
  void rotate(std::vector<std::vector<int>>& matrix) {
        int n = matrix.size();
        for (int i = 0; i < n; i++) {
            for (int j = i; j < n; j++) {
                std::swap(matrix[i][j], matrix[j][i]);
            }
            reverse(matrix[i].begin(), matrix[i].end());
        }
    }
};

template<typename T>
using Matrix = std::vector<std::vector<T>>;

int main(void) {
    Matrix<int> case_1 = {
        {1, 2, 3},
        {4, 5, 6},
        {7, 8, 9}
    };
    Matrix<int> case_2 = { 
        {5, 1, 9, 11},
        {2, 4, 8, 10},
        {13, 3, 6, 7},
        {15, 14, 12, 16}
    };

    Solution s_1;

    auto result_1 = case_1;
    s_1.rotate(result_1);
    for(const auto& a: result_1) {
        for(const auto& b: a) {
            std::cout << b << " ";
        }
        std::cout << std::endl;
    }

    auto result_2 = case_2;
    s_1.rotate(result_2);
    for(const auto& a: result_2) {
        for(const auto& b: a) {
            std::cout << b << " ";
        }
        std::cout << std::endl;
    }

    SolutionAns s_2;

    auto result_1_ans = case_1;
    s_2.rotate(result_1_ans);
    for(const auto& a: result_1_ans) {
        for(const auto& b: a) {
            std::cout << b << " ";
        }
        std::cout << std::endl;
    }

    auto result_2_ans = case_2;
    s_2.rotate(result_2_ans);
    for(const auto& a: result_2_ans) {
        for(const auto& b: a) {
            std::cout << b << " ";
        }
        std::cout << std::endl;
    }
}
