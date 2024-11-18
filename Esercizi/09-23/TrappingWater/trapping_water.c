// https://leetcode.com/problems/trapping-rain-water/description/

int trap(int *height, int heightSize) {

    int j = heightSize - 1;
    int i = 0;
    int left_ptr = i + 1;
    int right_ptr = j - 1;

    int water = 0;
    int diff = 0;

    while (i < j - 1) {
        if (height[i] <= height[j]) {
            diff = height[i] - height[left_ptr];
            if (diff > 0)
                water += diff;
            else
                i = left_ptr;
            left_ptr++;
        }
        else {
            diff = height[j] - height[right_ptr];
            if (diff > 0)
                water += diff;
            else
                j = right_ptr;
            right_ptr--;
        }
    }

    return water;
}