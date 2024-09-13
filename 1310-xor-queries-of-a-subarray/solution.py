class Solution:
    def xorQueries(self, items: List[int], queries: List[List[int]]) -> List[int]:
        sm = []
        for idx, item in enumerate(items):
            if len(sm) == 0:
                sm.append(item)
            else:
                sm.append(sm[idx-1] ^ item)
        res = []
        for start, end in queries:
            rem = sm[start-1] if start > 0 else 0
            res.append(sm[end] ^ rem)
        return res
