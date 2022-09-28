//! Cartesian Tree
//!

type Nodes<'a, T> = Vec<CartesianTreeNode<'a, T>>;
type Stack = Vec<CartesianNodeIdx>;
type Actions = Vec<CartesianTreeAction>;

/// An index into a collection of cartesian tree nodes
#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Clone)]
struct CartesianNodeIdx(usize);

impl<'a, T: Ord> std::ops::Index<CartesianNodeIdx> for Vec<CartesianTreeNode<'a, T>> {
    type Output = CartesianTreeNode<'a, T>;
    fn index(&self, index: CartesianNodeIdx) -> &Self::Output {
        &self[index.0]
    }
}

impl<'a, T: Ord> std::ops::IndexMut<CartesianNodeIdx> for Vec<CartesianTreeNode<'a, T>> {
    fn index_mut(&mut self, index: CartesianNodeIdx) -> &mut Self::Output {
        &mut self[index.0]
    }
}

#[derive(Debug)]
struct CartesianTreeNode<'a, T: Ord> {
    /// A reference to the array value that this node represents
    value: &'a T,

    /// The locations of the children and parent of this node.
    left_child_idx: Option<CartesianNodeIdx>,
    right_child_idx: Option<CartesianNodeIdx>,
}

impl<'a, T: Ord> From<&'a T> for CartesianTreeNode<'a, T> {
    fn from(value: &'a T) -> Self {
        CartesianTreeNode {
            value,
            left_child_idx: None,
            right_child_idx: None,
        }
    }
}

/// When constructing a cartesian tree, we either
/// push a node to or pop a node from a stack.
/// We keep track of these actions because we can
/// use them to generate the cartesian tree number.
#[derive(Debug, Eq, PartialEq)]
enum CartesianTreeAction {
    Push,
    Pop,
}

/// A cartesian tree is a heap ordered binary tree
/// derived from some underlying array. An in-order
/// traversal of the tree yields the underlying array.
#[derive(Debug)]
pub struct CartesianTree<'a, T: Ord> {
    nodes: Vec<CartesianTreeNode<'a, T>>,
    root_idx: Option<CartesianNodeIdx>,
    action_profile: Vec<CartesianTreeAction>,
}

// To create the cartesian tree, we pop the stack until either
// it's empty or the element atop the stack has a smaller value
// than the element we are currently trying to add to the stack.
// Once we break out of the `pop` loop, we make the item we popped
// a left child of the new item we are adding. Additionally, we make
// this new item a right/left child of the item atop the stack
impl<'a, T: Ord> From<&'a [T]> for CartesianTree<'a, T> {
    fn from(underlying: &'a [T]) -> Self {
        let len = underlying.len();
        let mut nodes = Vec::with_capacity(len);
        let mut stack = Vec::<CartesianNodeIdx>::with_capacity(len);
        let mut action_profile = Vec::with_capacity(len * 2);
        for (idx, value) in underlying.iter().enumerate() {
            nodes.push(value.into());
            let node_idx = CartesianNodeIdx(idx);
            Self::add_node_to_cartesian_tree(&mut nodes, &mut stack, &mut action_profile, node_idx);
        }
        let root_idx = stack.first().map(|min| min.clone());
        CartesianTree {
            nodes,
            root_idx,
            action_profile,
        }
    }
}

impl<'a, T: Ord> CartesianTree<'a, T> {
    pub fn in_order_traversal(&self) -> Vec<&T> {
        let mut res = Vec::with_capacity(self.nodes.len());
        self.traversal_helper(&self.root_idx, &mut res);
        res
    }

    fn traversal_helper(&self, cur_idx: &Option<CartesianNodeIdx>, res: &mut Vec<&'a T>) {
        let nodes = &self.nodes;
        match cur_idx {
            None => {}
            Some(cur_sub_root) => {
                self.traversal_helper(&nodes[cur_sub_root.clone()].left_child_idx, res);
                res.push(&nodes[cur_sub_root.clone()].value);
                self.traversal_helper(&nodes[cur_sub_root.clone()].right_child_idx, res);
            }
        }
    }

    /// Calculates the cartesian tree number of this tree
    /// using the sequence of `push` and `pop` operations
    /// stored in the `action_profile`. Note that calculating this
    /// value only makes sense when the underlying array is small.
    /// More specifically, this procedure assumes that the underlying
    /// array has at most 32 items. This makes sense in our context
    /// since we're mostly interested in the cartesian tree numbers
    /// of RMQ blocks
    pub fn cartesian_tree_number(&self) -> u64 {
        let mut number = 0;
        let mut offset = 0;
        for action in &self.action_profile {
            if action == &CartesianTreeAction::Push {
                number |= 1 << offset;
            }
            offset += 1;
        }
        number
    }

    /// Adds the node at the given idx into the tree by wiring up the
    /// child and parent pointers. it is assumed that the
    /// node has already been added to `nodes` the list of nodes.
    /// This procedure returns an optional index value
    /// that is populated if the root changed.
    fn add_node_to_cartesian_tree(
        nodes: &mut Nodes<T>,
        stack: &mut Stack,
        actions: &mut Actions,
        new_idx: CartesianNodeIdx,
    ) -> () {
        let mut last_popped = None;
        loop {
            match stack.last() {
                None => break,
                Some(top_node_idx) => {
                    // If the new node is greater than the value atop the stack,
                    // we make the new node a right child of that value
                    if nodes[top_node_idx.clone()].value < nodes[new_idx.clone()].value {
                        nodes[top_node_idx.clone()].right_child_idx = Some(new_idx.clone());
                        break;
                    }
                    last_popped = stack.pop();
                    actions.push(CartesianTreeAction::Pop);
                }
            }
        }
        // We make the last item we popped a left child of the
        // new node
        if let Some(last_popped_idx) = last_popped {
            nodes[new_idx.clone()].left_child_idx = Some(last_popped_idx);
        }
        stack.push(new_idx);
        actions.push(CartesianTreeAction::Push);
    }
}

#[test]
fn test_cartesian_tree() {
    use pretty_assertions::assert_eq;
    
    let v = [93, 84, 33, 64, 62, 83, 63];
    let tree: CartesianTree<'_, _> = v.as_ref().into();
    assert!(tree.root_idx.is_some());
    assert_eq!(tree.nodes[tree.root_idx.clone().unwrap()].value, &33);
    for (&l, &r) in tree.in_order_traversal().into_iter().zip(v.iter()) {
        assert_eq!(l, r);
    }
}