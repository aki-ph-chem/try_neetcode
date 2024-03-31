#include <iostream>
#include <string>
#include <stack>

// 模範解答
class SolutionAns {
    public:
        int minSwaps(std::string s) {
            int answer = 0;
            std::stack<char> stc;
            stc.push(']');

            auto n = (int)s.size();
            for(int i = 0; i < n; ++i) {
                auto top = stc.top();

                if(s[i] == ']') {
                    if(top == '[') {
                        stc.pop();
                    }else {
                        stc.push('[');
                        ++answer;
                    }
                } else {
                    stc.push('[');
                }
            }

            return answer;
        }
};

int main(void) {
    std::string case_1 = "][][";
    // => 1
    std::string case_2 = "]]][[[";
    // => 2

    SolutionAns s_ans;

    std::cout << s_ans.minSwaps(case_1) << std::endl;
    std::cout << s_ans.minSwaps(case_2) << std::endl;
}
