"""
# Definition for a Node.

class Node:

    def __init__(self, val = 0, neighbors = None):
        self.val = val
        self.neighbors = neighbors if neighbors is not None else []
"""

from typing import Optional



class Solution:
    def cloneGraph(self, inp: Optional["Node"]) -> Optional["Node"]:
        if inp is None:
            return None
        res = None
        nodes = dict()
        q = [inp]
        while len(q) > 0:
            c = q.pop()
            nodes[c.val] = Node(c.val)
            if res is None:
                res = nodes[c.val]

            for neigh in c.neighbors:
                if neigh.val not in nodes:
                    q.append(neigh)
        q = [inp]
        while len(q) > 0:
            c = q.pop()
            neighs = []
            for neigh in c.neighbors:
                neighs.append(nodes[neigh.val])
            nodes[c.val].neighbors = neighs
            for neigh in c.neighbors:
                if len(nodes[neigh.val].neighbors) == 0:
                    q.append(neigh)
        return res


