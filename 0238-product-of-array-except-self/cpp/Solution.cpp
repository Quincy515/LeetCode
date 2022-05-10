class Solution {
public:
    vector<int> productExceptSelf(vector<int>& nums) {
        int n = nums.size();
        vector<int> leftProduct(n);
        vector<int> rightProduct(n);
        int product = 1;
        for (int i = 0; i < n; ++i) {
            product *= nums[i];
            leftProduct[i] = product;
        }
        product = 1;
        for (int i = n - 1; i >= 0; --i) {
            product *= nums[i];
            rightProduct[i] = product;
        }

        vector<int> result(n);
        for (int i = 0; i < n; ++i) {
            result[i] = 1;
            if (i - 1 >= 0) {
                result[i] *= leftProduct[i - 1];
            }
            if (i + 1 < n) {
                result[i] *= rightProduct[i + 1];
            }
        }
        return result;
    }
};

class Solution {
public:
    vector<int> productExceptSelf(vector<int>& nums) {
        int n = nums.size();
        vector<int> result(n);
        int leftProduct = 1;
        for (int i = 0; i < n; ++i) {
            result[i] = leftProduct;
            leftProduct *= nums[i];
        }
        int rightProduct = 1;
        for (int i = n - 1; i >= 0; --i) {
            result[i] *= rightProduct;
            rightProduct *= nums[i];
        }
        return result;
    }
};
