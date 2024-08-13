class Solution:
    def combinationSum2(self, candidates: List[int], target: int) -> List[List[int]]:
        res: set[tuple[int]] = set()
        candidates.sort()

        def go(acc: list[int], stidx: int):
            nonlocal res
            s = sum(acc)
            if s == target:
                res.add(tuple(sorted(acc)))
                return
            if s > target:
                return

            for idx in range(stidx, len(candidates)):
                if idx > stidx and candidates[idx] == candidates[idx - 1]:
                    continue
                if candidates[idx] > target:
                    break

                arr = acc + [candidates[idx]]
                go(arr.copy(), idx + 1)
                arr.pop()

        go([], 0)
        return [list(x) for x in res]
