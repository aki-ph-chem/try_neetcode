#include <vector>
#include <iostream>

class SolutionAns {
public:
    void setZeroes(std::vector<std::vector<int>>& matrix) {
        int m = matrix.size();
        int n = matrix[0].size();
        
        bool isFirstRowZero = false;
        bool isFirstColZero = false;
        
        for (int i = 0; i < m; i++) {
            if (matrix[i][0] == 0) {
                isFirstColZero = true;
                break;
            }
        }
        
        for (int j = 0; j < n; j++) {
            if (matrix[0][j] == 0) {
                isFirstRowZero = true;
                break;
            }
        }
        
        for (int i = 1; i < m; i++) {
            for (int j = 1; j < n; j++) {
                if (matrix[i][j] == 0) {
                    matrix[i][0] = 0;
                    matrix[0][j] = 0;
                }
            }
        }
        
        for (int i = 1; i < m; i++) {
            for (int j = 1; j < n; j++) {
                if (matrix[i][0] == 0 || matrix[0][j] == 0) {
                    matrix[i][j] = 0;
                }
            }
        }
        
        if (isFirstColZero) {
            for (int i = 0; i < m; i++) {
                matrix[i][0] = 0;
            }
        }
        
        if (isFirstRowZero) {
            for (int j = 0; j < n; j++) {
                matrix[0][j] = 0;
            }
        }
    }
};

template<typename T>
using Matrix = std::vector<std::vector<T>>; 

int main(void) {
    Matrix<int> m_1 = {{1, 1, 1}, {1, 0, 1}, {1, 1, 1}}; 
    Matrix<int> m_2 = {{0, 1, 2, 0}, {3, 4, 5, 2}, {1, 3, 1, 5}}; 

    SolutionAns s_1;

    auto result_m_1 = m_1;
    s_1.setZeroes(result_m_1);
    for(const auto& v : result_m_1) {
        for(const auto& w: v) {
            std::cout << w << " ";
        }
        std::cout << std::endl;
    }

    auto result_m_2 = m_2;
    s_1.setZeroes(result_m_2);
    for(const auto& v : result_m_2) {
        for(const auto& w: v) {
            std::cout << w << " ";
        }
        std::cout << std::endl;
    }
}
