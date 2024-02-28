#include <iostream>

// 9x9のグリッドの中に3x3のサブグリッドを考える
void grid_subcell(int i) {
    for(int j = 0; j < 9; ++j) {
        auto i_in_cell = i / 3 * 3 + j / 3; 
        auto j_in_cell = i % 3 * 3 + j % 3;

        std::cout << "(i,j) = " << i << "," << j << ":"; 
        std::cout << "(j,i) = " << j << "," << i << ":"; 
        std::cout << "(i_in_cell,j_in_cell) = ";
        std::cout << i_in_cell << "," << j_in_cell << '\n';
    }
}

int main(void) {
    // i = 0のときの探索される点
    grid_subcell(0);
}
