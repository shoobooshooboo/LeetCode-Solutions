#include <vector>
using std::vector;
class Solution {
public:
    int removeElement(vector<int>& nums, int val) {
        int j = 0;
        int size = nums.size();
        for(int i = 0; i < size; i++){
            if(nums[i] != val){
                nums[j] = nums[i];
                j++;
            }
        }
        return j;
    }
};