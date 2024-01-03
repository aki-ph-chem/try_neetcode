#include <iostream>
#include <vector>
#include <string>

// 模範解答
class SolutionAns {
public:
    bool exist(std::vector<std::vector<char>>& board, std::string word) {
        int m = board.size();
        int n = board[0].size();
        
        for (int i = 0; i < m; i++) {
            for (int j = 0; j < n; j++) {
                if (board[i][j] == word[0]) {
                    if (dfs(board, word, 0, i, j, m, n)) {
                        return true;
                    }
                }
            }
        }
        
        return false;
    }

private:
    bool dfs(std::vector<std::vector<char>>& board, std::string word,
        int index, int i, int j, int m, int n) {
        
        if (i < 0 || i >= m || j < 0 || j >= n || board[i][j] != word[index]) {
            return false;
        }
        if (index == word.size() - 1) {
            return true;
        }
        
        // 一旦値を変更
        board[i][j] = '#';
        
        if (dfs(board, word, index + 1, i - 1, j, m, n)
            || dfs(board, word, index + 1, i + 1, j, m, n)
            || dfs(board, word, index + 1, i, j - 1, m, n)
            || dfs(board, word, index + 1, i, j + 1, m, n)) {
            return true;
        }
        
        // 元に戻す
        board[i][j] = word[index];
        return false;
    }
};

int main(void) {
    auto case_1 = std::pair(
        std::vector{
            std::vector{'A', 'B', 'C', 'E'},
            std::vector{'S', 'F', 'C', 'S'},
            std::vector{'A', 'D', 'E', 'E'},
        },
        "ABCCED"
    );
    // => true
    auto case_2 = std::pair(
        std::vector{
            std::vector{'A', 'B', 'C', 'E'},
            std::vector{'S', 'F', 'C', 'S'},
            std::vector{'A', 'D', 'E', 'E'},
        },
        "SEE"
    );
    // => true
    auto case_3 = std::pair(
        std::vector{
            std::vector{'A', 'B', 'C', 'E'},
            std::vector{'S', 'F', 'C', 'S'},
            std::vector{'A', 'D', 'E', 'E'},
        },
        "ABCD"
    );
    // => false

    SolutionAns s_ans;

    std::cout << s_ans.exist(case_1.first, case_1.second) << std::endl;
    std::cout << s_ans.exist(case_2.first, case_1.second) << std::endl;
    std::cout << s_ans.exist(case_3.first, case_3.second) << std::endl;
}
