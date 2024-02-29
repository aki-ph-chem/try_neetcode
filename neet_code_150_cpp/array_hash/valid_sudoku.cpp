#include <iostream>
#include <vector>
#include <unordered_map>
#include <unordered_set>
#include <map>

// 模範解答
class SolutionAns {
    public:
        bool is_valid_sudoku(const std::vector<std::vector<char>> & board) {
            const int cnt = 9;
            bool row[cnt][cnt] = {false};
            bool col[cnt][cnt] = {false};
            bool sub[cnt][cnt] = {false};

            for(int r = 0; r < cnt; ++r){
                for(int c = 0; c < cnt; ++c){
                    if(board[r][c] == '.')
                        continue; // if not number pass

                    int idx = board[r][c] - '0' - 1; //char to num idx
                    int area = (r/3) * 3 + (c/3); // これが??

                    //if number already exists
                    if(row[r][idx] || col[c][idx] || sub[area][idx]){
                        return false;
                    }

                    row[r][idx] = true;
                    col[c][idx] = true;
                    sub[area][idx] = true;
                }
            }
            return true;
        }
};

// AC
// Rustの模範解答より
class SolutionAnsRust {
    public:
        bool isValidSudoku(const std::vector<std::vector<char>>& board) {
            std::unordered_set<char> row, col, bx;

            for(int i = 0; i < 9; ++i) {
                for(int j = 0; j < 9; ++j) {
                    auto r = board[i][j];
                    auto c = board[j][i];
                    auto b = board[i / 3 * 3 + j / 3][i % 3 * 3 + j % 3];

                    if(r != '.') {
                        if(row.find(r) != row.end()) {
                            return false;
                        }
                        row.insert(r);
                    }

                    if(c != '.') {
                        if(col.find(c) != col.end()) {
                            return false;
                        }
                        col.insert(c);
                    }

                    if(b != '.') {
                        if(bx.find(b) != bx.end()) {
                            return false;
                        }
                        bx.insert(b);
                    }
                }

                row.clear();
                col.clear();
                bx.clear();
            }

            return true;
        }
};

// AC
// Pythonの模範解答より
class SolutionAnsPython {
    public:
        bool isValidSudoku(const std::vector<std::vector<char>>& board) {
            // key = r, key = c
            std::unordered_map<int, std::unordered_set<char>> row, col;
            // key = {r / 3, c /3}
            std::map<std::pair<int,int>, std::unordered_set<char>> subcell;

            for(int r = 0; r < 9; ++r) {
                for(int c = 0; c < 9; ++c) {
                    if(board[r][c] == '.') {
                        continue;
                    }

                    auto b_rc = board[r][c];
                    if(row[r].find(b_rc) != row[r].end()
                            || col[c].find(b_rc) != col[c].end()
                            || subcell[std::pair(r/3, c/3)].find(b_rc) 
                            != subcell[std::pair(r/3, c/3)].end() 
                            ) {
                        return false;
                    }

                    row[r].insert(b_rc);
                    col[c].insert(b_rc);
                    subcell[std::pair(r/3, c/3)].insert(b_rc);
                }
            }

            return true;
        }
};

int main(void) {
     auto case_1 = std::vector{
        std::vector{'5', '3', '.', '.', '7', '.', '.', '.', '.'},
        std::vector{'6', '.', '.', '1', '9', '5', '.', '.', '.'},
        std::vector{'.', '9', '8', '.', '.', '.', '.', '6', '.'},
        std::vector{'8', '.', '.', '.', '6', '.', '.', '.', '3'},
        std::vector{'4', '.', '.', '8', '.', '3', '.', '.', '1'},
        std::vector{'7', '.', '.', '.', '2', '.', '.', '.', '6'},
        std::vector{'.', '6', '.', '.', '.', '.', '2', '8', '.'},
        std::vector{'.', '.', '.', '4', '1', '9', '.', '.', '5'},
        std::vector{'.', '.', '.', '.', '8', '.', '.', '7', '9'},
     };
    auto case_2 = std::vector{
        std::vector{'8', '3', '.', '.', '7', '.', '.', '.', '.'},
        std::vector{'6', '.', '.', '1', '9', '5', '.', '.', '.'},
        std::vector{'.', '9', '8', '.', '.', '.', '.', '6', '.'},
        std::vector{'8', '.', '.', '.', '6', '.', '.', '.', '3'},
        std::vector{'4', '.', '.', '8', '.', '3', '.', '.', '1'},
        std::vector{'7', '.', '.', '.', '2', '.', '.', '.', '6'},
        std::vector{'.', '6', '.', '.', '.', '.', '2', '8', '.'},
        std::vector{'.', '.', '.', '4', '1', '9', '.', '.', '5'},
        std::vector{'.', '.', '.', '.', '8', '.', '.', '7', '9'},
    };

    SolutionAns s_1;
    std::cout <<"case_1" << std::endl;
    if(s_1.is_valid_sudoku(case_1)) {
        std::cout << "valid sudoku" << std::endl;
    }
    std::cout <<"case_2" << std::endl;
    if(s_1.is_valid_sudoku(case_2)) {
        std::cout << "valid sudoku" << std::endl;
    }

    SolutionAnsRust s_ans_rs;
    std::cout << s_ans_rs.isValidSudoku(case_1) << std::endl;
    std::cout << s_ans_rs.isValidSudoku(case_2) << std::endl;

    SolutionAnsPython s_ans_py;
    std::cout << s_ans_py.isValidSudoku(case_1) << std::endl;
    std::cout << s_ans_py.isValidSudoku(case_2) << std::endl;
}
