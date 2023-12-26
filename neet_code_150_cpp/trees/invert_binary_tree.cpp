#include <iostream>
#include <vector>

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
        TreeNode* invertTree(TreeNode* root) {
            auto result = root; 

            if(root) {
                root->left = invertTree(root->left);
                root->right = invertTree(root->right);
            }

            return result;
        }
};

// 模範解答
class SolutionAns {
    public:
        TreeNode* invertTree(TreeNode* root) {
            if(!root) {
                return nullptr;
            }

            std::swap(root->left, root->right);
            invertTree(root->left);
            invertTree(root->right);

            return root;
        }
};

int main(void) {
    auto case_1 = std::vector{4, 2, 7, 1, 3, 6, 9};
    TreeNode tree_1_0(case_1[0]);
    TreeNode tree_1_1(case_1[1]);
    TreeNode tree_1_2(case_1[2]);
    TreeNode tree_1_3(case_1[3]);
    TreeNode tree_1_4(case_1[4]);
    TreeNode tree_1_5(case_1[5]);
    TreeNode tree_1_6(case_1[6]);

    tree_1_0.left = &tree_1_1;
    tree_1_1.left = &tree_1_3;
    tree_1_1.right = &tree_1_4;

    tree_1_0.right = &tree_1_2;
    tree_1_2.left = &tree_1_5;
    tree_1_2.right = &tree_1_6;

    auto case_2 = std::vector{2, 1, 3};
    TreeNode tree_2_0(case_2[0]);
    TreeNode tree_2_1(case_2[1]);
    TreeNode tree_2_2(case_2[2]);

    tree_2_0.left = &tree_2_1;
    tree_2_0.right = &tree_2_2;


    SolutionAns s_ans;

    // tree_1_0
    show_tree_depth(&tree_1_0);
    auto res_1 = s_ans.invertTree(&tree_1_0);
    show_tree_depth(res_1);

    // tree_1_0
    show_tree_depth(&tree_2_0);
    auto res_2 = s_ans.invertTree(&tree_2_0);
    show_tree_depth(res_2);
}
