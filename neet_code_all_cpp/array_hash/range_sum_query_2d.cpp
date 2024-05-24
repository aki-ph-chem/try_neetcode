#include <iostream>
#include <vector>

// AC
class NumMatrix {
    private:
        std::vector<std::vector<int>> prefix_sum;
    public:
        NumMatrix(std::vector<std::vector<int>>& matrix) {
            int m = matrix.size();
            int n = matrix[0].size();
            prefix_sum = std::vector(m, std::vector(n + 1, 0));

            for(int i_m = 0; i_m < m; ++i_m) {
                for(int i_n = 0; i_n < n; ++i_n) {
                    prefix_sum[i_m][i_n + 1] = prefix_sum[i_m][i_n] + matrix[i_m][i_n];
                }
            }
        }

        int sumRegion(int row1, int col1, int row2, int col2) {
            int sum = 0;
            for(int i = row1; i <= row2; ++i) {
                sum += prefix_sum[i][col2 + 1] - prefix_sum[i][col1];
            }

            return sum;
        }

        void debug(void) {
            for(auto& v: prefix_sum) {
                for(auto& w: v) {
                    std::cout<< w << " ";
                }
                std::cout << '\n';
            }
        }
};

// Pythonの模範解答
// AC
class NumMatrixPy {
    private:
        std::vector<std::vector<int>> prefix_sum;

    public:
        NumMatrixPy(std::vector<std::vector<int>>& matrix) {
            int m = matrix.size(), n = matrix[0].size();
            prefix_sum = std::vector(m + 1, std::vector(n + 1, 0));

            for(int i = 0; i < m; ++i) {
                int prev = 0;
                for(int j = 0; j < n; ++j){
                    prev += matrix[i][j];
                    prefix_sum[i + 1][j + 1] = prefix_sum[i][j + 1] + prev;
                }
            }
        }

        int sumRegion(int row1, int col1, int row2, int col2) {
            int sum_col2 = prefix_sum[row2 + 1][col2 + 1] 
                - prefix_sum[row1][col2 + 1];
            int sum_col1 = prefix_sum[row2 + 1][col1] 
                - prefix_sum[row1][col1];

            return sum_col2 - sum_col1;
        }
};

// C++の模範解答
class NumMatrixAns {
public:

    std::vector<std::vector<int>> dp;

    NumMatrixAns(std::vector<std::vector<int>>& matrix) {
        int r = matrix.size(),c = matrix[0].size();
        dp = matrix;

        for(int i =0;i<r;i++){
            for(int j =0;j<c;j++){
                if(i>0) dp[i][j] += dp[i-1][j]; // add prev row
                if(j>0) dp[i][j] += dp[i][j-1]; // add prev col

                // remove diagonal as it is added twice above
                if(i>0&&j>0) dp[i][j] -= dp[i-1][j-1];
            }
        }
    }
    
    int sumRegion(int row1, int col1, int row2, int col2) {
        int answer = dp[row2][col2];

        if(row1>0) answer -= dp[row1-1][col2];  // remove prev row on col1
        if(col1>0) answer -= dp[row2][col1-1]; // remo prev col on row2

        // add prev diagonal as pre row and prev col both contains that value
        if(row1>0&&col1>0) answer += dp[row1-1][col1-1];
        return answer;
    }
};

int main(void) {
    std::vector<std::vector<int>> case_1 = {
        {3, 0, 1, 4, 2},
        {5, 6, 3, 2, 1},
        {1, 2, 0, 1, 5},
        {4, 1, 0, 1, 7},
        {1, 0, 3, 0, 5},
    };
    std::vector<std::vector<int>> case_1_query = {
        {2, 1, 4, 3}, // => 8
        {1, 1, 2, 2}, // => 11
        {1, 2, 2, 4}, // => 12
    };


    NumMatrix n_mat(case_1);
    //n_mat.debug();
    std::cout << n_mat.sumRegion(case_1_query[0][0], case_1_query[0][1], case_1_query[0][2], case_1_query[0][3]) << std::endl;
    std::cout << n_mat.sumRegion(case_1_query[1][0], case_1_query[1][1], case_1_query[1][2], case_1_query[1][3]) << std::endl;
    std::cout << n_mat.sumRegion(case_1_query[2][0], case_1_query[2][1], case_1_query[2][2], case_1_query[2][3]) << std::endl;

    NumMatrixPy n_mat_py(case_1);
    std::cout << n_mat_py.sumRegion(case_1_query[0][0], case_1_query[0][1], case_1_query[0][2], case_1_query[0][3]) << std::endl;
    std::cout << n_mat_py.sumRegion(case_1_query[1][0], case_1_query[1][1], case_1_query[1][2], case_1_query[1][3]) << std::endl;
    std::cout << n_mat_py.sumRegion(case_1_query[2][0], case_1_query[2][1], case_1_query[2][2], case_1_query[2][3]) << std::endl;

    NumMatrixAns n_mat_ans(case_1);
    std::cout << n_mat_ans.sumRegion(case_1_query[0][0], case_1_query[0][1], case_1_query[0][2], case_1_query[0][3]) << std::endl;
    std::cout << n_mat_ans.sumRegion(case_1_query[1][0], case_1_query[1][1], case_1_query[1][2], case_1_query[1][3]) << std::endl;
    std::cout << n_mat_ans.sumRegion(case_1_query[2][0], case_1_query[2][1], case_1_query[2][2], case_1_query[2][3]) << std::endl;
}
