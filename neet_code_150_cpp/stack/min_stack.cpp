#include <vector>
#include <stack>
#include <iostream>

// AC
// しかしメモリ効率が悪い
class MinStack {
#define DEBUG
    private:
        std::vector<int> volume;
        std::vector<int> min_list;

    public:
        MinStack() {}

        void push(int val){
            if(min_list.empty()) {
                min_list.push_back(val);
            } else if(val <= min_list.back()) {
                min_list.push_back(val);
            }            

            volume.push_back(val);
        }

        void pop() {
            if(volume.back() == min_list.back()) {
                min_list.pop_back();
            }
            volume.pop_back();
        }

        int top() {
#ifdef DEBUG
            std::cout << volume.back() << std::endl;
#endif
            return volume.back();
        }

        int get_min() {
#ifdef DEBUG
            std::cout << min_list.back() << std::endl;
#endif
            return min_list.back();
        }

#ifdef DEBUG
        void show(void) {
            std::cout << "volume" << std::endl;
            for(const auto& v: volume) {
                std::cout << v << " ";
            }
            std::cout << std::endl;

            std::cout << "min_list" << std::endl;
            for(const auto& v: min_list) {
                std::cout << v << " ";
            }
            std::cout << std::endl;
        }
#endif
};

// AC
// メモリはあまり変わらない?
class MinStackB {
    private:
        std::vector<std::pair<int, int>> volume;
    public:
        MinStackB() {}

        void push(int val) {
            if(volume.empty()) {
                volume.push_back({val, val});
                return;
            }

            int current_min = volume.back().second;
            if(val <= current_min) {
                volume.push_back({val, val});
            } else {
                volume.push_back({val, current_min});
            }
        }

        void pop() {
            volume.pop_back();
        }

        int top() {
#ifdef DEBUG
            std::cout << volume.back().first << std::endl;
#endif
            return volume.back().first;
        }

        int get_min() {
#ifdef DEBUG
            std::cout << volume.back().second << std::endl;
#endif
            return volume.back().second;
        }
};

// 模範解答
class MinStackAns {
    public:
        MinStackAns() {
        }

        void push(int val) {
            stk.push(val);

            if (minStk.empty() || val < minStk.top().first) {
                minStk.push({val, 1});
            } else if (val == minStk.top().first) {
                minStk.top().second++;
            }
        }

        void pop() {
            if (stk.top() == minStk.top().first) {
                minStk.top().second--;
                if (minStk.top().second == 0) {
                    minStk.pop();
                }
            }
            stk.pop();
        }

        int top() {
            return stk.top();
        }

        int getMin() {
            return minStk.top().first;
        }
    private:
        std::stack<int> stk;
        std::stack<std::pair<int, int>> minStk;
};


int main(void) {
    MinStack ms_1;
    std::cout << "ms_1" << std::endl;

    ms_1.push(0);
    ms_1.push(1);
    ms_1.push(0);

    ms_1.get_min();
    ms_1.pop();
    ms_1.get_min();

    MinStack ms_2;
    std::cout << "ms_2" << std::endl;

    ms_2.push(-2);
    ms_2.push(0);
    ms_2.push(-3);

    ms_2.show();

    ms_2.get_min();
    ms_2.pop();
    ms_2.top();
    ms_2.get_min();

    MinStackB ms_3;
    std::cout << "ms_3" << std::endl;

    ms_3.push(0);
    ms_3.push(1);
    ms_3.push(0);

    ms_3.get_min();
    ms_3.pop();
    ms_3.get_min();

    MinStackB ms_4;
    std::cout << "ms_4" << std::endl;

    ms_4.push(-2);
    ms_4.push(0);
    ms_4.push(-3);

    ms_4.get_min();
    ms_4.pop();
    ms_4.top();
    ms_4.get_min();
}
