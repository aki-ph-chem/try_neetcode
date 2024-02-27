#include <iostream>
#include <sstream>
#include <string>
#include <queue>
#include <strstream>
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

// 解けなかった
class Codec {
    public:
        std::string serialize(const TreeNode* root) {
            std::string result;

            if(!root) {
                return result;
            }

            //BFS
            std::queue<const TreeNode*> q;
            q.push(root);
            // for "null"
            std::queue<std::string> null_buf;

            while(!q.empty()) {
                auto count = (int)q.size();

                for(int i = 0; i < count; ++i) {
                    auto node = q.front();
                    q.pop();

                    result += "," + std::to_string(node->val);
                    if(!null_buf.empty()) {
                        result += null_buf.front();
                        null_buf.pop();
                    }

                    if(node->left) {
                        q.push(node->left);
                    } else {
                        null_buf.push(",null");
                    }

                    if(node->right) {
                        q.push(node->right);
                    } else {
                        null_buf.push(",null");
                    }
                }
            }

            return result;
        }

        TreeNode* deserialize(std::string data) {
            auto node = new TreeNode(3);
            return node;
        }
};

// 模範解答
class SolutionAns {
    public:
        std::string serialize(TreeNode* root) {
            std::ostrstream out;
            encode(root, out);
            return out.str();
        }

        TreeNode* deserialize(std::string data) {
            std::istringstream in(data);
            return decode(in);
        }

    private:
        void encode(TreeNode* root, std::ostrstream& out) {
            if(!root) {
                out << "N ";
                return;
            }

            out << root->val << " ";

            encode(root->left, out);
            encode(root->right, out);
        }

        TreeNode* decode(std::istringstream& in) {
            std::string value = "";
            in >> value;

            if(value == "N") {
                return nullptr;
            }

            auto root = new TreeNode(std::stoi(value));

            root->left = decode(in);
            root->right = decode(in);

            return root;
        }
};

void show_tree_dfs(const TreeNode* root) {
    if(!root) {
        return;
    }

    std::cout << root->val << std::endl;;

    // right
    show_tree_dfs(root->left);
    // left
    show_tree_dfs(root->right);
}

template<typename T>
void show_result(std::vector<std::vector<T>>& result) {
    for(auto& v: result) {
        for(auto& w: v) {
            std::cout << w << " ";
        }
        std::cout << '\n';
    }
}

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
    //show_tree_dfs(&root_1);

    /*
    Codec c_1;
    std::cout << c_1.serialize(&root_1) << std::endl;
    */

    SolutionAns s_ans;
    auto serialized_root_1 = s_ans.serialize(&root_1);
    std::cout << serialized_root_1 << std::endl;
    auto deserilize_serialized_root_1 = s_ans.deserialize(serialized_root_1);
    show_tree_dfs(deserilize_serialized_root_1);
}
