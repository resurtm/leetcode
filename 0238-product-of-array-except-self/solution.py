class Solution:
    def productExceptSelf(self, nums: List[int]) -> List[int]:
        res = [0] * len(nums)
        res[0] = 1
        for idx in range(1, len(nums)):
            res[idx] = res[idx - 1] * nums[idx - 1]
        r = 1
        for idx in range(len(nums) - 1, -1, -1):
            res[idx] *= r
            r *= nums[idx]
        return res

    def productExceptSelfSlow(self, nums: List[int]) -> List[int]:
        curr, sfx = None, []
        for idx in range(len(nums)):
            if curr is None:
                curr = nums[idx]
            else:
                curr *= nums[idx]
            sfx.append(curr)
        curr, pfx = None, []
        for idx in range(len(nums) - 1, -1, -1):
            if curr is None:
                curr = nums[idx]
            else:
                curr *= nums[idx]
            pfx.insert(0, curr)
        res = []
        for idx in range(len(nums)):
            a = 1 if idx > len(nums) - 2 else pfx[idx + 1]
            b = 1 if idx < 1 else sfx[idx - 1]
            res.append(a * b)
        return res
