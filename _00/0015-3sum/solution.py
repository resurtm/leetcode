class Solution:
    def threeSum(self, nums: list[int]) -> list[list[int]]:
        res = set()
        nums = sorted(nums)
        for i in range(len(nums)):
            x = -nums[i]
            j, k = i + 1, len(nums) - 1
            while j < k:
                s = nums[j] + nums[k]
                if s < x:
                    j += 1
                if s > x:
                    k -= 1
                if s == x:
                    res.add(tuple(sorted([nums[i], nums[j], nums[k]])))
                    j += 1
                    k -= 1
        return [x for x in res]
