//! # Cartesian Tree
//!
//! Convert any array into a Cartesian Tree in Linear Time.
//!
//! A cartesian tree is a derivative data structure. It is derived from an underlying array.
//! More formally, the cartesian tree `T` of an array `A` is a min binary heap of the elements of `A`
//! organized such that an in order traversal of the tree yields the original array.
//!
//! To construct a cartesian tree from an underlying array, we are guided by the following:
//!  * An in-order traversal must yield the array elements in their
//! * The tree should be a min heap. That is, the smallest element should be at the root.
//! * When doing an in-order traversal, the right child is retrieved after both the parent and the left child —
//! consequently, the right-most node will be the last node retrieved.
//!

pub mod tree;

#[cfg(test)]
mod test_cartesian_tree {
    use super::tree;
    use pretty_assertions::assert_eq;
    #[test]
fn test_cartesian_tree() {
    let v = [93, 84, 33, 64, 62, 83, 63];
    let tree: tree::CartesianTree<'_, _> = v.as_ref().into();

    for (&l, &r) in tree.in_order_traversal().into_iter().zip(v.iter()) {
        assert_eq!(l, r);
    }
}
}
