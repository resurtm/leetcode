from typing import List


class Solution:
    def arrayPairSum(self, nums: List[int]) -> int:
        nums.sort()
        x = 0
        for i in range(0, len(nums), 2):
            x += min(nums[i], nums[i + 1])
        return x
