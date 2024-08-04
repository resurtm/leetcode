class RecentCounter:
    def __init__(self):
        self.it = []

    def ping(self, t: int) -> int:
        self.it.append(t)
        res = 0
        for idx in range(len(self.it) - 1, -1, -1):
            if t - self.it[idx] > 3000:
                break
            res += 1
        return res


# Your RecentCounter object will be instantiated and called as such:
# obj = RecentCounter()
# param_1 = obj.ping(t)
