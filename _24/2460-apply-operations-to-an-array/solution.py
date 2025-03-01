class Solution:
    def applyOperations(self, nums: List[int]) -> List[int]:
        res1, res2, l = [], [], len(nums)
        for i in range(l):
            if i != l - 1 and nums[i] == nums[i + 1]:
                nums[i] *= 2
                nums[i + 1] = 0
            if nums[i] == 0:
                res2.append(nums[i])
            else:
                res1.append(nums[i])
        return res1 + res2
