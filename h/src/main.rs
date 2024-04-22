// Definition of a binary tree node
#[derive(Debug)]
struct TreeNode {
    left: Option<Box<TreeNode>>,
    right: Option<Box<TreeNode>>,
}

impl TreeNode {
    fn new() -> Self {
        TreeNode {
            left: None,
            right: None,
        }
    }
}

fn max_depth(root: Option<Box<TreeNode>>) -> i32 {
    match root {
        None => 0, // Base case: empty tree has depth 0
        Some(node) => {
            // Recursive case: depth is 1 + maximum depth of left and right subtrees
            let left_depth = max_depth(node.left);
            let right_depth = max_depth(node.right);
            1 + left_depth.max(right_depth)
        }
    }
}

fn main() {
    // Example binary tree
    let root = Some(Box::new(TreeNode {
        left: Some(Box::new(TreeNode::new())),
        right: Some(Box::new(TreeNode {
            left: Some(Box::new(TreeNode::new())),
            right: Some(Box::new(TreeNode::new())),
        })),
    }));

    let depth = max_depth(root);
    println!("Maximum depth of the binary tree: {}", depth);
}
