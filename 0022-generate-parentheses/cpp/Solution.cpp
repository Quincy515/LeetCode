class Solution {
public:
    vector<string> result;
    vector<string> generateParenthesis(int n) {
        vector<char> path(2 * n);
        backtrack(n, 0, 0, 0 ,path);
        return result;
    }
    void backtrack(int n, int leftUsed, int rightUsed, int k, vector<char>& path) {
        if (k == 2 * n) {
            string str(path.begin(), path.end());
            result.push_back(str);
            return;
        }
        if (leftUsed < n) {
            path[k] = '(';
            backtrack(n, leftUsed + 1, rightUsed, k + 1, path);
        }
        if (leftUsed > rightUsed) {
            path[k] = ')';
            backtrack(n, leftUsed, rightUsed + 1, k + 1, path);
        }
    }
};