#include <iostream>
#include <vector>
#include <stack>

void show_stack(std::stack<int> stk) {
    while(!stk.empty()) {
        std::cout << stk.top() << "";
        stk.pop();
    }
    std::cout << std::endl;
}

// AC
class SolutionAnsPython {
    public:
        bool validateStackSequences(std::vector<int>& pushed, std::vector<int>& popped) {
            int idx = 0;
            std::stack<int> stack;
            for(auto& n: pushed){
                stack.push(n);
                while(idx < popped.size() && !stack.empty() && popped[idx] == stack.top()) {

                    stack.pop();
                    ++idx;
                }
            }
            //show_stack(stack);

            return stack.empty();
        }
};

int main(void) {
    using Case = std::pair<std::vector<int>, std::vector<int>>;
    Case case_1 = {{1, 2, 3, 4, 5}, {4, 5, 3, 2, 1}};
    // => true
    Case case_2 = {{1, 2, 3, 4, 5}, {4, 3, 5, 1, 2}};
    // => false

    SolutionAnsPython s_ans_py;

    std::cout << s_ans_py.validateStackSequences(case_1.first, case_1.second) << std::endl; 
    std::cout << s_ans_py.validateStackSequences(case_2.first, case_2.second) << std::endl; 
}
