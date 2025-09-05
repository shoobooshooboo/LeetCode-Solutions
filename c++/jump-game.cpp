#include <algorithm>
#include <vector>
using std::vector;

class Solution {
public:
    bool canJump(vector<int>& nums) {
        int i = 0;
        for(int max = 0; i < nums.size() && i <= max; i++)
            max = std::max(nums[i] + i, max);

        return i == nums.size();
    }
};