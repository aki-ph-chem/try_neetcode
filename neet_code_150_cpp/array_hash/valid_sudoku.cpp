#include <iostream>
#include <vector>

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
}
