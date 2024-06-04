#include <cwchar>
#include <iostream>
#include <vector>
#include <string>

// Pythonの模範解答より
class SolutionAnsPython {
    public:
        // AC 
        std::vector<std::string> fullJustify(std::vector<std::string>& words, int maxWidth) {
            std::vector<std::string> result{};
            std::vector<std::string> line{};
            int length = 0;
            int i = 0;

            while(i < words.size()) {
                if(length + line.size() + words[i].size() > maxWidth) {
                    auto extra_space = maxWidth - length;
                    auto word_cnt = line.size() - 1;
                    auto spaces = extra_space / std::max((std::size_t)1, word_cnt);
                    auto remainder = extra_space % std::max((std::size_t)1, word_cnt);

                    for(auto j = 0; j < std::max((size_t)1, line.size() - 1); ++j) {
                        for(int i = 0; i < spaces; ++i) {
                            line[j] += " ";
                        }
                        if(remainder) {
                            line[j] += " ";
                            --remainder;
                        }
                    }

                    std::string tmp_str = "";
                    for(auto&s: line) {
                        tmp_str += s;
                    }
                    result.push_back(tmp_str);

                    line.clear();
                    length = 0;
                }

                line.push_back(words[i]);
                length += words[i].size();
                ++i;
            }

            std::string last_line = line[0];
            for(auto itr = ++(line.begin()); itr != line.end(); ++itr) {
                last_line += (" " + *itr); 
            }

            auto tail_spaces = maxWidth - last_line.size();
            for(int i = 0; i < tail_spaces; ++i) {
                last_line += " ";
            }
            result.push_back(last_line);

            return result;
        }
};

void show_result(std::vector<std::string>& result) {
    for(auto&s: result) {
        std::cout << s << "," << '\n';
    }
}

int main(void) {
    using Case = std::pair<std::vector<std::string>, int>;
    Case case_1 = {
        {
            "This",
            "is",
            "an",
            "example",
            "of",
            "text",
            "justification.",
        },
        16,
    };

    SolutionAnsPython s_ans_py;
    auto res_1 = s_ans_py.fullJustify(case_1.first, case_1.second);

    show_result(res_1);
}
