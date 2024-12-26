class Solution:
    def pivotIndex(self, nums: List[int]) -> int:

        r = sum(nums) - nums[0]
        l = 0
        for idx in range(0, len(nums)):
            num = nums[idx]
            if l == r:
                return idx

            l += num

            r -= nums[idx + 1] if idx != len(nums) - 1 else 0
        return -1

