// 104. Maximum Depth of Binary Tree
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
impl Solution {
    pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if root.is_none() {
            return 0;
        }
        
        let mut depth = 0;
        let mut stack = vec![];
        stack.push((root.unwrap(), 1));
        
        while !stack.is_empty() {
            if let Some((node, d)) = stack.pop() {
                if d > depth { depth = d; }
                if !node.borrow().left.is_none() { 
                    stack.push((node.borrow().left.clone().unwrap(), d + 1)); 
                }
                if !node.borrow().right.is_none() { 
                    stack.push((node.borrow().right.clone().unwrap(), d + 1)); 
                }
            }
        }
        
        return depth;
    }
}

fn main() {}


// In Python
// # Definition for a binary tree node.
// # class TreeNode:
// #     def __init__(self, val=0, left=None, right=None):
// #         self.val = val
// #         self.left = left
// #         self.right = right
// class Solution:
//     def maxDepth(self, root):
//         if root is None:
//             return 0
        
//         stack = []
//         depth = 0
//         stack.append((root, 1))
        
//         while stack != []:
//             node, d = stack.pop()
//             if d > depth:
//                 depth = d
//             if node.left is not None:
//                 stack.append((node.left, d + 1))
//             if node.right is not None:
//                 stack.append((node.right, d + 1))
        
//         return depth