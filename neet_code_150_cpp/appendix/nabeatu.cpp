#include <iostream>
#include <vector>

// 世界のナベアツのように3の倍数と3が含まれる数値のときに'aho'と出力する

// 3が含まれるか否かの判定
bool is_contain_3(int x) {
    while(x > 0) {
        if(x % 10 == 3) {
            return true;
        }
        x /= 10;
    }

    return false;
}

void nabeatu_array(const std::vector<int>& nums) {
    for(const auto& v: nums) {
        std::cout << v << " ";
        if(v % 3 == 0) {
            std::cout << "aho" << std::endl;
        } else if(is_contain_3(v)) {
            std::cout << "aho" << std::endl;
        } else {
            std::cout << std::endl;
        }
    }
}

void nabeatu_n(int n) {
    for(int i = 1; i <= n; ++i) {
        std::cout << i << " ";
        if(i % 3 == 0) {
            std::cout << "aho" << std::endl;
        } else if(is_contain_3(i)) {
            std::cout << "aho" << std::endl;
        } else {
            std::cout << std::endl;
        }
    }
}

int main(void) {
    auto case_1 = std::vector{1,2,3,4,5,6,7,8,9,10,11,12,13,14};
    //nabeatu_array(case_1);

    //nabeatu_n(20);
    nabeatu_n(40);
}
