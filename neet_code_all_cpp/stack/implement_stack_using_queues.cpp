#include <iostream>
#include <queue>
#include <utility>

// 解けなかった
class MyStack {
    private:
        std::queue<int> q_1;
        std::queue<int> q_2;

    public:
        MyStack() {}

        void push(int x) {
            q_1.push(x);
        }

        void pop() {
        }

        int top() {
        }

        bool empty() {
            return q_1.empty();
        }
};

// 模範解答
class MyStackAns {
    private:
        std::queue<int> q_1;
        std::queue<int> q_2;

    public:
        MyStackAns() {}

        void push(int x) {
            q_1.push(x);
        }

        int pop() {
            while(q_1.size() != 1) {
                q_2.push(q_1.front());
                q_1.pop();
            }
            int result = q_1.front();
            q_1.pop();
            std::swap(q_1, q_2);

            return result;
        }

        int top() {
            while(q_1.size() != 1) {
                q_2.push(q_1.front());
                q_1.pop();
            }

            int result = q_1.front();
            q_1.pop();
            std::swap(q_1, q_2);
            q_1.push(result);

            return result;
        }

        bool empty() {
            return (q_1.empty() && q_2.empty());
        }
};

int main(void) {
    MyStackAns s_ans;

    s_ans.push(1);
    s_ans.push(2);
    std::cout << s_ans.top() << std::endl;
    std::cout << s_ans.pop() << std::endl;
    std::cout << s_ans.empty() << std::endl;
}
