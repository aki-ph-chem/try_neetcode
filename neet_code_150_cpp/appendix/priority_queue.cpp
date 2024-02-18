#include <functional>
#include <iostream>
#include <vector>
#include <queue>

int main(void) {
    // 基本的な使い方
    // デフォルトでは降順(std::less<T>)
    std::priority_queue<int> q_max;
    q_max.push(5);
    q_max.push(2);
    q_max.push(10);

    std::cout << "q_max" << std::endl;
    while(!q_max.empty()) {
        std::cout << q_max.top() << std::endl;
        q_max.pop();
    }

    // 昇順にする
    std::priority_queue<int,
        // コンテナ
        std::vector<int>,
        // 昇順に指定
        std::greater<int>> q_min;
    q_min.push(5);
    q_min.push(2);
    q_min.push(10);

    std::cout << "q_min" << std::endl;
    while(!q_min.empty()) {
        std::cout << q_min.top() << std::endl;
        q_min.pop();
    }

    // std::pair<int, int> を要素とする場合
    std::priority_queue<std::pair<int, int>,
        std::vector<std::pair<int, int>>,
        // 昇順に指定
        std::greater<std::pair<int, int>>> q_pair;
    q_pair.push({5, 31});
    q_pair.push({2, 13});
    q_pair.push({10, 23});

    std::cout << "q_pair" << std::endl;
    while(!q_pair.empty()) {
        std::cout << q_pair.top().first << "," << q_pair.top().second << '\n';
        q_pair.pop();
    }

    // std::vector<int> を要素とする場合
    std::priority_queue<std::vector<int>,
        std::vector<std::vector<int>>,
        std::greater<std::vector<int>>> q_vec;
    q_vec.push({2, 4});
    q_vec.push({11, 12, 14, 15});
    q_vec.push({21, 41, 61});

    std::cout << "q_vec" << std::endl;
    while(!q_vec.empty()) {
        for(auto& v: q_vec.top()) {
            std::cout << v << " ";
        }
        std::cout << '\n';
        q_vec.pop();
    }
}
