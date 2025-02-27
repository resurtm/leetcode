class Solution:
    def lenLongestFibSubseq(self, arr: List[int]) -> int:
        res, dp = 0, [[0] * len(arr) for _ in range(len(arr))]
        idx = {x: i for i, x in enumerate(arr)}
        for i in range(len(arr)):
            for j in range(i):
                diff = arr[i] - arr[j]
                if diff in idx and idx[diff] < j:
                    dp[j][i] += dp[idx[diff]][j] + 1
                else:
                    dp[j][i] = 2
                res = max(res, dp[j][i])
        return res if res > 2 else 0
