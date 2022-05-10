class Solution {
public:
    vector<vector<int>> result;
    vector<vector<int>> combine(int n, int k) {
        vector<int> path;
        backtrack(n, k, 1, path);
        return result;
    }
    void backtrack(int n, int k, int step, vector<int>& path) {
        if (path.size() == k) {
            result.push_back(path);
            return;
        }
        if (step == n + 1) {
            return;
        }
        backtrack(n, k, step + 1, path);
        path.push_back(step);
        backtrack(n, k, step + 1, path);
        path.pop_back();
    }
};