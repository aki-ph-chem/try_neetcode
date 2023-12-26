#include <cmath>
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
        bool isSamaTree(TreeNode* p, TreeNode* q) {
            if(!p && !q) {
                return true;
            } else if(!p || !q) {
                return false;
            }

            if(p->val != q->val) {
                return false;
            }

            return isSamaTree(p->left, q->left) || isSamaTree(p->right, q->right);
        }
};

// 模範解答
class SolutionAns {
    public:
        bool isSameTree(TreeNode* p, TreeNode* q) {
            if (p == NULL && q == NULL) {
                return true;
            }
            if (p == NULL || q == NULL) {
                return false;
            }
            if (p->val != q->val) {
                return false;
            }

            return isSameTree(p->left, q->left) && isSameTree(p->right, q->right);
        }
};

int main(void) {
    TreeNode p_1(1); 
    TreeNode p_1_1(2); 
    TreeNode p_1_2(3); 
    p_1.left = &p_1_1;
    p_1.right = &p_1_2;
    p_1_1.left = nullptr;
    p_1_1.right = nullptr;
    p_1_2.left = nullptr;
    p_1_2.right = nullptr;
    TreeNode q_1(1); 
    TreeNode q_1_1(2); 
    TreeNode q_1_2(3); 
    q_1.left = &p_1_1;
    q_1.right = &p_1_2;
    q_1_1.left = nullptr;
    q_1_1.right = nullptr;
    q_1_2.left = nullptr;
    q_1_2.right = nullptr;
    /*
        p        q 
    
        1         1 
       / \       / \
      /   \     /   \
     2     3   2     3

     => true 
   */

    TreeNode p_2(1);
    TreeNode p_2_1(2);
    p_2.left = &p_2_1;
    p_2.right = nullptr;
    p_2_1.left = nullptr;
    p_2_1.right = nullptr;
    TreeNode q_2(1);
    TreeNode q_2_1(2);
    q_2.left = nullptr;
    q_2.right = &q_2_1;
    q_2_1.left = nullptr;
    q_2_1.right = nullptr;
    /*
        p   q
      
        1   1 
       /     \
      /       \
     2         2
    
    => false
    */

    Solution s_1;
    std::cout << s_1.isSamaTree(&p_1, &q_1) << std::endl;
    std::cout << s_1.isSamaTree(&p_2, &q_2) << std::endl;

    Solution s_ans;
    std::cout << s_ans.isSamaTree(&p_1, &q_1) << std::endl;
    std::cout << s_ans.isSamaTree(&p_2, &q_2) << std::endl;
}
