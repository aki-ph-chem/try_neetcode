#include <iostream>
#include <vector>

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

void show_tree_depth(TreeNode* root) {
    if(!root) {
        return;
    }

    std::cout << root->val << std::endl;;

    // left
    show_tree_depth(root->left);
    // right
    show_tree_depth(root->right);
}

// 解けなかった
class Solution {
    public:
        TreeNode* buildTree(std::vector<int>& preorder, std::vector<int>& inorder) {
        }
};

// 模範解答
class SolutionAns {
    public:
        TreeNode* buildTree(std::vector<int>& preorder, std::vector<int>& inorder) {
            int index = 0;
            return build(preorder, inorder, index, 0, inorder.size() - 1);
        }

    private:
        TreeNode* build(std::vector<int>& preorder, std::vector<int>& inorder, int& index, int i, int j) {
            if (i > j) {
                return NULL;
            }

            TreeNode* root = new TreeNode(preorder[index]);

            int split = 0;
            for (int i = 0; i < inorder.size(); i++) {
                if (preorder[index] == inorder[i]) {
                    split = i;
                    break;
                }
            }
            index++;

            root->left = build(preorder, inorder, index, i, split - 1);
            root->right = build(preorder, inorder, index, split + 1, j);

            return root;
        }
};

int main(void) {
    // 行きがけ
    auto pre_1 = std::vector{3,9,20,15,7};
    // 帰りがけ
    auto in_1 = std::vector{9,3,15,20,7};

    auto pre_2 = std::vector{-1};
    auto in_2 = std::vector{-1};

    SolutionAns s_ans;

    auto res_1 = s_ans.buildTree(pre_1, in_1);
    auto res_2 = s_ans.buildTree(pre_2, in_2);

    std::cout << "res_1" << std::endl;
    show_tree_depth(res_1);

    std::cout << "res_2" << std::endl;
    show_tree_depth(res_2);
}
