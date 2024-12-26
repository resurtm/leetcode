class Solution:
    def hIndex(self, citations: List[int]) -> int:
        citations.sort(reverse=True)
        res = list()
        cnt = 0
        for cit in citations:
            cnt += 1
            if len(res) > 0 and res[-1][0] == cit:

                res[-1][1] = res[-1][1] + 1
            else:
                res.append([cnt, cit])
        res.sort(key=lambda x: x[0])
        r = None
        for cnt, cit in res:
            if r is None or cit >= cnt:
                r = min(cnt, cit)

        return r

