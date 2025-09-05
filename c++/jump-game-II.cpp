#include <algorithm>
#include <vector>
using std::vector;

class Solution {
public:
    int jump(vector<int>& nums) {
        int jumps = 0;
        for (int i = 0; i < nums.size() - 1; i++){
            jumps++;
            int distance = nums[i] + i;
            if (distance >= nums.size() - 1) break;
            int max = i;
            for(;i <= std::min(distance, (int)nums.size() - 1); i++)
                if (nums[max] + max < nums[i] + i)
                    max = i;
            i = max - 1;
        }
        return jumps;
    }
};