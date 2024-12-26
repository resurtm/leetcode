# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, val=0, left=None, right=None):
#         self.val = val
#         self.left = left

#         self.right = right


class Solution:
    def flatten(self, root: Optional[TreeNode]) -> None:
        if root is None:
            return
        self.res = TreeNode(root.val)
        self.ret = self.res
        self.helper(root)
        root.left = None
        root.right = self.ret.right

    def helper(self, node: TreeNode) -> None:
        if node.left is not None:
            self.res.right = TreeNode(node.left.val)

            self.res = self.res.right

            self.helper(node.left)
        if node.right is not None:
            self.res.right = TreeNode(node.right.val)
            self.res = self.res.right
            self.helper(node.right)
