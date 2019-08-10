#[derive(Debug)]
struct TreeNode<K, V> {
    key: K,
    value: V,
    left: Option<Box<TreeNode<K, V>>>,
    right: Option<Box<TreeNode<K, V>>>,
}
trait BinarySearchTree<K: PartialOrd, V> {
    fn insert(&mut self, key: K, value: V);
}
impl<K, V> TreeNode<K, V> {
    fn new(key: K, value: V) -> TreeNode<K, V> {
        TreeNode {
            key,
            value,
            left: None,
            right: None,
        }
    }
}
impl<K: PartialOrd, V> BinarySearchTree<K, V> for TreeNode<K, V> {
    fn insert(&mut self, key: K, value: V) {
        if key < self.key {
            if let Some(ref mut left) = self.left {
                left.insert(key, value);
            } else {
                self.left = Some(Box::new(TreeNode::new(key, value)));
            }
        } else {
            if let Some(ref mut right) = self.right {
                right.insert(key, value);
            } else {
                self.right = Some(Box::new(TreeNode::new(key, value)));
            }
        }
    }
}

fn main() {
    let mut root = TreeNode::new(3, 4);
    root.insert(2, 10);
    root.insert(4, 11);
    root.insert(1, 11);
    println!("{:?}", root);
}
