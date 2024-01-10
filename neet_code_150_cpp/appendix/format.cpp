#include <iostream>
#include <format>
#include <vector>
#include <string>

// Need C++20

int main(void) {
    // 基本
    std::cout << std::format("x = {}", 123) << std::endl; 
    auto msg = std::format("The answer is {}.", 100);
    std::cout << msg << std::endl;

    auto array = std::vector{1,2,3};
    // Need C++23
    // コンパイルできない
    //std::cout << std::format("array = {}", array) << std::endl;
}
