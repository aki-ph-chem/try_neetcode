#include <iostream>
#include <vector>
#include <numeric>

// 模範解答
class SolutionAns {
public:
    std::vector<std::vector<int>> combinationSum(std::vector<int>& candidates, int target) {
        std::vector<int> curr;
        std::vector<std::vector<int>> res;
        helper(candidates, target, 0, curr, res);
        return res;
    }

private:
    void helper(
            std::vector<int>& cands, 
            int target, 
            int i, 
            std::vector<int>& curr,  
            std::vector<std::vector<int>>& res)
    {
        // iがcandsのサイズを超える、もしくはtarget < 0なら終了
        if (i >= cands.size() || target < 0) {
            return;
        }
        // target が0になったら終了
        if (target == 0) {
            res.push_back(curr);
            return;
        }

        // cand[i]を選ぶ場合
        // i番目をcurrにプッシュ
        curr.push_back(cands[i]);
        helper(cands, target - cands[i], i, curr, res);

        // cand[i]を選ばない場合
        // 末尾の要素を削除
        curr.pop_back();
        helper(cands, target, i + 1, curr, res);
    }
};

// Rustの模範解答より
class SolutionRustAns {
    public:
        std::vector<std::vector<int>> combinationSum(std::vector<int>& candidates, int target) {
            std::vector<int> current;
            std::vector<std::vector<int>> result;
            dfs(candidates, target, 0, result, current);

            return result;
        }

    private:
        void dfs(
                std::vector<int>& candidates,
                int target,
                int idx,
                std::vector<std::vector<int>>& result,
                std::vector<int>& current
                ) {
            // currentの和がtargetに等しければ終了
            auto sum = std::accumulate(current.begin(), current.end(), 0); 
            if(sum == target) {
                result.push_back(current);
                return;
                // sum がtargetの値を超えたら終了
            } else if(sum > target) {
                return;
            }

            // currentの更新
            for(int i = idx; i < candidates.size(); ++i) {
                current.push_back(candidates[i]);
                // i番目以上の要素を選ぶ
                dfs(candidates, target, i, result, current);
                current.pop_back();
            }
        }
};

int main(void) {
    auto case_1 = std::pair(std::vector{2, 3, 6, 7}, 7);
    // => {{2,2,3},{7}}
    auto case_2 = std::pair(std::vector{2, 3, 5}, 8);
    // => {{2,2,2,2}, {2,3,3}, {3,5}}
    auto case_3 = std::pair(std::vector{2}, 1);
    // => {}
    auto case_4 = std::pair(std::vector{2, 3, 6, 7}, 7);
    // => {{2, 3, 6, 7}, {7}}

    SolutionAns s_ans;
    auto res_1 = s_ans.combinationSum(case_1.first, case_1.second);
    auto res_2 = s_ans.combinationSum(case_2.first, case_2.second);
    auto res_3 = s_ans.combinationSum(case_3.first, case_3.second);
    auto res_4 = s_ans.combinationSum(case_4.first, case_4.second);

    std::cout << "case_1" << std::endl;
    for(const auto &a: res_1) {
        for(const auto &b: a){
            std::cout << b << " ";
        }
        std::cout << std::endl;
    }

    std::cout << "case_2" << std::endl;
    for(const auto &a: res_2) {
        for(const auto &b: a){
            std::cout << b << " ";
        }
        std::cout << std::endl;
    }

    std::cout << "case_3" << std::endl;
    for(const auto &a: res_3) {
        for(const auto &b: a){
            std::cout << b << " ";
        }
        std::cout << std::endl;
    }

    std::cout << "case_4" << std::endl;
    for(const auto &a: res_4) {
        for(const auto &b: a){
            std::cout << b << " ";
        }
        std::cout << std::endl;
    }

    SolutionRustAns s_rust_ans;
    auto res_rust_1 = s_rust_ans.combinationSum(case_1.first, case_1.second);
    auto res_rust_2 = s_rust_ans.combinationSum(case_2.first, case_2.second);
    auto res_rust_3 = s_rust_ans.combinationSum(case_3.first, case_3.second);
    auto res_rust_4 = s_rust_ans.combinationSum(case_4.first, case_4.second);

    std::cout << "case_1" << std::endl;
    for(const auto &a: res_1) {
        for(const auto &b: a){
            std::cout << b << " ";
        }
        std::cout << std::endl;
    }

    std::cout << "case_2" << std::endl;
    for(const auto &a: res_2) {
        for(const auto &b: a){
            std::cout << b << " ";
        }
        std::cout << std::endl;
    }

    std::cout << "case_3" << std::endl;
    for(const auto &a: res_3) {
        for(const auto &b: a){
            std::cout << b << " ";
        }
        std::cout << std::endl;
    }

    std::cout << "case_4" << std::endl;
    for(const auto &a: res_4) {
        for(const auto &b: a){
            std::cout << b << " ";
        }
        std::cout << std::endl;
    }
}
