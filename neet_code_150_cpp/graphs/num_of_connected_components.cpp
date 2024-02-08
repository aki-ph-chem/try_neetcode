#include <iostream>
#include <pthread.h>
#include <utility>
#include <vector>

// AC(けんちょ本に同じ問題あった...)
class Solution {
    public:
        int countComponents(int n, std::vector<std::vector<int>>& edges) {
            std::vector<int> parent(n, -1);
            std::vector<int> size_graph(n, 1);

            for(auto& v: edges) {
                unite(v[0], v[1], parent, size_graph);
            }

            int result = 0;
            for(int i = 0; i < n; ++i) {
                if(root(i, parent) == i) {
                    ++result;
                }
            }

            return result;
        }

    private:
        // for Union-Find
        int root(int x, std::vector<int>& parents) {
            if(parents[x] == -1) {
                return x;
            } else {
                return parents[x] = root(parents[x], parents);
            }
        }

        bool unite(int x,
                int y,
                std::vector<int>& parent,
                std::vector<int>& size
                ) {

            x = root(x, parent); 
            y = root(y, parent);
            if(x == y) return false;

            if(size[x] < size[y]) std::swap(x, y);

            parent[y] = x;
            size[x] += size[y];

            return true;
        }
};

// 模範解答
class SolutionAns {
    public:
        int countComponents(int n, std::vector<std::vector<int>>& edges) {
            std::vector<int> parents;
            std::vector<int> ranks;
            for(int i = 0; i < n; ++i) {
                parents.push_back(i);
                ranks.push_back(1);
            }

            auto result = n;
            for(auto& v: edges) {
                result -= doUnion(parents, ranks, v[0], v[1]);
            }

            return result;
        }

    private:
        int doFind(std::vector<int>& parents, int n) {
            auto p = parents[n];
            while(p != parents[p]) {
                parents[p] = parents[parents[p]];
                p = parents[p];
            }

            return p;
        }

        int doUnion(std::vector<int>& parents, std::vector<int>& ranks, int n_1, int n_2) {
            auto p_1 = doFind(parents, n_1);
            auto p_2 = doFind(parents, n_2);

            if(p_1 == p_2) {
                return 0;
            }

            if(ranks[p_1] > ranks[p_2]) {
                parents[p_2] = p_1;
                ranks[p_1] += ranks[p_2];
            } else {
                parents[p_1] = p_2;
                ranks[p_2] += ranks[p_1];
            }

            return 1;
        }
};

int main(void) {
    std::pair<int, std::vector<std::vector<int>>> case_1 = {3, {{0, 1}, {0, 2}}};
    std::pair<int, std::vector<std::vector<int>>> case_2 = {6, {{0, 1}, {1, 2}, {2, 3}, {4, 5}}};

    Solution s_1;
    std::cout << s_1.countComponents(case_1.first, case_1.second) << std::endl;
    std::cout << s_1.countComponents(case_2.first, case_2.second) << std::endl;

    SolutionAns s_ans;
    std::cout << s_ans.countComponents(case_1.first, case_1.second) << std::endl;
    std::cout << s_ans.countComponents(case_2.first, case_2.second) << std::endl;
}
