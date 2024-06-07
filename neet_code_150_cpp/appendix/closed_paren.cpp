#include <iostream>
#include <string>
#include <stack>

bool is_valid_paren(std::string& s) {
    std::stack<char> stack;
    for(auto&c: s) {
        if(!stack.empty()) {
            auto stack_top = stack.top();
            if(stack_top == '(' && c == ')') {
                stack.pop();
            } else {
                stack.push(c);
            }
        } else {
            stack.push(c);
        }
    }

    return stack.empty();
}

int main(void) {
    std::string case_1 = "(())";
    // => true
    std::string case_2 = "()()()";
    // => true
    std::string case_3 = ")(";
    // => false
    std::string case_4 = "(()()";
    // => false

    std::cout << is_valid_paren(case_1) << std::endl;
    std::cout << is_valid_paren(case_2) << std::endl;
    std::cout << is_valid_paren(case_3) << std::endl;
    std::cout << is_valid_paren(case_4) << std::endl;
}
