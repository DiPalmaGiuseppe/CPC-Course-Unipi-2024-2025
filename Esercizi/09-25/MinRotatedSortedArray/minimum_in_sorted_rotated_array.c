// https://leetcode.com/problems/find-minimum-in-rotated-sorted-array/

int findMin(int* nums, int numsSize) {
    int left = 0, right = numsSize-1,mid;
    while(left<right){
        mid = left + (right - left) / 2;
        if(nums[mid] <= nums[left] && nums[mid] < nums[right])
            right = mid;
        else if(nums[mid] >= nums[left] && nums[mid] > nums[right])
            left = mid+1;
        else if(nums[mid] > nums[left] && nums[mid] < nums[right])
            return nums[left];
    }
    mid = left + (right - left) / 2;
    return nums[mid];
}