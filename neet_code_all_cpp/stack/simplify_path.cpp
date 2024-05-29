#include <iostream>
#include <string>
#include <vector>

template<class T>
void show(std::vector<T>& res) {
    for(auto& v: res) {
        std::cout << v << " ";
    }
    std::cout << std::endl;
}

template<class T>
void show_d(std::vector<T>& res, char delimiter) {
    for(auto& v: res) {
        std::cout << v << delimiter;
    }
    std::cout << std::endl;
}

// AC
class SolutionAns2 {
    public:
        std::string simlifyPath(std::string& path) {
            std::vector<std::string> stack;
            auto tmp = split_by(std::move(path));
            //show_d(tmp, ',');
            for(auto&s : tmp) {
                if(s == "..") {
                    if(!stack.empty()) {
                        stack.pop_back();
                    }
                } else if(s == "." || s == "") {
                    continue;
                } else {
                    stack.push_back(s);
                } 
            }

            std::string result = "";
            for(auto&s : stack) {
                result.push_back('/');
                result += s;
            }
            if(result.empty()) {
                result.push_back('/');
            }

            return result;
        }

        std::vector<std::string> split_by(std::string&& src) {
            std::vector<std::string> result;

            int i_1 = 0;
            while(i_1 < src.size()) {
                auto tmp = std::string{""};
                if(src[i_1] == '/') {
                    int i_2 = i_1 + 1;
                    while(i_2 < src.size() && src[i_2] != '/') {
                        tmp.push_back(src[i_2]);
                        ++i_2;
                    }
                    result.push_back(tmp);
                    i_1 = i_2 - 1;
                }

                ++i_1;
            }

            return result;
        }
};

int main(void) {
    std::string case_1 = "/home/";
    std::string case_2 = "/home//foo";
    std::string case_3 = "/../";
    std::string case_4 = "/.../a/../b/c/../d/./";

    SolutionAns2 s_ans_2;
    std::cout << s_ans_2.simlifyPath(case_1) << std::endl;
    std::cout << s_ans_2.simlifyPath(case_2) << std::endl;
    std::cout << s_ans_2.simlifyPath(case_3) << std::endl;
    std::cout << s_ans_2.simlifyPath(case_4) << std::endl;
}
