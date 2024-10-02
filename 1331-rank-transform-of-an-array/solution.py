class Solution:
    def arrayRankTransform(self, arr: List[int]) -> List[int]:
        h = {x: i + 1 for i, x in enumerate(sorted(set(arr)))}
        return [h[x] for x in arr]
