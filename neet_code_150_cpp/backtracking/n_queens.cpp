#include <ios>
#include <iostream>
#include <string>
#include <vector>
#include <unordered_set>

// 模範解答
class SolutionAns {
    public:
        std::vector<std::vector<std::string>> solveNQueens(int n) {
            std::vector<std::vector<std::string>> res;
            std::vector<std::string> board(n, std::string(n, '.'));
            backtrack(n, 0, res, board);

            return res;
        }

    private:
        std::unordered_set<int> cols;
        std::unordered_set<int> negDiag;
        std::unordered_set<int> posDiag;

        void backtrack(
                int n , 
                int row, 
                std::vector<std::vector<std::string>>& res,
                std::vector<std::string>& board
                ) {

            if(row == n) {
                res.push_back(board);
                return;
            }

            for(int col = 0; col < n; ++col) {
                if(
                        cols.find(col) != cols.end() 
                        || negDiag.find(row - col) != negDiag.end()
                        || posDiag.find(row + col) != posDiag.end()
                        ) {
                    continue;
                } 

                cols.insert(col);
                negDiag.insert(row - col);
                posDiag.insert(row + col);
                board[row][col] = 'Q';

                backtrack(n, row + 1, res, board);

                cols.erase(col);
                negDiag.erase(row - col);
                posDiag.erase(row + col);
                board[row][col] = '.';
            }
        }
};

void show_result(std::vector<std::vector<std::string>>& result) {
    for(auto& v: result) {
        for(auto& w: v) {
            std::cout << w << " ";
        }
        std::cout << '\n';
    }
}

int main(void) {
    SolutionAns s_ans;
    auto res_1_ans = s_ans.solveNQueens(4);
    auto res_2_ans = s_ans.solveNQueens(1);

    show_result(res_1_ans);
    show_result(res_2_ans);
}
