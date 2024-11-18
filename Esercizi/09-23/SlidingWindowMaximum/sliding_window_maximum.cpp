// https://leetcode.com/problems/sliding-window-maximum/

class Solution {
public:
    vector<int> maxSlidingWindow(vector<int>& nums, int k) {
        vector<int> res; 
        deque<int> d;
        for(int i=0; i<nums.size(); i++){

            while (!d.empty() && nums[i] > d.back()) {
                d.pop_back();
            }
            
            d.push_back(nums[i]);

            if (i >= k && nums[i - k] == d.front()) {
                d.pop_front();
            }
            
            if (i >= k - 1) {
                res.push_back(d.front());
            }
        }
        return res;
    }
};

