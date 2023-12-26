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
// 解けなかった
class Solution {
    public:
        int diameterOfBinaryTree(TreeNode* root) {
            if(!root) {
                return 0;
            }

            return 1 + std::max(diameterOfBinaryTree(root->left),
                    diameterOfBinaryTree(root->right));
        }
};

// 模範解答
class SolutionAns {
    public:
        int diameterOfBinaryTree(TreeNode* root) {
            int result = 0;
            dfs(root, result);
            return result;
        }

    private:
        int dfs(TreeNode* root, int& result) {
            if (root == NULL) {
                return 0;
            }

            // left,rightをそれぞれ計算
            int left = dfs(root->left, result);
            int right = dfs(root->right, result);

            // 今までの値とleft + rightの値とresultを比較
            result = std::max(result, left + right);
            // left,rightの大きい方に1を足して返す
            return 1 + std::max(left, right);
        }
};

int main(void) {
    TreeNode root_1(1);
    TreeNode root_1_1(2);
    TreeNode root_1_2(3);
    TreeNode root_1_3(4);
    TreeNode root_1_4(5);
    root_1.left = &root_1_1;
    root_1.right = &root_1_2;
    root_1_2.left = &root_1_3;
    root_1_2.right = &root_1_4;
/*
     
          1 
         / \
        /   \
       2     3
      / \
     /   \
    4     5

    => 3
*/
    TreeNode root_2(1);
    TreeNode root_2_1(2);
    root_2.left = &root_2_1;
/*
       1
      /
     /
    2

    => 1
*/

    Solution s_1;
    std::cout << s_1.diameterOfBinaryTree(&root_1) << std::endl;
    std::cout << s_1.diameterOfBinaryTree(&root_2) << std::endl;

    SolutionAns s_ans;
    std::cout << s_ans.diameterOfBinaryTree(&root_1) << std::endl;
    std::cout << s_ans.diameterOfBinaryTree(&root_2) << std::endl;
}
