#include <iostream>
#include <iterator>
#include <vector>

int main(void) {
    auto v_1 = std::vector{1,2,3,4};

    // range base for
    for(const auto &v: v_1) {
        std::cout << v << " ";
    }
    std::cout << std::endl;

    // イテレータでfor

    // 前から
    for(auto iter = v_1.begin(); iter != v_1.end(); ++iter) {
        std::cout << *iter << " ";
    }
    std::cout << std::endl;

    // 後ろから
    for(auto iter = std::rbegin(v_1); iter != std::rend(v_1); ++iter) {
        std::cout << *iter << " ";
    }
    std::cout << std::endl;

}
