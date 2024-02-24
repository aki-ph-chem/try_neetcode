#include <iostream>
#include <vector>
//#define DEBUG_PRINT

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
        int goodNodes(TreeNode* root) {
            if(!root) {
                return 0;
            }
            std::vector<int> max_nodes;
            //dfs(root, max_nodes, root->val);
            dfs_2(root, max_nodes, root->val);

            return (int)max_nodes.size();
        }

    private:
        void dfs(const TreeNode* root, std::vector<int>& max_nodes, int current_node) {
            if(!root) {
#ifdef DEBUG_PRINT
                std::cout << "cunnrent_node: "<< current_node << " is stored" << std::endl;
#endif
                max_nodes.push_back(current_node);
                return;
            }
#ifdef DEBUG_PRINT
            std::cout << "now at: " << root->val << std::endl;
#endif
            current_node = std::max(current_node, root->val);

            dfs(root->left, max_nodes, current_node);
            dfs(root->right, max_nodes, current_node);
        }

        void dfs_2(const TreeNode* root, std::vector<int>& max_nodes, int current_node) {
            if(!root) {
                return;
            } else if (!(root->left) && !(root->right)) {
#ifdef DEBUG_PRINT
                std::cout << "cunnrent_node: "<< current_node << " is stored" << std::endl;
#endif
                max_nodes.push_back(current_node);
                return;
            }

#ifdef DEBUG_PRINT
            std::cout << "now at: " << root->val << std::endl;
#endif
            current_node = std::max(current_node, root->val);

            dfs(root->left, max_nodes, current_node);
            dfs(root->right, max_nodes, current_node);
        }
};

// 模範解答
class SolutionAns {
    public:
        int goodNodes(TreeNode* root) {
            int result = 0;
            dfs(root, root->val, result);

            return result;
        }

        void dfs(TreeNode* root, int maxSoFar, int& result) {
            if(!root){
                return;
            }

            if(root->val >= maxSoFar) {
                ++result;
            }

            dfs(root->left, std::max(maxSoFar, root->val), result);
            dfs(root->right, std::max(maxSoFar, root->val), result);
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


int main(void) {
    TreeNode root_1(3);
    TreeNode root_1_1(1);
    TreeNode root_1_2(4);
    TreeNode root_1_3(3);
    TreeNode root_1_4(1);
    TreeNode root_1_5(5);
    root_1.left = &root_1_1;
    root_1.right = &root_1_2;
    root_1_1.left = &root_1_3;
    root_1_2.left = &root_1_4;
    root_1_2.right = &root_1_5;
    // => 4
    //show_tree_dfs(&root_1);

    Solution s_1;
    std::cout << s_1.goodNodes(&root_1) << std::endl;

    SolutionAns s_ans;
    std::cout << s_ans.goodNodes(&root_1) << std::endl;
}
