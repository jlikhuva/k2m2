# Word Level Parallelism

[![Crates.io][crates-badge]][crates-url]

Bit level algorithms  useful when developing specialized integer data structures such as the [x-fast trie](http://web.stanford.edu/class/archive/cs/cs166/cs166.1166/lectures/15/Small15.pdf)

## Overview

Arithmetic and logical operations take, for all intents and purposes, constant time. Such operations operate on whole words. (A word is the size of a single memory segment. This crate, assumes a word size width of `64`. For a more in-depth discussion of computer memory, refer to [this note](https://akkadia.org/drepper/cpumemory.pdf)). For instance, it takes constant time to add two `64` bit numbers.

The central idea behind the algorithms in this library  is this:

* If you have a bunch of small integers — each smaller that sixty four bits, e.g. a bunch of bytes, we can pack many of them into a single sixty four bit integer.
* We can then operate on that packed integer as if it were a single number. For example, we can fit 8 byte sized numbers in a single word.
* By operating on the packed integer, we are in effect operating on 8 different integers in parallel.

This is what is called `world level parallelism`.

## Algorithms

The Algorithms implemented include:

### Finding the `top(k)` bits of an integer

The first procedure is quite simple. The goal is, given a number `x` and a length `k`, to extract the first `k` bits of `x` in `O(1)`. A procedure that does this will be handy when implementing the x-fast trie.

### The `SardineCan` Structure

Suppose we wish to maintain a set of small sized integers in a B-Tree. And suppose too that we wish to take advantage of the fact that we can fit many of these integers in a single, larger integer. How would we go about designing a single node in such a B-Tree?

Recall that a B-Tree of order `b` is a multi-way search tree in which each node is a bucket that must contain between `b - 1` and `2b - 1` keys. Furthermore, each node has one more child than the number of keys it contains. That is, each node must have between `b` and `2b` child nodes. Operations on B-Trees rely on one key operation: `node.rank(x)`. This operation searches through the keys of a single node (which are sorted) and either returns the location of `x` in the node, or the index of the child we need to descend into in order to complete the operation at hand. In run of the mill B-Trees, `node.rank(x)` is implemented using binary search and thus takes `O(lg b)`. However, if our keys are small integers, we can perform `node.rank(x)` in `O(1)`.

The `SardineCan` implements a B-Tree Node specialized for storing small integers.

### `O(1)` Most Significant Bit: FourRussiansMSB

When we talk of the most significant bit of a number, we're often referring to the 0-indexed location of the highest bit set. Note that this is a more general problem than simply finding the number that would be formed if only the `msb` were set. For instance, `MSB(010010001)` is `7` and not `128`.

The simplest method for finding this index in by doing a linear scan over the bits of the number in question while keeping a count of the number of bits seen thus far. This scheme runs in `O(lg n)` where `n` is the highest number our function may operate on.

```rust
/// A procedure for finding the index of the most significant
/// bit in time linear to the number of bits used
/// to represent the value.
fn get_msb_idx_of(query: u64) -> u8 {
    for i in (0..64).rev() {
        if query & (1 << i) != 0 {
            return i;
        }
    }
    panic!("MSB(0) is undefined")
}
```

We can improve upon the linear scanning procedure using bit level binary search. This brings down the running time to `O(lg lg n)`. Often, however, when we know that we'll be doing many `msb` queries, we use a lookup table to compute this value. Using that method, we're able to locate the index of the highest bit set in constant  `O(1)` time, albeit with an added preprocessing step to build the lookup table.

We can, using bit level parallelism, locate the index of the most significant bit in constant time without using a lookup table.

## References

1. [CS 166 Lecture 15](http://web.stanford.edu/class/archive/cs/cs166/cs166.1196/lectures/15/Slides15.pdf)
2. [CS 166 Lecture 16](http://web.stanford.edu/class/archive/cs/cs166/cs166.1196/lectures/16/Slides16.pdf)
3. [CS 166 Lecture 17](http://web.stanford.edu/class/archive/cs/cs166/cs166.1196/lectures/17/Slides17.pdf)
4. [6.851](http://courses.csail.mit.edu/6.851/fall17/scribe/lec12.pdf)
5. [The Original Fusion Tree Paper](https://reader.elsevier.com/reader/sd/pii/0022000093900404?token=1610EF62181DAC974715067B85459A4709A9BC64E39827CE0369C6C8E18540DFD1DBAD38BEE35BFF95C4C05E45A1D1D5)
6. [This StackOverflow Question. Scroll down until you find the answer by user `templatetypedef`](https://stackoverflow.com/questions/3878320/understanding-fusion-trees)

[crates-badge]: https://img.shields.io/crates/v/bit-parallelism.svg
[crates-url]: https://crates.io/crates/bit-parallelism
