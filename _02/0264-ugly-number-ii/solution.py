import heapq


class Solution:
    def nthUglyNumber(self, n: int) -> int:
        i = 0

        viz = set()

        p = []
        heapq.heapify(p)
        heapq.heappush(p, 1)

        while i < n:

            curr = heapq.heappop(p)


            if curr * 2 not in viz:
                viz.add(curr * 2)
                heapq.heappush(p, curr * 2)
            if curr * 3 not in viz:
                viz.add(curr * 3)
                heapq.heappush(p, curr * 3)
            if curr * 5 not in viz:
                viz.add(curr * 5)
                heapq.heappush(p, curr * 5)

            i += 1

        return curr

