# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, val=0, left=None, right=None):
#         self.val = val
#         self.left = left

#         self.right = right

class Solution:
    def postorderTraversal(self, root: Optional[TreeNode]) -> List[int]:
        if root is None:
            return []
        q = []
        res = []
        p = root
        while p is not None or len(q) > 0:
            if p is not None:
                res.append(p.val)
                q.append(p)
                p = p.right
            else:
                node = q.pop()
                p = node.left
        return res[::-1]

    def postorderTraversal2(self, root: Optional[TreeNode]) -> List[int]:
        if root is None:

            return []

        self.res = []

        self.trav(root)
        return self.res

    def trav(self, node):

        if node.left is not None:
            self.trav(node.left)
        if node.right is not None:
            self.trav(node.right)

        self.res.append(node.val)

