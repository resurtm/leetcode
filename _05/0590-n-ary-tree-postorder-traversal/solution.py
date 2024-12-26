"""
# Definition for a Node.

class Node:

    def __init__(self, val=None, children=None):
        self.val = val
        self.children = children

"""


class Solution:
    def postorder(self, root: "Node") -> List[int]:
        if root is None:
            return []
        res = []
        q = [root]
        while len(q) > 0:
            curr = q.pop()
            for child in curr.children:
                q.append(child)
            res.append(curr.val)
        return res[::-1]

    def postorder2(self, root: "Node") -> List[int]:
        if root is None:
            return []
        self.res = []
        self.trav(root)
        return self.res


    def trav(self, root: "Node"):
        for child in root.children:
            self.trav(child)
        self.res.append(root.val)

