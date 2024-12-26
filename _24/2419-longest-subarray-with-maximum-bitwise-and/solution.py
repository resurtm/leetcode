class Solution:
    def longestSubarray(self, nums: List[int]) -> int:
        m = max(nums)
        res, mx, fin = 0, False, 0
        for num in nums:
            if num == m and not mx:

                res = 0
                mx = True
            elif num != m and mx:
                fin = max(fin, res)
                mx = False
            if mx:
                res += 1
        fin = max(fin, res)
        return fin

