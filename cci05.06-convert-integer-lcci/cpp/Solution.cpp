class Solution {
public:
    int convertInteger(int A, int B) {
        int C = A ^ B;
        int count = 0;
        for (int i = 0; i < 32; ++i) {
            if ((C & 1) == 1) {
                count++;
            }
            C >>= 1;
        }
        return count;
    }
};
