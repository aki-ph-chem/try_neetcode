#include <iostream>
#include <vector>

template<typename T>
void show_vector(std::vector<T>& vector) {
    for(auto &v: vector) {
        std::cout << v << " ";
    }
    std::cout << std::endl;
}

int main(void) {
    auto array_1 = std::vector{11, 22, 33};
    show_vector(array_1);

    array_1.push_back(44);
    show_vector(array_1);

    // array_1の後ろにdelta_1をつなげる
    auto delta_1 = std::vector{111, 444};
    array_1.insert(array_1.end(), delta_1.begin(), delta_1.end());
    show_vector(array_1);

    auto array_2 = std::vector{4, 5, 8, 2};
    auto iter_array_2 = array_2.begin();
    // array_2の1番目へのイテレータ
    ++iter_array_2;
    // array_2の一番目と2番目の間にdelta_1を挿入する
    array_2.insert(iter_array_2, delta_1.begin(), delta_1.end());
    show_vector(array_2);
}
