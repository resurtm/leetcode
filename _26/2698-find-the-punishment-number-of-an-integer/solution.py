class Solution:
    def punishmentNumber(self, n: int) -> int:
        return sum([x * x for x in range(1, n + 1) if self.check(x)])

    def check(self, x: int, acc: list[str] = None, pos: int = 0) -> bool:
        if x == 1:
            return True
        if acc is None:
            acc = []
        xs = str(x * x)
        if pos >= len(xs):
            return x == sum([int(c) for c in acc])
        for szi in range(len(xs) - 1, 0, -1):
            acc.append(xs[pos : pos + szi])
            if self.check(x, acc, pos + szi):
                return True
            acc.pop()
        return False
