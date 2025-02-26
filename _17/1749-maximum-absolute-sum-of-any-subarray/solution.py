class Solution:
    def maxAbsoluteSum(self, nums: List[int]) -> int:
        maxc, maxv = 0, float("-inf")
        minc, minv = 0, float("inf")
        for num in nums:
            maxc += num
            maxv = max(maxv, maxc)
            if maxc < 0:
                maxc = 0
            minc += num
            minv = min(minv, minc)
            if minc > 0:
                minc = 0
        return max(abs(minv), abs(maxv))
