from collections import defaultdict
from typing import List


class Solution:
    def countPairs(self, n: int, edges: List[List[int]]) -> int:
        links = defaultdict(list)
        for edge in edges:
            links[edge[0]].append(edge[1])
            links[edge[1]].append(edge[0])
        viz, res, sum = set(), 0, n
        for nx in range(n):
            if nx in viz:
                continue
            curr, q = set([nx]), [nx]
            while len(q) > 0:
                for link in links[q.pop(0)]:
                    if link not in viz:
                        viz.add(link)
                        curr.add(link)
                        q.append(link)
            sum -= len(curr)
            res += len(curr) * sum
        return res
