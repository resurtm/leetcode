# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, val=0, left=None, right=None):
#         self.val = val
#         self.left = left

#         self.right = right

class Solution:
    def goodNodes(self, curr: TreeNode, mx: int = None) -> int:

        res = 0
        if mx is None:
            res += 1
            mx = curr.val
        else:
            res += 1 if mx <= curr.val else 0
        if curr.left is not None:
            res += self.goodNodes(curr.left, max(mx, curr.left.val))
        if curr.right is not None:
            res += self.goodNodes(curr.right, max(mx, curr.right.val))
        return res

