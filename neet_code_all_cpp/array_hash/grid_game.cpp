#include <climits>
#include <iostream>
#include <vector>

// 模範解答
class SolutionAns {
    public:
        long long gridGame(std::vector<std::vector<int>>& grid) {
            int n = grid[0].size();
            long long top = grid[0][0], bottom = 0;
            long long result = LONG_MAX;
            for(int i = 1; i < n; ++i) {
                top += grid[0][i];
            }

            for(int i = 0; i < n; ++i) {
                top -= grid[0][i];

                result = std::min(result, std::max(top, bottom));

                bottom += grid[1][i];
            }

            return result;
        }
};

// AC
// Rustの模範解答より
class SolutionAnsRust {
    public:
        long long gridGame(std::vector<std::vector<int>>& grid) {
            auto n = (int)grid[0].size();
            std::vector<long long> memo_0(n + 1), memo_1(n + 1);
            for(int i = 0; i < n; ++i) {
                memo_0[i + 1] = memo_0[i] + (long long)grid[0][i];
                memo_1[i + 1] = memo_1[i] + (long long)grid[1][i];
            }

            long long result = LONG_MAX;
            for(int i = 0; i < n; ++i) {
                result = std::min(result, std::max(memo_1[i], memo_0[n] - memo_0[i + 1]));
            }

            return result;
        }
};

int main(void) {
    using Case = std::vector<std::vector<int>>;

    Case case_1 = {{2,5,4},{1,5,1}};
    // => 4
    Case case_2 = {{3,3,1},{8,5,2}};
    // => 4
    Case case_3 = {{1,3,1,15},{1,3,3,1}};
    // => 7

    SolutionAns s_ans;

    std::cout << s_ans.gridGame(case_1) << std::endl;
    std::cout << s_ans.gridGame(case_2) << std::endl;
    std::cout << s_ans.gridGame(case_3) << std::endl;

    SolutionAnsRust s_ans_rs;

    std::cout << s_ans_rs.gridGame(case_1) << std::endl;
    std::cout << s_ans_rs.gridGame(case_2) << std::endl;
    std::cout << s_ans_rs.gridGame(case_3) << std::endl;
}
