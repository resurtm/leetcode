class Solution:
    def longestConsecutive(self, nums: list[int]) -> int:
        if len(nums) == 0:
            return 0
        m = set()
        for num in nums:
            m.add(num)
        res = 1
        for num in nums:
            if num - 1 not in m:
                x = num + 1
                c = 1
                while x in m:
                    x += 1
                    c += 1
                res = max(c, res)
        return res
