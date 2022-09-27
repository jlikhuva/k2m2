//! # Word Level Parallelism
//! 
//! Bit level algorithms for developing useful when developing specialized integer data structures such as the [x-fast trie](http://web.stanford.edu/class/archive/cs/cs166/cs166.1166/lectures/15/Small15.pdf)
//! 
//! ## Overview
//! 
//! Arithmetic and logical operations take, for all intents and purposes, constant time. Such operations operate on whole words. (A word is the size of a single memory segment. This crate, assumes a word size width of `64`. For a more in-depth discussion of computer memory, refer to [this note](https://akkadia.org/drepper/cpumemory.pdf)). For instance, it takes constant time to add two `64` bit numbers.
//! 
//! The central idea behind the algorithms in this library  is this:
//! 
//! * If you have a bunch of small integers — each smaller that sixty four bits, e.g. a bunch of bytes, we can pack many of them into a single sixty four bit integer.
//! * We can then operate on that packed integer as if it were a single number. For example, we can fit 8 byte sized numbers in a single word.
//! * By operating on the packed integer, we are in effect operating on 8 different integers in parallel. 
//! 
//! This is what is called `world level parallelism`.
//! 
//! ## Algorithms
//! 
//! The Algorithms implemented include:
//! 
//! ### Finding the `top(k)` bits of an integer
//! 
//! The first procedure is quite simple. The goal is, given a number `x` and a length `k`, to extract the first `k` bits of `x` in `O(1)`. A procedure that does this will be handy when implementing the x-fast trie.
//! 
//! ### The `SardineCan` Structure
//! 
//! Suppose we wish to maintain a set of small sized integers in a B-Tree. And suppose too that we wish to take advantage of the fact that we can fit many of these integers in a single, larger integer. How would we go about designing a single node in such a B-Tree?
//! 
//! Recall that a B-Tree of order `b` is a multi-way search tree in which each node is a bucket that must contain between `b - 1` and `2b - 1` keys. Furthermore, each node has one more child than the number of keys it contains. That is, each node must have between `b` and `2b` child nodes. Operations on B-Trees rely on one key operation: `node.rank(x)`. This operation searches through the keys of a single node (which are sorted) and either returns the location of `x` in the node, or the index of the child we need to descend into in order to complete the operation at hand. In run of the mill B-Trees, `node.rank(x)` is implemented using binary search and thus takes `O(lg b)`. However, if our keys are small integers, we can perform `node.rank(x)` in `O(1)`.
//! 
//! The `SardineCan` implements a B-Tree Node specialized for storing small integers.
//! 
//! ### `O(1)` Most Significant Bit: FourRussiansMSB
//! 
//! When we talk of the most significant bit of a number, we're often referring to the 0-indexed location of the highest bit set. Note that this is a more general problem than simply finding the number that would be formed if only the `msb` were set. For instance, `MSB(010010001)` is `7` and not `128`.
//! 
//! The simplest method for finding this index in by doing a linear scan over the bits of the number in question while keeping a count of the number of bits seen thus far. This scheme runs in `O(lg n)` where `n` is the highest number our function may operate on.
//! 
//! ```rust
//! /// A procedure for finding the index of the most significant
//! /// bit in time linear to the number of bits used
//! /// to represent the value.
//! fn get_msb_idx_of(query: u64) -> u8 {
//!     for i in (0..64).rev() {
//!         if query & (1 << i) != 0 {
//!             return i;
//!         }
//!     }
//!     panic!("MSB(0) is undefined")
//! }
//! ```
//! 
//! We can improve upon the linear scanning procedure using bit level binary search. This brings down the running time to `O(lg lg n)`. Often, however, when we know that we'll be doing many `msb` queries, we use a lookup table to compute this value. Using that method, we're able to locate the index of the highest bit set in constant  `O(1)` time, albeit with an added preprocessing step to build the lookup table.
//! 
//! We can, using bit level parallelism, locate the index of the most significant bit in constant time without using a lookup table.
//! 
//! ## References
//! 
//! 1. [CS 166 Lecture 15](http://web.stanford.edu/class/archive/cs/cs166/cs166.1196/lectures/15/Slides15.pdf)
//! 2. [CS 166 Lecture 16](http://web.stanford.edu/class/archive/cs/cs166/cs166.1196/lectures/16/Slides16.pdf)
//! 3. [CS 166 Lecture 17](http://web.stanford.edu/class/archive/cs/cs166/cs166.1196/lectures/17/Slides17.pdf)
//! 4. [6.851](http://courses.csail.mit.edu/6.851/fall17/scribe/lec12.pdf)
//! 5. [The Original Fusion Tree Paper](https://reader.elsevier.com/reader/sd/pii/0022000093900404?token=1610EF62181DAC974715067B85459A4709A9BC64E39827CE0369C6C8E18540DFD1DBAD38BEE35BFF95C4C05E45A1D1D5)
//! 6. [This StackOverflow Question. Scroll down until you find the answer by user `templatetypedef`](https://stackoverflow.com/questions/3878320/understanding-fusion-trees)
//!

// Test that pointer width is compatible. This asserts that usize is 64 bits, 
// which a lot of algorithms in this crate currently assume.
#[cfg(not(any(
    target_pointer_width = "64",
)))]
compile_error! {
    "This crate requires the platform pointer to have a width of 64"
}

pub mod sardine_can;
pub mod four_russians_msb;

const USIZE_BITS: usize = 64;

///  Given a number `x` and a length k, extract the first k bits of x in O(1). 
pub fn top_k_bits_of(x: usize, k: usize) -> usize {
    assert!(k != 0);
    let mut mask: usize = 1;

    // Shift the 1 to the index that is `k`
    // positions from the last index location.
    // That is `k` away from 64
    mask <<= USIZE_BITS - k;

    // Turn that one into a zero. And all
    // the other 63 zeros into ones.
    mask = !mask;

    // I think this is the most interesting/entertaining part.
    // Adding a one triggers a cascade of carries that flip all
    // the bits (all ones) before the location of the zero from above into
    // zeros. The cascade stops when they reach the zero from
    // above. Since it is a zero, adding a 1 does not trigger a carry
    //
    // In the end, we have a mask where the top k bits are ones
    mask += 1;

    // This is straightforward
    x & mask
}



#[cfg(test)]
mod test_bit_parallelism {
    use rand::Rng;
    use pretty_assertions::assert_eq;

    use super::sardine_can;
    use super::four_russians_msb;
    

    #[test]
    fn sardine_add() {
        let mut rng = rand::thread_rng();
        let mut can = sardine_can::SardineCan::default();
        for _ in 0..8 {
            let small_int = rng.gen_range(0..=1 << 7);
            can.add(small_int);
            println!("{:b}, can is {}", small_int, can)
        }
        //_11101110_10101110_11111000_11001101_10101111_10001101_11110111_11100001
        //_01010110_00111110_00111110_01000011_00011011_00101111_00100011_01111010
        //1100111
    }

    #[test]
    fn sardine_tile() {
        let tiled = sardine_can::SardineCan::parallel_tile_64(0b1100111);
        println!("{:b}", tiled)
        // 1100111_01100111_01100111_01100111
        // 01100111_01100111_01100111_01100111_01100111_01100111_01100111_01100111
        // 11100111_11100111_11100111_11100111_11100111_11100111_11100111_11100111
    }

    #[test]
    fn test_stacker() {
        // Test alternative method of computing rank
        let a = 0b10000000_10000000_10000000_10000000_10000000_10000000_100000001u64;
        let b = 0b10000000_00000000_10000000_10000000_00000000_10000000_00000000_10000000u64;
        let mut c = a as u128 * b as u128;
        println!("{:b}", c);
        c >>= 63;
        println!("{:b}", c);
        println!("{}", c & 0b111);
    }

    #[test]
    fn sardine_rank() {
        let mut rng = rand::thread_rng();
        let mut can = sardine_can::SardineCan::default();
        for _ in 0..8 {
            let small_int = rng.gen_range(0..=1 << 7);
            can.add(small_int);
        }
        println!("{}", can.parallel_rank(0b1100111));
        // _10000000_00000000_10000000_10000000_00000000_10000000_00000000_10000000
        // 10000000_10000000_10000000_10000000_10000000_10000000_100000001
    }

    #[test]
    fn pack() {
        let tt = 0b00010000_10000000_10000000_10000000_10000000_00000000_00000000_00000000u64;
        let m = 0b10000001_00000010_00000100_00001000_00010000_00100000_010000001u64;
        let mut c = tt as u128 * m as u128;
        println!("{:b}", c);
        c >>= 49;
        println!("{:b}", c);
        if tt >> 56 == 0 {
            c &= 0b0111_1111;
        } else {
            c |= 0b1000_0000;
            c &= 0b1111_1111;
        }
        println!("{:b}", c);
        // 100000_01100000_11100001_11100011_11000111_10001111_00011110_001111000
        // 100000_10000001_01000010_11000101_11001011_10010111_00101110_01011100_01111000
    }

    #[test]
    fn get_msb() {
        let msb = four_russians_msb::get_msb_idx_of(873);
        assert_eq!(9, msb);
        let base: usize = 2;
        let msb = four_russians_msb::get_msb_idx_of(base.pow(32) as u64);
        assert_eq!(32, msb);
        let msb = four_russians_msb::get_msb_idx_of(base.pow(55) as u64);
        assert_eq!(55, msb);
        let msb = four_russians_msb::get_msb_idx_of((base.pow(56) + 13) as u64);
        assert_eq!(56, msb);
        let msb = four_russians_msb::get_msb_idx_of((base.pow(61) + 31) as u64);
        assert_eq!(61, msb);
        let msb = four_russians_msb::get_msb_idx_of((2u128.pow(64) - 1) as u64);
        assert_eq!(63, msb);
        let msb = four_russians_msb::get_msb_idx_of(base.pow(48) as u64);
        assert_eq!(48, msb);
        let msb = four_russians_msb::get_msb_idx_of(base.pow(63) as u64);
        assert_eq!(63, msb);
        let msb = four_russians_msb::get_msb_idx_of(255);
        assert_eq!(7, msb);
        let msb = four_russians_msb::get_msb_idx_of(1);
        assert_eq!(0, msb);
        let msb = four_russians_msb::get_msb_idx_of(16);
        assert_eq!(4, msb);
        let msb = four_russians_msb::get_msb_idx_of(256);
        assert_eq!(8, msb);
        let msb = four_russians_msb::get_msb_idx_of(25);
        assert_eq!(4, msb);
        let msb = four_russians_msb::get_msb_idx_of(91);
        assert_eq!(6, msb);
        let msb = four_russians_msb::get_msb_idx_of(base.pow(16) as u64);
        assert_eq!(16, msb);
        let msb = four_russians_msb::get_msb_idx_of(1 << 18);
        assert_eq!(18, msb);
    }
}
