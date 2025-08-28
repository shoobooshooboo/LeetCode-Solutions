#include <vector>
using std::vector;
class Solution {
public:
    int removeDuplicates(vector<int>& nums) {
        int j = 1;
        for (int i = 1; i < nums.size(); i++){
            if (nums[i] == nums[i - 1]) continue;
            nums[j++] = nums[i];
        }
        return j;
    }
};