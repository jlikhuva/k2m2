# Cartesian Tree

Convert any array into a Cartesian Tree in Linear Time.

## Background

A cartesian tree is a derivative data structure. It is derived from an underlying array.
More formally, the cartesian tree `T` of an array `A` is a min binary heap of the elements of `A`
organized such that an in order traversal of the tree yields the original array.

To construct a cartesian tree from an underlying array, we are guided by the following:

* An in-order traversal must yield the array elements in their
* The tree should be a min heap. That is, the smallest element should be at the root.
* When doing an in-order traversal, the right child is retrieved after both the parent and the left child —
consequently, the right-most node will be the last node retrieved.

## Wait, but Why?

Why are cartesian trees important? They are quite useful when solving the `Range Min Query` problem: given a cartesian tree for an array, we can answer any RMQ on that array. In particular, $RMQ_A(o, j) = LCA_T(A[i], A[j])$. That is we can answer RMQ by doing lowest common ancestor searches in the cartesian tree. See [here for more details](http://courses.csail.mit.edu/6.851/fall17/scribe/lec15.pdf).

## Implementation Details

We build the cartesian tree incrementally -- adding in elements in the order that they appear in the array. To add an element `X`, we inspect the right spine of the tree starting with the right most node. We follow parent pointers until we find an element, `Y` , in the tree that is smaller than `X`.  We the modify the tree, making  `X` a right child of `Y`. We also make the rest of the right subtree that is below `X`  a left subtree of the new node.

Traversing the right spine of tree from the right-most node can be done efficiently by keeping nodes on the right spine in a stack. That way, the rightmost node is always at the top of the stack.

## Cartesian Tree Isomorphisms

When do two cartesian trees for two different arrays, `A` and `B`,  have the same shape? How can we tell this efficiently?

Put simply, if two arrays have the same cartesian tree shape, then the minimal values, in **any** range, in both arrays occur at the same index. This means that, the sequence of `Push` and `Pop` operations when constructing the cartesian trees for the two arrays are exactly the same. Therefore, to know if two arrays are isomorphic, we could simply compare the operations needed to construct each tree.

As an aside, when we are only interested in whether two arrays have isomorphic trees, we don't even need to construct the tree. We can instead create a bit-string from the sequence of `Push` and `Pop` operations. The number formed by this bit-string is called the `cartesian tree number`. Therefore, with this scheme, ***two arrays have isomorphic trees if they have the same cartesian tree number.***

## Further Reading

[Cartesian Trees applied to the RMQ Problem](https://github.com/jlikhuva/blog/blob/main/posts/mathematical-sciences/rmq.md#cartesian-trees--the-lca-rmq-equivalence)