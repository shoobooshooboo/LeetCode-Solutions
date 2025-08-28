#include <vector>
using std::vector;
class Solution {
public:
    void merge(vector<int>& nums1, int m, vector<int>& nums2, int n) {
        int i = 0, j = 0;
        vector<int> nums1Copy;
        while(i < m && j < n){
            if(nums1[i] < nums2[j]){
                nums1Copy.push_back(nums1[i]);
                i++;
            }else{
                nums1Copy.push_back(nums2[j]);
                j++;
            }
        }
        while(i < m){
            nums1Copy.push_back(nums1[i]);
            i++;
        }
        while(j < n){
            nums1Copy.push_back(nums2[j]);
            j++;
        }
        nums1 = nums1Copy;
    }
};