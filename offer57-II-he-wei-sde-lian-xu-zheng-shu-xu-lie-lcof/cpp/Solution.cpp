class Solution {
public:
    vector<vector<int>> findContinuousSequence(int target) {
        vector<vector<int>> res;
        int i = 1;
        int j = 2;
        int sum = 3;
        while (i != j) {
            if (sum == target) {
                vector<int> tmp;
                for (int k = i; k <= j; ++k) {
                    tmp.push_back(k);
                }
                res.push_back(tmp);
                sum -= i;
                i++;
                j++;
                sum += j;
            } else if (sum > target) {
                sum -= i;
                i++;
            } else {
                j++;
                sum += j;
            }
        }
        return res;
    }
};