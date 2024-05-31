#include <iostream>
#include <string>
#include <vector>

template<class T>
void show(std::vector<T>& array) {
    for(auto& v: array) {
        std::cout << v << " ";
    }
    std::cout << std::endl;
}

// Rustの模範解答より
class SolutionAnsRust {
    public:
        std::string decodeString(std::string s) {
            std::vector<std::string> stack;

            for(auto&c : s) {
                if(c != ']') {
                    stack.push_back(std::string{c});
                } else {
                    std::string sub_str = "";
                    while(!stack.empty() && stack.back() != "[") {
                        sub_str = stack.back() + sub_str;
                        stack.pop_back();
                    }
                    stack.pop_back();

                    std::string multi_plier = "";
                    while(!stack.empty()) {
                        // 例外を使って文字列が整数か判別
                        try {
                            int num = std::stoi(stack.back());
                            multi_plier = stack.back() + multi_plier;
                            stack.pop_back();
                        }
                        catch (...) {
                            break;
                        }
                    }
                    //std::cout << multi_plier << std::endl;

                    std::string str_push_stack = "";
                    for(int i = 0; i < std::stoi(multi_plier); ++i) {
                        str_push_stack += sub_str;
                    }
                    stack.push_back(str_push_stack);
                }
            }

            //show(stack);
            std::string result = "";
            for(auto& s: stack){
                result += s;
            }

            return result;
        }

        // 例外を使わずに解く
        // AC
        std::string decodeString2(std::string s) {
            std::vector<std::string> stack;

            auto is_numeric = [](std::string& s) {
                int lower = 0; 
                int upper = '9' - '0';
                for(auto& c: s) {
                    int diff = c - '0';
                    if(diff < 0 || diff > upper) {
                        return false;
                    } 
                }

                return true;
            };

            for(auto&c : s) {
                if(c != ']') {
                    stack.push_back(std::string{c});
                } else {
                    std::string sub_str = "";
                    while(!stack.empty() && stack.back() != "[") {
                        sub_str = stack.back() + sub_str;
                        stack.pop_back();
                    }
                    stack.pop_back();

                    std::string multi_plier = "";
                    while(!stack.empty()) {
                        if(is_numeric(stack.back())) {
                            multi_plier = stack.back() + multi_plier;
                            stack.pop_back();
                        } else {
                            break;
                        }
                    }

                    std::string str_push_stack = "";
                    for(int i = 0; i < atoi(multi_plier.c_str()); ++i) {
                        str_push_stack += sub_str;
                    }
                    stack.push_back(str_push_stack);
                }
            }

            //show(stack);
            std::string result = "";
            for(auto& s: stack){
                result += s;
            }

            return result;
        }
};

int main(void) {
    std::string case_1 = "3[a]2[bc]";
    // => aaabcbc
    std::string case_2 = "3[a2[c]]";
    // => accaccacc

    SolutionAnsRust s_ans_rs;
    std::cout << s_ans_rs.decodeString(case_1) << std::endl;
    std::cout << s_ans_rs.decodeString(case_2) << std::endl;

    std::cout << s_ans_rs.decodeString2(case_1) << std::endl;
    std::cout << s_ans_rs.decodeString2(case_2) << std::endl;
}
