#include <vector>
#include <iostream>

class SolutionAns {
    public:
        std::vector<int> spiral_order(std::vector<std::vector<int>>& matrix) {
            int left = 0;
            int top = 0;
            int right = matrix[0].size() - 1;
            int bottom = matrix.size() - 1;

            std::vector<int> result;

            while (top <= bottom && left <= right) {
                for (int j = left; j <= right; j++) {
                    result.push_back(matrix[top][j]);
                }
                top++;

                for (int i = top; i <= bottom; i++) {
                    result.push_back(matrix[i][right]);
                }
                right--;

                if (top <= bottom) {
                    for (int j = right; j >= left; j--) {
                        result.push_back(matrix[bottom][j]);
                    }
                }
                bottom--;

                if (left <= right) {
                    for (int i = bottom; i >= top; i--) {
                        result.push_back(matrix[i][left]);
                    }
                }
                left++;
            }

            return result;
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
        {1,2,3,4},
        {5,6,7,8},
        {9,10,11,12}
    };

    SolutionAns s_1;
    auto result_1 = s_1.spiral_order(case_1);
    for(const auto& a: result_1) {
        std::cout << a << " ";
    }
    std::cout << std::endl;

    auto result_2 = s_1.spiral_order(case_2);
    for(const auto& a: result_2) {
        std::cout << a << " ";
    }
    std::cout << std::endl;
}
