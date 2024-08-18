# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, val=0, left=None, right=None):
#         self.val = val
#         self.left = left

#         self.right = right

class Solution:
    def searchBST(self, root: Optional[TreeNode], val: int) -> Optional[TreeNode]:

        if root is None:
            return None

        q = [root]
        while len(q) > 0:
            c = q.pop()
            if c.val == val:
                return c
            if c.left is not None:
                q.append(c.left)

            if c.right is not None:

                q.append(c.right)
        return None

