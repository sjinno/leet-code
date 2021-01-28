// # Definition for a binary tree node.
// # class TreeNode:
// #     def __init__(self, val=0, left=None, right=None):
// #         self.val = val
// #         self.left = left
// #         self.right = right
// class Solution:
//     def isSameTree(self, p: TreeNode, q: TreeNode) -> bool:
//         if p is None and q is None:
//             return True
//         elif p is None or q is None:
//             return False
        
//         if p.val != q.val:
//             return False
        
//         return self.isSameTree(p.left, q.left) and self.isSameTree(p.right, q.right)

// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//   pub val: i32,
//   pub left: Option<Rc<RefCell<TreeNode>>>,
//   pub right: Option<Rc<RefCell<TreeNode>>>,
// }
// 
// impl TreeNode {
//   #[inline]
//   pub fn new(val: i32) -> Self {
//     TreeNode {
//       val,
//       left: None,
//       right: None
//     }
//   }
// }
use std::rc::Rc;
use std::cell::RefCell;
fn is_same_tree(p: Option<Rc<RefCell<TreeNode>>>, q: Option<Rc<RefCell<TreeNode>>>) -> bool {
    match (p, q) {
        (None, None) => true,
        (None, _) | (_, None) => false,
        (Some(a), Some(b)) => {
            if a != b { return false; }
            Solution::is_same_tree(a.borrow().left.clone(), b.borrow().left.clone()) 
         && Solution::is_same_tree(a.borrow().right.clone(), b.borrow().right.clone())
        }
    }
}
