class Solution:
    def mostProfitablePath(
        self, edges: List[List[int]], bob: int, amount: List[int]
    ) -> int:
        links = defaultdict(list)
        for edge in edges:
            links[edge[0]].append(edge[1])
            links[edge[1]].append(edge[0])

        bob_path = [bob]

        def dfs1(curr: int, parent: int):
            if curr == 0:
                return True
            for link in links[curr]:
                if link == parent:
                    continue
                bob_path.append(link)
                if x := dfs1(link, curr):
                    return True
                bob_path.pop()

        dfs1(bob, -1)
        res = -float("inf")

        def dfs2(curr: int, parent: int, acc: int, sec: int):
            nonlocal res
            end = True
            for link in links[curr]:
                if link == parent:
                    continue
                to_add = amount[link]
                bob_sec = len(bob_path) - 1 if sec >= len(bob_path) else sec
                if link == bob_path[bob_sec]:
                    to_add //= 2
                elif link in bob_path[:bob_sec]:
                    to_add = 0
                dfs2(link, curr, acc + to_add, sec + 1)
                end = False
            if end:
                res = max(res, acc)

        dfs2(0, -1, amount[0], 1)
        return res
