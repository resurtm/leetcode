class Solution:
    def findSubsequences(self, nums: List[int]) -> List[List[int]]:
        res = set()

        def backtrack(pos, acc):
            nonlocal res
            for x in acc:
                if len(x) >= 2:
                    res.add(tuple(x))
            if pos == len(nums) - 1:
                return
            to_add = []
            for it in acc:
                if it[-1] > nums[pos + 1]:
                    continue
                to_add.append(it + [nums[pos + 1]])
            backtrack(pos + 1, acc + to_add)

        for i in range(len(nums)):
            backtrack(i, [[nums[i]]])
        return [list(x) for x in res]
