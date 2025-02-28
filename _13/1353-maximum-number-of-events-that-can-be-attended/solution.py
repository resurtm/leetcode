class Solution:
    def maxEvents(self, events: List[List[int]]) -> int:
        events, days, l = sorted(events), 0, len(events)
        for a, b in events:
            days = max(days, max(a, b))
        h, event, day, res = [], 0, 1, 0
        heapq.heapify(h)
        while day <= days:
            if event < l and not h:
                day = events[event][0]
            while event < l and events[event][0] <= day:
                heapq.heappush(h, events[event][1])
                event += 1
            while h and h[0] < day:
                heapq.heappop(h)
            if h:
                heapq.heappop(h)
                res += 1
            elif event >= l:
                break
            day += 1
        return res
