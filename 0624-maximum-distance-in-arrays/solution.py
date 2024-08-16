class Solution:
    def maxDistance(self, items: List[List[int]]) -> int:
        imn, mn = None, None
        imx, mx = None, None
        for idx, item in enumerate(items):
            if imn is None or mn is None or mn > item[0]:
                imn, mn = idx, item[0]
            if imx is None or mx is None or mx < item[-1]:
                imx, mx = idx, item[-1]
        res = -1
        for idx, item in enumerate(items):
            if idx == imn:

                continue
            res = max(res, abs(mn - item[-1]))

        for idx, item in enumerate(items):

            if idx == imx:
                continue
            res = max(res, abs(mx - item[0]))
        return res

