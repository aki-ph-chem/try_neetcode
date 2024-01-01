#include <iostream>

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
        TreeNode* lowestCommonAncestor(TreeNode* rot, TreeNode* p, TreeNode* q) {
            if(!rot) {
                return nullptr;
            }

            lowestCommonAncestor(rot->left, p, q);
            lowestCommonAncestor(rot->right, p, q);
        }
};

// 模範解答
class SolutionAns {
    public:
        TreeNode* lowestCommonAncestor(TreeNode* root, TreeNode* p, TreeNode* q) {
            if (p->val < root->val && q->val < root->val) {
                return lowestCommonAncestor(root->left, p, q);
            } else if (p->val > root->val && q->val > root->val) {
                return lowestCommonAncestor(root->right, p, q);
            } else {
                return root;
            }
        }
};

int main(void) {
    auto root_1 = TreeNode(6);
    auto root_1_1 = TreeNode(2);
    auto root_1_2 = TreeNode(8);
    auto root_1_3 = TreeNode(0);
    auto root_1_4 = TreeNode(4);
    auto root_1_5 = TreeNode(7);
    auto root_1_6 = TreeNode(9);
    auto root_1_7 = TreeNode(3);
    auto root_1_8 = TreeNode(5);
    // 6--8
    // |
    // 2
    root_1.left = &root_1_1;
    root_1.right = &root_1_2;
    // 2--0
    // |
    // 4
    root_1_1.left = &root_1_3;
    root_1_1.right = &root_1_4;
    // 8--7
    // |
    // 9
    root_1_2.left = &root_1_5;
    root_1_2.right = &root_1_6;
    // 4--5
    // |
    // 3
    root_1_4.left = &root_1_7;
    root_1_4.right = &root_1_8;
    /*
          6
         / \
        /   \
       /     \
      2       8
     / \     / \
    /   \   /   \
   0     4 7     9
        / \
       /   \
      3     5

    */

    //show_tree_depth(&root_1);
    //(2, 8)
    auto case_1 = std::pair(&root_1_1, &root_1_2);
    // => 6
    
    // (2, 4)
    auto case_2 = std::pair(&root_1_1, &root_1_4);
    // => 2

    /*
    Solution s_1;
    std::cout << s_1.lowestCommonAncestor(&root_1, case_1.first, case_1.second) << std::endl;
    std::cout << s_1.lowestCommonAncestor(&root_1, case_2.first, case_2.second) << std::endl;
    */
    
    SolutionAns s_ans;
    std::cout << s_ans.lowestCommonAncestor(&root_1, case_1.first, case_1.second)->val << std::endl;
    std::cout << s_ans.lowestCommonAncestor(&root_1, case_2.first, case_2.second)->val << std::endl;
}
