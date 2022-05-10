class Solution {
public:
    int maxProfit(vector<int>& prices) {
        int n = prices.size();
        vector<int> max(n);
        int curMax = 0;
        for (int i = n - 1; i >= 0; --i) {
            max[i] = curMax;
            if (prices[i] > curMax) {
                curMax = prices[i];
            }
        }
        int result = 0;
        for (int i = 0; i < n; ++i) {
            if (result < max[i] - prices[i]) {
                result = max[i] - prices[i];
            }
        }
        return result;
    }
};
