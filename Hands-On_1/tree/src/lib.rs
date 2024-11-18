use std::ops::Add;

pub struct Node {
    key: u32,
    id_left: Option<usize>,
    id_right: Option<usize>,
}

impl Node {
    fn new(key: u32) -> Self {
        Self {
            key,
            id_left: None,
            id_right: None,
        }
    }
}

pub struct Tree {
    nodes: Vec<Node>,
}

impl Tree {
    pub fn with_root(key: u32) -> Self {
        Self {
            nodes: vec![Node::new(key)],
        }
    }

    /// Adds a child to the node with `parent_id` and returns the id of the new node.
    /// The new node has the specified `key`. The new node is the left  child of the  
    /// node `parent_id` if `is_left` is `true`, the right child otherwise.
    ///
    /// # Panics
    /// Panics if the `parent_id` does not exist, or if the node `parent_id ` has  
    /// the child already set.
    pub fn add_node(&mut self, parent_id: usize, key: u32, is_left: bool) -> usize {
        assert!(
            parent_id < self.nodes.len(),
            "Parent node id does not exist"
        );
        if is_left {
            assert!(
                self.nodes[parent_id].id_left.is_none(),
                "Parent node has the left child already set"
            );
        } else {
            assert!(
                self.nodes[parent_id].id_right.is_none(),
                "Parent node has the right child already set"
            );
        }

        let child_id = self.nodes.len();
        self.nodes.push(Node::new(key));

        let child = if is_left {
            &mut self.nodes[parent_id].id_left
        } else {
            &mut self.nodes[parent_id].id_right
        };

        *child = Some(child_id);

        child_id
    }

    /// Returns the sum of all the keys in the tree
    pub fn sum(&self) -> u32 {
        self.rec_sum(Some(0))
    }

    /// A private recursive function that computes the sum of
    /// nodes in the subtree rooted at `node_id`.
    fn rec_sum(&self, node_id: Option<usize>) -> u32 {
        if let Some(id) = node_id {
            assert!(id < self.nodes.len(), "Node id is out of range");
            let node = &self.nodes[id];

            let sum_left = self.rec_sum(node.id_left);
            let sum_right = self.rec_sum(node.id_right);

            return sum_left + sum_right + node.key;
        }

        0
    }

    /// Returns true if tree is a BST, false instead
    pub fn is_bst(&self) -> bool {
        self.rec_is_bst(Some(0)).0
    }

    /// A private recursive function that evaluates if the subtree
    /// of the input node is a BST
    fn rec_is_bst(&self, node_id: Option<usize>) -> (bool, Option<u32>, Option<u32>) {
        if let Some(id) = node_id {
            assert!(id < self.nodes.len(), "Node id is out of range");
            let node = &self.nodes[id];

            let (is_left_bst, min_left, max_left) = self.rec_is_bst(node.id_left);
            let (is_right_bst, min_right, max_right) = self.rec_is_bst(node.id_right);

            if !is_left_bst || !is_right_bst {
                return (false, None, None);
            }

            match (min_left, max_left, min_right, max_right) {
                (None, None, None, None) => return (true, Some(node.key), Some(node.key)),
                (None, None, Some(min_right), Some(max_right)) if min_right >= node.key => {
                    return (true, Some(node.key), Some(max_right));
                }
                (Some(min_left), Some(max_left), None, None) if max_left < node.key => {
                    return (true, Some(min_left), Some(node.key));
                }
                (Some(min_left), Some(max_left), Some(min_right), Some(max_right))
                    if max_left < node.key && min_right >= node.key =>
                {
                    return (true, Some(min_left), Some(max_right));
                }
                _ => {
                    return (false, None, None);
                }
            }
        }

        (true, None, None)
    }

    /// Return the max value of a simple path from leaf to leaf
    pub fn max_leaf_path(&self) -> Option<u32> {
        match self.rec_max_leaf_path(Some(0)) {
            (Some(best), _) => Some(best),
            _ => None,
        }
    }

    /// Auxiliary function that performs sum between Option<u32> inputs
    /// if a or b or both are None, the result is None
    /// the result is a+b instead
    fn add(&self, a: Option<u32>, b: Option<u32>) -> Option<u32> {
        match (a, b) {
            (Some(a), Some(b)) => Some(a.add(b)),
            (_, _) => None,
        }
    }

    /// A private recursive function that evaluate the
    /// max value of a simple path in the node subtree
    fn rec_max_leaf_path(&self, node_id: Option<usize>) -> (Option<u32>, Option<u32>) {
        if let Some(id) = node_id {
            assert!(id < self.nodes.len(), "Node id is out of range");
            let node = &self.nodes[id];

            let (best_left, max_left) = self.rec_max_leaf_path(node.id_left);
            let (best_right, max_right) = self.rec_max_leaf_path(node.id_right);

            let best_node = self.add(self.add(max_left, max_right), Some(node.key));
            let best = best_left.max(best_right.max(best_node));
            let max = self.add(max_left.max(max_right), Some(node.key));

            if max.is_none() {
                return (best, Some(node.key));
            } else {
                return (best, max);
            }
        }
        (None, None)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum() {
        let mut tree = Tree::with_root(10);
        assert_eq!(tree.sum(), 10);

        tree.add_node(0, 5, true); // id 1
        tree.add_node(0, 22, false); // id 2
        assert_eq!(tree.sum(), 37);

        tree.add_node(1, 7, false); // id 3
        tree.add_node(2, 20, true); // id 4
        assert_eq!(tree.sum(), 64);
    }

    #[test]
    fn test_is_bst() {
        // tree with only one element is a binary search tree
        let mut tree = Tree::with_root(10);
        assert!(tree.is_bst());

        tree.add_node(0, 5, true); // id 1
        tree.add_node(0, 22, false); // id 2
        assert!(tree.is_bst());

        tree.add_node(1, 4, true); // id 3
        assert!(tree.is_bst());
        tree.add_node(1, 9, false); // id 4
        assert!(tree.is_bst());

        tree.add_node(2, 15, true); // id 5
        tree.add_node(2, 23, false); // id 6
        assert!(tree.is_bst());

        tree.add_node(3, 15, true); // id 5
        assert!(!tree.is_bst());
    }

    #[test]
    fn test_max_leaf_path() {
        let mut tree = Tree::with_root(10);
        assert!(tree.max_leaf_path().is_none());

        tree.add_node(0, 5, true); // id 1
        assert!(tree.max_leaf_path().is_none());

        tree.add_node(1, 9, false); // id 2
        assert!(tree.max_leaf_path().is_none());

        tree.add_node(0, 22, false); // id 3
        tree.add_node(3, 2, true); // id 4
        assert_eq!(tree.max_leaf_path().unwrap(), 48);

        tree.add_node(1, 4, true); // id 5
        assert_eq!(tree.max_leaf_path().unwrap(), 48);

        tree.add_node(3, 50, false); // id 6
        assert_eq!(tree.max_leaf_path().unwrap(), 96);

        tree.add_node(4, 50, true); // id 7
        assert_eq!(tree.max_leaf_path().unwrap(), 124);
    }
}
