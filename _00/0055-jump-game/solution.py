class Solution:
    def canJump(self, nums: List[int]) -> bool:
        if len(nums) == 1:
            return True
        curr = len(nums) - 2
        dist = 1
        while curr != 0:
            if nums[curr] >= dist:
                curr -= 1
                dist = 1

            else:
                curr -= 1
                dist += 1
        return nums[curr] >= dist

