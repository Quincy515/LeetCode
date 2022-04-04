class Solution {
public:
    long res = LONG_MAX;
    int smallestDifference(vector<int>& a, vector<int>& b) {
        std::sort(a.begin(), a.end());
        std::sort(b.begin(), b.end());
        int n = a.size();
        int m = b.size();
        int i = 0;
        int j = 0;
        while (i < n && j < m) {
            if (a[i] >= b[j]) {
                res = std::min(res, (long)a[i] - b[j]);
                j++;
            } else {
                res = std::min(res, (long)b[j] - a[i]);
                i++;
            }
        }
        return (int)res;
    }
};