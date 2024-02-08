#include <iostream>
#include <vector>

// 模範解答
// Union-Findを使う
class SolutionAns {
    public:
        std::vector<int> findRedundantConnection(std::vector<std::vector<int>>& edges) {
            auto n = (int)edges.size();

            std::vector<int> parents;
            std::vector<int> ranks;

            for(int i = 0; i <= n; ++i) {
                parents.push_back(i);
                ranks.push_back(i);
            } 

            std::vector<int> result;
            for(auto& v: edges) {
                auto [n_1, n_2] = std::pair(v[0], v[1]);

                if(!doUnion(parents, ranks, n_1, n_2)) {
                    result = {n_1, n_2};
                    break;
                }
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

        bool doUnion(std::vector<int>& parents, std::vector<int>& ranks, int n_1, int n_2) {
            auto [p_1, p_2] = std::pair(doFind(parents, n_1), doFind(parents, n_2));

            if(p_1 == p_2) {
                return false;
            }

            if(ranks[p_1] > ranks[p_2]) {
                parents[p_2] = p_1;
                ranks[p_1] += ranks[p_2];
            } else {
                parents[p_1] = p_2;
                ranks[p_2] += ranks[p_1];
            }

            return true;
        }
};

// AC
// Union-Findの実装が少し違う
class SolutionRe {
    public:
        std::vector<int> findRedundantConnection(std::vector<std::vector<int>>& edges) {
            auto n = (int)edges.size();

            std::vector<int> parents(n + 1, -1);
            std::vector<int> size_graph(n + 1, 1);
            std::vector<int> result;

            for(auto& v: edges) {
                auto [n_1, n_2] = std::pair(v[0], v[1]);

                if(!unite(n_1, n_2, parents, size_graph)) {
                    result = {n_1, n_2};
                    break;
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
                std::vector<int>& size) {

            x = root(x, parent);
            y = root(y, parent);

            if(x == y) {
                return false;
            }

            if(size[x] < size[y]) {
                std::swap(x, y);
            }

            parent[y] = x;
            size[x] += size[y];

            return true;
        }
};

void show_result(const std::vector<int>& result) {
    for(auto& v: result){
        std::cout << v << " ";
    }
    std::cout << std::endl;
}

int main(void) {
    auto case_1 = std::vector{std::vector{1, 2}, std::vector{1, 3}, std::vector{2, 3}};
    // => [2, 3]
    auto case_2 = std::vector{std::vector{1, 2}, std::vector{2, 3}, std::vector{3, 4}, std::vector{1, 4}, std::vector{1, 5}};
    // => [1, 4]

    SolutionAns s_ans;
    auto res_1 = s_ans.findRedundantConnection(case_1);
    auto res_2 = s_ans.findRedundantConnection(case_2);

    show_result(res_1);
    show_result(res_2);

    SolutionRe s_re;
    auto res_1_re = s_re.findRedundantConnection(case_1);
    auto res_2_re = s_re.findRedundantConnection(case_2);

    show_result(res_1_re);
    show_result(res_2_re);
}
