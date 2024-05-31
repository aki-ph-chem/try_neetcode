#include <iostream>
#include <stack>
#include <string>
#include <algorithm>
#include <vector>

class SolutionAns {
public:
    std::string removeKdigits(std::string num, int k) {
      int n = num.size();
      
      std::stack<char>s;
      int count = k;
      
      for(int i = 0 ; i < n; i++)
      {
        while(!s.empty() && count > 0 && s.top() > num[i])
        {
          s.pop();
          count--;
        }
        s.push(num[i]);
      }
      
      // In case the num was already in a non increasing order (e.x: 123456)
      while(s.size() != n - k) s.pop();
     
      std::string res = "";
      while(!s.empty())
      {
        res += s.top();
        s.pop();
      }
      std::reverse(res.begin() , res.end());
      // Remove the zeros from the left if they exist.
      while (res[0] == '0') res.erase(0 , 1);
    
      
      return (res == "") ? "0": res;
    }
};

// AC
class SolutionAnsPython {
public:
    std::string removeKdigits(std::string num, int k) {
      std::vector<char> s;
      
      for(auto& c: num) {
          while(!s.empty() && k > 0 && s.back() > c) {
              s.pop_back();
              --k;
          }

          if(!s.empty() || c != '0') {
              s.push_back(c);
          }
      }
      while(!s.empty() && k != 0) {
          s.pop_back();
          --k;
      }

      if(s.empty()) {
          return "0";
      }

      std::string result = "";
      for(auto&c: s) {
          result.push_back(c);
      }

      return result;
    }
};

int main(void) {
    using Case = std::pair<std::string, int>;
    Case case_1 = {"1432219", 3};
    // => "1219"
    Case case_2 = {"10200", 1};
    // => "200"
    Case case_3 = {"10", 2};
    // => "0"

    SolutionAns s_ans;

    /*
    std::cout << s_ans.removeKdigits(case_1.first, case_1.second) << std::endl;
    std::cout << s_ans.removeKdigits(case_2.first, case_2.second) << std::endl;
    std::cout << s_ans.removeKdigits(case_3.first, case_3.second) << std::endl;
    */

    SolutionAnsPython s_ans_py;

    std::cout << s_ans_py.removeKdigits(case_1.first, case_1.second) << std::endl;
    std::cout << s_ans_py.removeKdigits(case_2.first, case_2.second) << std::endl;
    std::cout << s_ans_py.removeKdigits(case_3.first, case_3.second) << std::endl;
}
