class Solution:
    def numOfSubarrays(self, arr: List[int]) -> int:
        n, m = len(arr), 1e9 + 7
        for i, x in enumerate(arr):
            arr[i] = x % 2
        dp_zero, dp_one = [0] * n, [0] * n
        if arr[-1] == 0:
            dp_zero[-1] = 1
        else:
            dp_one[-1] = 1
        for i in range(n - 2, -1, -1):
            if arr[i] == 1:
                dp_one[i] = (1 + dp_zero[i + 1]) % m
                dp_zero[i] = dp_one[i + 1]
            else:
                dp_zero[i] = (1 + dp_zero[i + 1]) % m
                dp_one[i] = dp_one[i + 1]
        res = 0
        for x in dp_one:
            res += x
            res %= m
        return int(res)
