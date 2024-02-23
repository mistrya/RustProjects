use std::{cell::RefCell, rc::Rc};

#[derive(Debug, Clone)]
pub struct TreeNode {
  val: i32,
  left: Option<TreeNodeRef>,
  right: Option<TreeNodeRef>,
}

type TreeNodeRef = Rc<RefCell<TreeNode>>;

pub fn tree_sum_recursive(root: Option<&TreeNodeRef>) -> i32 {
    // if `root` has `Some`thing
    // return `root.val` + left_node_val + right_node_val
    if let Some(root) = root {
        root.borrow().val
            // recursively call left path
            + tree_sum_recursive(root.borrow().left.as_ref())
            // recursively call right path
            + tree_sum_recursive(root.borrow().right.as_ref())
    } else {
        // root is None (i.e. empty or null)
        // so return `0`
        0
    }
}