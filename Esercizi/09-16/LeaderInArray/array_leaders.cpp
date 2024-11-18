// https://www.geeksforgeeks.org/problems/leaders-in-an-array-1587115620/1

#include <vector.h>

class Solution {
    // Function to find the leaders in the array.
  public:
    vector<int> leaders(vector<int>& arr) {
        vector<int> ldrs;
        int max = arr.back();
        for (auto el = arr.rbegin(); el != arr.rend(); el++) { 
            if(*el >= max){
                ldrs.push_back(*el);
                max = *el;
            }
        }
        
        reverse(ldrs.begin(), ldrs.end());
        
        return ldrs;
    }
};