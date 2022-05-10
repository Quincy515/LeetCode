class Solution {
public:
    vector<vector<int>> result;
    vector<vector<int>> combinationSum3(int k, int n) {
        vector<int> path;
        backtrack(k, n, 1, 0, path);
        return result;
    }
    void backtrack(int k, int n, int step, int sum, vector<int> &path) {
        if (sum == n && path.size() == k) {
            result.push_back(path);
            return;
        }
        if (sum > n || path.size() > k || step > 9) {
            return;
        }
        backtrack(k, n, step + 1, sum, path);
        path.push_back(step);
        backtrack(k, n, step + 1, sum + step, path);
        path.pop_back();
    }
};