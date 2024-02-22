#include <iostream>
#include <iterator>
#include <vector>
#include <queue>

// Tree
struct TreeNode {
    int val;
    TreeNode* left;
    TreeNode* right;

    TreeNode()
        :val(0), left(nullptr), right(nullptr) {}

    TreeNode(int x)
        :val(x), left(nullptr), right(nullptr) {}

    TreeNode(int x, TreeNode* left, TreeNode* right)
        :val(x), left(left), right(right) {}
};

// 解けなかった
class Solution {
    public:
        std::vector<int> rightSideView(const TreeNode* root) {
            std::vector<int> result;
            dfs(root, result);

            return result;
        }

    private:
        void dfs(const TreeNode* root, std::vector<int>& result) {
            if(!root){
                return;
            }

            result.push_back(root->val);
            dfs(root->right, result);
        }
};

// 模範解答 1
// BFSで解く
class SolutionAns {
    public:
        std::vector<int> rightSideView(TreeNode* root) {
            if(!root) {
                return {};
            }

            std::queue<TreeNode*> q;
            q.push(root);

            std::vector<int> result;

            while(!q.empty()) {
                int count = q.size();

                for(int i = count; i > 0; --i) {
                    auto node = q.front();
                    q.pop();

                    if(i == count) {
                        result.push_back(node->val);
                    }

                    if(node->right) {
                        q.push(node->right);
                    }
                    if(node->left) {
                        q.push(node->left);
                    }
                }
            }

            return result;
        }
};

// 模範解答2
// DFSで解く
class SolutionAns2 {
    public:
        std::vector<int> rightSideView(const TreeNode* root) {
            std::vector<int> result;
            dfs(root, result, 0);

            return result;
        }

    private:
        void dfs(const TreeNode* root, std::vector<int>& result, int lvl) {
            if(!root) {
                return;
            }

            if(lvl == result.size()) {
                result.push_back(root->val);
            }

            dfs(root->right, result, lvl + 1);
            dfs(root->left, result, lvl + 1);
        }
};


void show_tree_dfs(const TreeNode* root) {
    if(!root) {
        return;
    }

    std::cout << root->val << std::endl;;

    // left
    show_tree_dfs(root->left);
    // right
    show_tree_dfs(root->right);
}

void show_result(const std::vector<int>& result) {
    for(auto& v: result) {
        std::cout<< v << " ";
    }
    std::cout << std::endl;
}

int main(void) {
    auto root_1 = TreeNode(1);
    auto root_1_2 = TreeNode(2);
    auto root_1_3 = TreeNode(3);
    auto root_1_4 = TreeNode(5);
    auto root_1_5 = TreeNode(4);
    root_1.left = &root_1_2;
    root_1.right = &root_1_3;
    root_1_2.right = &root_1_4;
    root_1_3.left = &root_1_5;
    //
    //    1
    //   / \
    //  2   3
    //   \   \
    //    5   4
    //
    // => [1,3,4]
    show_tree_dfs(&root_1);
    Solution s_1;
    auto res_1 = s_1.rightSideView(&root_1);
    show_result(res_1);

    SolutionAns s_ans;
    auto res_ans = s_ans.rightSideView(&root_1);
    show_result(res_ans);

    SolutionAns2 s_ans_2;
    auto res_ans_2 = s_ans_2.rightSideView(&root_1);
    show_result(res_ans_2);
}
