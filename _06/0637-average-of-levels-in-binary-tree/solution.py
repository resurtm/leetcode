# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, val=0, left=None, right=None):
#         self.val = val
#         self.left = left
#         self.right = right


class Solution:
    def averageOfLevels(self, root: Optional[TreeNode]) -> List[float]:
        if root is None:
            return [0.0]
        self.lev = {}
        self.max_lev = 0
        self.helper(root)
        res = []
        for l in range(self.max_lev + 1):
            x = sum(self.lev[l]) / len(self.lev[l])
            res.append(x)
        return res

    def helper(self, curr: TreeNode, lvl: int = 0):
        self.max_lev = max(self.max_lev, lvl)
        self.lev.setdefault(lvl, [])
        self.lev[lvl].append(curr.val)
        if curr.left is not None:
            self.helper(curr.left, lvl + 1)
        if curr.right is not None:
            self.helper(curr.right, lvl + 1)
