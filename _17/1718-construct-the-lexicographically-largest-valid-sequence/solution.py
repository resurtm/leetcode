class Solution:
    def constructDistancedSequence(self, n: int) -> list[int]:
        ln = 2 * n - 1
        res, use = [0] * ln, set()

        def bc(pos: int) -> bool:
            if pos == ln:
                return True
            if res[pos] != 0:
                return bc(pos + 1)
            for x in range(n, 0, -1):
                if x in use:
                    continue
                xn = pos + x if x > 1 else pos
                if xn >= ln or res[xn] != 0:
                    continue
                res[pos] = res[xn] = x
                use.add(x)
                if bc(pos + 1):
                    return True
                res[pos] = res[xn] = 0
                use.remove(x)
            return False

        bc(0)
        return res
