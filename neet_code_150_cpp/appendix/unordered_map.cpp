#include <iostream>
#include <unordered_map>
#include <vector>

int main(void) {

    std::unordered_map<std::string, int> map_1;
    // insrt()で要素を追加
    map_1.insert({"hoge", 12});
    map_1.insert({"huga", 13});
    map_1.insert({"piyo", 14});

    for(const auto &v: map_1) {
        std::cout << "(" << v.first << "," << v.second << ")" << std::endl;
    }

    // インデキシングで要素を追加
    map_1["piyo_piyo"] = 15;

    for(const auto &v: map_1) {
        std::cout << "(" << v.first << "," << v.second << ")" << std::endl;
    }

    std::unordered_map<int, std::vector<int>> map_2;
    map_2.insert({3, std::vector{1, 2, 3}});
    map_2.insert({6, std::vector{11, 22, 31}});
    map_2[8] = std::vector{111, 222};

    for(auto v: map_2) {
        std::cout << v.first << ": ";
        for(auto w: v.second) {
            std::cout << w << " ";
        }
        std::cout << std::endl;
    }

    if(map_2[1212].empty()) {
        std::cout << "map_2[1212] is empty" << std::endl;
    } else {
        std::cout << "foo" << std::endl;
    }
} 
