class Solution {
public:
    int hammingWeight(uint32_t n) {
        int oneCount = 0;
        long mask = 1;
        for (int i = 0; i < 32; ++i) {
            if ((n & mask) != 0) {
                oneCount++;
            }
            mask <<= 1;
        }
        return oneCount;
    }
};

class Solution {
public:
    int hammingWeight(uint32_t n) {
        int oneCount = 0;
        int i = 1;
        while (n != 0) {
            if ((n & 1) == 1) {
                oneCount++;
            }
            n >>= 1;
        }
        return oneCount;
    }
};
