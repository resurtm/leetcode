# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, val=0, left=None, right=None):
#         self.val = val
#         self.left = left
#         self.right = right

class Solution:
    def pathSum(self, root: Optional[TreeNode], targetSum: int) -> int:
        if root is None:
            return 0
        self.targetSum = targetSum
        self.res = 0
        self.traverse(root, [root.val])
        return self.res

    def traverse(self, node: TreeNode, sums: list[int]):
        self.res += sums.count(self.targetSum)
        if node.left is not None:
            nsums = [x + node.left.val for x in sums] + [node.left.val]
            self.traverse(node.left, nsums)
        if node.right is not None:
            nsums = [x + node.right.val for x in sums] + [node.right.val]
            self.traverse(node.right, nsums)

