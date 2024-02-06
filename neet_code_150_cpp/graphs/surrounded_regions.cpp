#include <iostream>
#include <vector>

// 模範解答
class SolutionAns {
    public:
        void solve(std::vector<std::vector<char>>& board) {
            auto [m, n] = std::pair((int)board.size(), (int)board[0].size());

            // 右端、左端
            for(int i = 0; i < m; ++i) {
                dfs(board, {i, 0});
                dfs(board, {i, n - 1});
            }

            // 上端、下端
            for(int j = 0; j < n; ++j) {
                dfs(board, {0, j});
                dfs(board, {m - 1, j});
            }

            for(int i = 0; i < m; ++i) {
                for(int j = 0; j < n; ++j) {

                    if(board[i][j] == 'O') {
                        board[i][j] = 'X';
                    }

                    if(board[i][j] == 'E') {
                        board[i][j] = 'O';
                    }
                }
            }
        }

    private:
        void dfs(std::vector<std::vector<char>>& board, std::pair<int, int> v) {
            auto [m, n] = std::pair((int)board.size(), (int)board[0].size());
            // 境界チェック
            if(v.first < 0 
                    || v.first >= m 
                    || v.second < 0 
                    || v.second >= n 
                    || board[v.first][v.second] != 'O' 
                    ) 
            {
                return;
            }

            board[v.first][v.second] = 'E';

            // 十字方向の再帰的な探索
            std::vector<std::pair<int, int>> directions = {
                {1, 0},
                {0, 1},
                {-1, 0},
                {0, -1},
            };
            for(auto& s: directions) {
                dfs(board, {v.first + s.first, v.second + s.second});
            }
        }

};

void show_result(const std::vector<std::vector<char>>& result) {
    for(auto& v: result) {
        for(auto& w: v) {
            std::cout << w << " ";
        }
        std::cout << '\n';
    }
}

int main(void) {
    std::vector<std::vector<char>> case_1 = {
        {'X', 'X', 'X', 'X'},
        {'X', 'O', 'O', 'X'},
        {'X', 'X', 'O', 'X'},
        {'X', 'O', 'X', 'X'},
    };
    // =>
    /*
       [ ['X', 'X', 'X', 'X']
         ['X', 'X', 'X', 'X']
         ['X', 'X', 'X', 'X']
         ['X', 'O', 'X', 'X']]
    */
    std::vector<std::vector<char>> case_2 = {{'X'}};
    // => [['X']]

    SolutionAns s_ans;
    auto res_1 = case_1;
    auto res_2 = case_2;

    s_ans.solve(res_1);
    s_ans.solve(res_2);

    show_result(res_1);
    show_result(res_2);
}
