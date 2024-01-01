#include <iostream>

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
        bool isValidBST(TreeNode* root) {
            if(!root){
                return true;
            }
            auto val_p = root->val;

            if(root->left){
                return root->left->val < val_p && isValidBST(root->left);
            }
            if(root->right){
                return root->right->val > val_p && isValidBST(root->right);
            }

            return false;
        }
};

// 模範解答
class SolutionAns {
    public:
        bool isValidBST(TreeNode* root) {
            // Leet Code の環境ではマクロで定義されている
            constexpr auto LONG_MIN = -(1<<30); 
            constexpr auto LONG_MAX = (1<<30); 

            return helper(root, LONG_MIN, LONG_MAX);
        }

    private:
        bool helper(TreeNode* root, long left, long right){
            if (!root){
                return true;
            }

            // 左の子 < 親 < 右の子 
            if (root->val < right && root->val > left){
                return helper(root->left, left, root->val) && helper(root->right, root->val, right);
            }

            return false;
        }
};

int main(void) {
    TreeNode root_1(2);
    TreeNode root_1_1(1);
    TreeNode root_1_2(3);
    root_1.left = &root_1_1;
    root_1.right = &root_1_2;
    /*
       2
      / \
     /   \
    1     3

     */
    // => true

    TreeNode root_2(5);
    TreeNode root_2_1(1);
    TreeNode root_2_2(4);
    TreeNode root_2_3(3);
    TreeNode root_2_4(6);
    root_2.left = &root_2_1;
    root_2.right = &root_2_2;
    root_2_2.left = &root_2_3;
    root_2_2.right = &root_2_4;
    /*
       5
      / \
     /   \
    1     4
         / \
        /   \
       3     6

     */
    // => false

    Solution s_1;
    std::cout << s_1.isValidBST(&root_1) << std::endl;
    std::cout << s_1.isValidBST(&root_2) << std::endl;

    SolutionAns s_ans;
    std::cout << s_ans.isValidBST(&root_1) << std::endl;
    std::cout << s_ans.isValidBST(&root_2) << std::endl;
}
