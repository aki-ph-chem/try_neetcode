#include <iostream>
#include <vector>
#include <algorithm>

struct Hoge {
    int x;
    int y;
};

bool operator<(const Hoge& lhs, const Hoge& rhs) {
    auto sq_norm_lhs = lhs.x * lhs.x + lhs.y * lhs.y;
    auto sq_norm_rhs = rhs.x * rhs.x + rhs.y * rhs.y;

    return sq_norm_lhs < sq_norm_rhs;
} 

struct Fuga {
    int x;
    int y;
    int z;
};

int main(void) {
    auto vec = std::vector{10, 30, 20, 50, 40};

    // 最大値、最小値のどちらかが欲しい場合
    std::cout << "min of vec: " << *std::min_element(vec.begin(), vec.end()) << std::endl;
    std::cout << "max of vec: " << *std::max_element(vec.begin(), vec.end()) << std::endl;

    // 最大値、最小値がペアで欲しい場合
    // std::pair<T,T>が帰ってくる(ただしTはポインタ)
    auto min_max_pair = std::minmax_element(vec.begin(), vec.end());
    std::cout << "min of vec: " << *min_max_pair.first << " max of vec: " << *min_max_pair.second << std::endl;

    // 構造体のベクタだった場合
    // '<' 演算子をオーバーロードする 
    auto vec_hoge = std::vector{Hoge{1,2}, Hoge{10, 20}, Hoge{3, 5}};
    auto min_max_hoge = std::minmax_element(vec_hoge.begin(), vec_hoge.end());
    std::cout << min_max_hoge.first->x << "," << min_max_hoge.first->y << std::endl; 
    std::cout << min_max_hoge.second->x << "," << min_max_hoge.second->y << std::endl; 

    // ラムダ式をコールバックで渡す
    auto vec_fuga = std::vector{Fuga{1,2,3}, Fuga{10, 20, 30}, Fuga{3, 5, 7}};
    auto ordering_fuga = [](const Fuga& lhs, const Fuga& rhs) {
        auto sq_norm_lhs = lhs.x * lhs.x + lhs.y * lhs.y + lhs.z * lhs.z;
        auto sq_norm_rhs = rhs.x * rhs.x + rhs.y * rhs.y + rhs.z * rhs.z;

        return sq_norm_lhs < sq_norm_rhs;
    };
    auto min_max_fuga = std::minmax_element(vec_fuga.begin(), vec_fuga.end(), ordering_fuga);
    std::cout << min_max_fuga.first->x << "," << min_max_fuga.first->y << "," << min_max_fuga.first->z <<std::endl; 
    std::cout << min_max_fuga.second->x << "," << min_max_fuga.second->y << "," << min_max_fuga.second->z <<std::endl; 
}
