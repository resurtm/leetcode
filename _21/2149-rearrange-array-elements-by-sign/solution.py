class Solution:
    def rearrangeArray(self, nums: List[int]) -> List[int]:
        l, p, n, res = len(nums), 0, 0, []
        while len(res) < l:
            while p < l and nums[p] < 0:
                p += 1
            if p < l:
                res.append(nums[p])
                p += 1
            while n < l and nums[n] > 0:
                n += 1
            if n < l:
                res.append(nums[n])
                n += 1
        return res
