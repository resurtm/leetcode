class Solution:
    def findMinDifference(self, pts: List[str]) -> int:
        items = []
        for pt in pts:
            hour, minute = pt.split(":")
            hour, minute = int(hour), int(minute)
            items.append(hour * 60 + minute)
        items.sort()
        res = self.cmp(items[-1], items[0])
        for idx in range(0, len(items) - 1):
            res = min(res, self.cmp(items[idx], items[idx + 1]))
        return res

    def cmp(self, a: int, b: int) -> int:
        x = abs(a - b)
        y = abs((a - 24 * 60) - b)
        z = abs((a - 24 * 60) - (b - 24 * 60))
        return min(x, y, z)

