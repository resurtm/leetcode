# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, val=0, left=None, right=None):
#         self.val = val
#         self.left = left

#         self.right = right


class Solution:
    def levelOrder(self, root: Optional[TreeNode]) -> List[List[int]]:
        if root is None:
            return []
        self.res = []
        self.traverse(root)
        return [lev for lev in self.res if len(lev) > 0]

    def traverse(self, node: TreeNode, level: int = 0) -> None:
        if len(self.res) - 1 != level:
            self.res.append([])
        self.res[level].append(node.val)
        if node.left is not None:
            self.traverse(node.left, level + 1)
        if node.right is not None:
            self.traverse(node.right, level + 1)
