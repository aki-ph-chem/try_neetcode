#include <iostream>
#include <vector>

// 下位の桁から桁の値を取得する
std::vector<int> get_digits_back(int n) {
    std::vector<int> result;
    while(n > 0) {
        result.push_back(n % 10);
        n /= 10;
    }

    return result;
}

// 上位の桁から桁の値を取得する
std::vector<long long> get_digits_front(long long n) {
    long long div = 1;
    while(n >= 10 * div) {
        div *= 10;
    }

    std::vector<long long> result;
    while(n != 0) {
            result.push_back(n / div);
            n %= div;
            div /= 10;
    }

    return result;
}

template<class T>
void show(std::vector<T>& result) {
    for(auto&v: result) {
        std::cout << v << " ";
    }

    std::cout << std::endl;
}

int main(void) {
    auto res_1_b = get_digits_back(12345);
    auto res_1_f = get_digits_front(12345);

    show(res_1_b);
    show(res_1_f);
}
