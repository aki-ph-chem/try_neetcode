#include <iostream>
#include <vector>
#include <string>
#include <unordered_map>

// std::unordered_mapを使う
void switch_str(std::string&& str) {
    auto map = std::unordered_map<std::string, int>{{"foo", 1}, {"hoge", 2}, {"piyo", 4}};

    auto key = 0;
    if(map.find(str) != map.end()) {
        key = map[str];
    } else {
        std::cerr << "Error" << std::endl;
        return;
    }

    switch(key) {
        case 1:
            std::cout<< "ok! foo" << std::endl;
            break;
        case 2:
            std::cout<< "ok! hoge" << std::endl;
            break;
        case 4:
            std::cout<< "ok! piyo" << std::endl;
            break;
    }
}

// enumを使う
void switch_str_2(std::string&& str) {
    enum string_code {
        Foo,
        Hoge,
        Piyo,
        Default,
    };

    auto hash_hit = [](std::string& str){
        if(str == "foo") return Foo;
        if(str == "hoge") return Hoge;
        if(str == "piyo") return Piyo;
        else return Default;
    };

    switch(hash_hit(str)) {
        case Foo:
            std::cout << "ok! foo" << std::endl;
            break;
        case Hoge:
            std::cout << "ok! hoge" << std::endl;
            break;
        case Piyo:
            std::cout << "ok! piyo" << std::endl;
            break;
        case Default:
            std::cerr << "Error" << std::endl;
            break;
    }
}

int main(void) {
    switch_str("foo");
    switch_str("ooo");

    switch_str_2("foo");
    switch_str_2("ooo");
    switch_str_2("piyo");
}
