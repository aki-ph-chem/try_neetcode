#include <iostream>
#include <vector>

// 模範解答
class SolutionAns {
    // Rustの模範解答よりもDPっぽい
    public:
        int uniquePaths(int m, int n) {
            std::vector<std::vector<int>> grid(m, std::vector<int>(n, 0));

            for(int i = 0; i < m; ++i) {
                grid[i][0] = 1;
            }

            for(int j = 0; j < n; ++j) {
                grid[0][j] = 1;
            }

            for(int i = 1; i < m; ++i) {
                for(int j = 1; j < n; ++j) {
                    grid[i][j] = grid[i - 1][j] + grid[i][j - 1];
                }
            }

            return grid[m - 1][n - 1];
        }

        // Rustの模範解答より
        int uniquePaths2(int m, int n) {
            std::vector<int> bottom(n, 1);

            for(int i = 0; i < m - 1; ++i) {
                std::vector<int> top(n, 1);
                for(int c = n - 2; c >= 0; --c) {
                    top[c] = bottom[c] + top[c + 1];
                }

                bottom = top;
            }

            return bottom[0];
        }
};

int main(void) {
    auto case_1 = std::pair(3, 7);
    auto case_2 = std::pair(3, 2);

    SolutionAns s_ans;

    std::cout << s_ans.uniquePaths(case_1.first, case_1.second) << std::endl;
    std::cout << s_ans.uniquePaths(case_2.first, case_2.second) << std::endl;

    std::cout << s_ans.uniquePaths2(case_1.first, case_1.second) << std::endl;
    std::cout << s_ans.uniquePaths2(case_2.first, case_2.second) << std::endl;
}
