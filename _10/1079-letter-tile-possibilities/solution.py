class Solution:
    def numTilePossibilities(self, tiles: str) -> int:
        t = collections.Counter(tiles)
        res = 0

        def backtrack():
            for k, v in t.items():
                if t[k] == 0:
                    continue
                t[k] -= 1
                nonlocal res
                res += 1
                backtrack()
                t[k] += 1

        backtrack()
        return res
