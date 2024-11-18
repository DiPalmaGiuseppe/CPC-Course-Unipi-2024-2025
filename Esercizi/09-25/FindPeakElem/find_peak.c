// https://leetcode.com/problems/find-peak-element/

int findPeakElement(int* nums, int numsSize) {
    int left = 0, right = numsSize-1, mid;
    if(numsSize==1)
        return 0;
    while (left<=right){
        mid = left + (right - left) / 2;

        if(mid > 0 && mid < numsSize-1){
            if(nums[mid] > nums[mid-1] && nums[mid] > nums[mid+1])
                return mid;
            if(nums[mid] < nums[mid-1] && nums[mid] > nums[mid+1])
                right = mid - 1;
            else 
                left = mid + 1;
        }else{
            if(mid==0){
                if(nums[0]>nums[1])
                    return 0;
                return 1;
            }
            if(mid == numsSize-1){
                if(nums[numsSize-1]>nums[numsSize-2])
                    return numsSize-1;
                return numsSize-2;
            }
        }
    }
    return mid;
}