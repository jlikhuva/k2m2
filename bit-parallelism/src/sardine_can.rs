//! # The `SardineCan` Structure
//!
//! Suppose we wish to maintain a set of small sized integers in a B-Tree. 
//! And suppose too that we wish to take advantage of the fact that we can fit many of 
//! these integers in a single, larger integer. How would we go about designing a single node in such a B-Tree?
//!
//! Recall that a B-Tree of order `b` is a multi-way search tree in which each node is a bucket 
//! that must contain between `b - 1` and `2b - 1` keys. Furthermore, each node has one more child 
//! than the number of keys it contains. That is, each node must have between `b` and `2b` child nodes. 
//! 
//! Operations on B-Trees rely on one key operation: `node.rank(x)`.
//!  This operation searches through the keys of a single node (which are sorted) and either returns 
//! the location of `x` in the node, or the index of the child we need to descend into in order 
//! to complete the operation at hand. 
//! 
//! In run of the mill B-Trees, `node.rank(x)` is implemented 
//! using binary search and thus takes `O(lg b)`. However, if our keys are small integers, 
//! we can perform `node.rank(x)` in `O(1)`.
//!
//! The `SardineCan` implements a B-Tree Node specialized for storing small integers.

/// The abstraction for a single node in our b-tree
/// that is specialized for holding small integers
/// that can be packed into a single machine word
#[derive(Debug, Default)]
pub struct SardineCan {
    /// The actual storage container
    buckets: u64,

    /// The count of items in this node.
    count: u8,
}

impl std::fmt::Display for SardineCan {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let res = format!("{:b}", self.buckets);
        writeln!(f, "{}", res)
    }
}

impl SardineCan {
    /// Procedure to store a single small integer in a given node
    /// Note that we do not handle the case where a can could be full.
    /// We ignore that because, ideally, this data structure would be part
    /// of a larger B-Tree implementation that would take care of such details
    pub fn add(&mut self, mut x: u8) {
        // Add the sentinel bit. It is set to 0
        x &= 0b0111_1111;

        // Make space in the bucket for the new item
        self.buckets <<= 8;

        // Add the new item into the bucket
        self.buckets |= x as u64;

        // Increment the count of items
        self.count += 1;
    }

    /// Produces a number that is the result of replicating `x`
    /// as many times to produce a value with as many bits as
    /// the bits in `buckets`
    pub fn parallel_tile_64(query: u8) -> u64 {
        // This carefully chosen multiplier will have the desired effect of replicating `x`
        // seven times, interspersing each instance of `x` with a 0
        let multiplier: u64 = 0b10000000_10000000_10000000_10000000_10000000_10000000_100000001;

        // Produce the provisional tiled number. We still need to set its
        // sentinel bits to 1
        let tiled_x = query as u64 * multiplier;

        // The bitmask to turn on  the sentinel bits
        let sentinel_mask: u64 =
            0b10000000_10000000_10000000_10000000_10000000_10000000_1000000010000000;

        // Set the sentinel bits to 1 and return the tiled number
        tiled_x | sentinel_mask
    }

    /// Calculate how many items in this can are less than or
    /// equal to `x`
    pub fn parallel_rank(&self, x: u8) -> u8 {
        Self::parallel_rank_helper(self.buckets, x)
    }

    fn parallel_rank_helper(packed_keys: u64, query: u8) -> u8 {
        // Perform the parallel comparison
        let mut difference = Self::parallel_tile_64(query) - packed_keys;

        // Ultimately, we're only interested in whether the spacer sentinel bits
        // are turned on or off. In particular, we just need to know how many are
        // turned on. Here we use the mask from `parallel_tile` to isolate them
        let sentinel_mask: u64 =
            0b10000000_10000000_10000000_10000000_10000000_10000000_1000000010000000;
        difference &= sentinel_mask;

        // There's an alternative method of counting up how many spacer bits are set to 1.
        // That method involves using a well chosen multiplier. To check it out look in
        // at the  `parallel_count` method below
        difference.count_ones() as u8
    }

    /// Counts up how many of the sentinel bits of `difference` are turned on
    pub fn parallel_count(difference: u64) -> u8 {
        let stacker = 0b10000000_10000000_10000000_10000000_10000000_10000000_100000001u64;
        let mut stacked = difference as u128 * stacker as u128;
        stacked >>= 63;
        stacked &= 0b111;
        stacked as u8
    }
}
