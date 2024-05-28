#include <complex>
#include <iostream>
#include <vector>
#include <stack>

// 間に合ってしまったのだが...
class StockSpanner {
    private:
        std::vector<int> stack;
    public:
        StockSpanner() {
        }

        int next(int price) {
            stack.push_back(price);
            int result = 0;
            for(int i = stack.size() - 1; i >=0; --i) {
                if(stack[i] > price) {
                    break;
                }
                ++result;
            }

            return result;
        }
};

// 模範解答
class SolutionAns {
    public:
        std::stack<std::pair<int, int>> st;
        std::pair<int, int> pr;

        SolutionAns() {
            pr = {0, 0};
        }

        int next(int price) {

            int ret = 1;
            while (!st.empty() && price >= pr.first)
            {
                ret += pr.second;
                st.pop();
                if (!st.empty())
                    pr = st.top();
            }
            st.push(std::make_pair(price, ret));
            pr = st.top();
            return (ret);
        }
};

// Rustの模範解答より
// AC
class SolutionAnsRust {
    public:
        // {price, span}
        std::vector<std::pair<int, int>> stack;

        SolutionAnsRust() {
        }

        int next(int price) {
            int span = 1;

            while(!stack.empty() && stack.back().first <= price) {
                span += stack.back().second;
                stack.pop_back();
            }
            stack.push_back({price, span});

            return span;
        }
};

int main(void) {
    StockSpanner stock_sp;
    std::cout << "100 " << stock_sp.next(100) << std::endl;
    // 1
    std::cout << "80 " << stock_sp.next(80) << std::endl;
    // 1
    std::cout << "60 " << stock_sp.next(60) << std::endl;
    // 1
    std::cout << "70 " << stock_sp.next(70) << std::endl;
    // 2
    std::cout << "60 " << stock_sp.next(60) << std::endl;
    // 1
    std::cout << "75 " << stock_sp.next(75) << std::endl;
    // 4
    std::cout << "85 " << stock_sp.next(85) << std::endl;
    // 6

    SolutionAns stock_sp_ans;
    std::cout << "100 " << stock_sp_ans.next(100) << std::endl;
    // 1
    std::cout << "80 " << stock_sp_ans.next(80) << std::endl;
    // 1
    std::cout << "60 " << stock_sp_ans.next(60) << std::endl;
    // 1
    std::cout << "70 " << stock_sp_ans.next(70) << std::endl;
    // 2
    std::cout << "60 " << stock_sp_ans.next(60) << std::endl;
    // 1
    std::cout << "75 " << stock_sp_ans.next(75) << std::endl;
    // 4
    std::cout << "85 " << stock_sp_ans.next(85) << std::endl;
    // 6

    SolutionAnsRust stock_sp_ans_rs;
    std::cout << "100 " << stock_sp_ans_rs.next(100) << std::endl;
    // 1
    std::cout << "80 " << stock_sp_ans_rs.next(80) << std::endl;
    // 1
    std::cout << "60 " << stock_sp_ans_rs.next(60) << std::endl;
    // 1
    std::cout << "70 " << stock_sp_ans_rs.next(70) << std::endl;
    // 2
    std::cout << "60 " << stock_sp_ans_rs.next(60) << std::endl;
    // 1
    std::cout << "75 " << stock_sp_ans_rs.next(75) << std::endl;
    // 4
    std::cout << "85 " << stock_sp_ans_rs.next(85) << std::endl;
    // 6
}
