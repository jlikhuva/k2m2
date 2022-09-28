//! # `O(1)` Most Significant Bit: FourRussiansMSB
//!
//! When we talk of the most significant bit of a number, we're often referring to the 0-indexed location of the highest bit set. Note that this is a more general problem than simply finding the number that would be formed if only the `msb` were set. For instance, `MSB(010010001)` is `7` and not `128`.
//!
//! The simplest method for finding this index in by doing a linear scan over the bits of the number in question while keeping a count of the number of bits seen thus far. This scheme runs in `O(lg n)` where `n` is the highest number our function may operate on.
//!
//! ```rust
//! /// A procedure for finding the index of the most significant
//! /// bit in time linear to the number of bits used
//! /// to represent the value.
//!
//!
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

#[derive(Debug)]
pub struct FourRussiansMSB {
    /// The secondary routing bit array
    macro_bit_array: u8,

    /// This is simply the number whose `msb` we'd like to find.
    /// It is logically split into blocks of 8 bits
    micro_arrays: u64,
}

impl FourRussiansMSB {
    pub fn build(query: u64) -> Self {
        let macro_bit_array = Self::generate_macro_bit_array(query);
        FourRussiansMSB {
            macro_bit_array,
            micro_arrays: query,
        }
    }

    /// Generates the routing macro array. To do so, it
    /// relies on the observation that a block contains a
    /// 1 bit if it's highest bit is a 1 or if its
    /// lower 7 bits' numeric value is greater than 0.
    fn generate_macro_bit_array(query: u64) -> u8 {
        // The first step is to extract information about the highest bit in each block.
        let high_bit_mask =
            0b10000000_10000000_10000000_10000000_10000000_10000000_10000000_10000000u64;
        let is_high_bit_set = query & high_bit_mask;

        // The second step is to extract information about the lower seven bits
        // in each block. To do so, we use parallel_compare, which is basically
        // subtraction.
        let packed_ones =
            0b00000001_00000001_00000001_00000001_00000001_00000001_00000001_00000001u64;
        let mut are_lower_bits_set = query | high_bit_mask;
        are_lower_bits_set -= packed_ones;
        are_lower_bits_set &= high_bit_mask;

        // We unify the information from the first two steps into a single value
        // that tells us if a block could conceivably contain the MSB
        let is_block_active = is_high_bit_set | are_lower_bits_set;

        // To generate the macro array, we need to form an 8-bit number out of the
        // per-block highest bits from the last step. To pack them together, we simply use
        // an appropriate multiplier which does the work of a series of bitshifts
        let packer = 0b10000001_00000010_00000100_00001000_00010000_00100000_010000001u64;
        let mut macro_bit_array = is_block_active as u128 * packer as u128;
        macro_bit_array >>= 49;
        if is_block_active >> 56 == 0 {
            macro_bit_array &= 0b0111_1111;
        } else {
            macro_bit_array |= 0b1000_0000;
            macro_bit_array &= 0b1111_1111;
        }
        macro_bit_array as u8
    }

    pub fn get_msb(&self) -> u8 {
        let block_id = self.msb_by_rank(self.macro_bit_array);
        let block_start = (block_id - 1) * 8;
        let msb_block = self.get_msb_block(block_start); // msb block is wrong!!
        let msb = self.msb_by_rank(msb_block);
        let in_block_location = msb - 1;
        block_start + in_block_location
    }

    /// Given a block id -- which is the msb value in the macro routing array,
    /// this method retrieves the 8 bits that represent that block
    /// from the `micro_arrays`. `block_id 0 refers to the highest
    fn get_msb_block(&self, block_start: u8) -> u8 {
        let block_mask = 0b1111_1111u64;
        let mut block = self.micro_arrays >> block_start;
        block &= block_mask;
        block as u8
    }

    /// Finds the index of the most significant bit in the
    /// provided 8-bit number by finding its rank among the
    /// 8 possible powers of 2: <1, 2, 4, 8, 16, 32, 64, 128>.
    /// To do so in constant time, it employs techniques from
    /// our discussion of `parallel_rank`
    fn msb_by_rank(&self, query: u8) -> u8 {
        // Perform the parallel comparison
        let tiled_query = Self::parallel_tile_128(query);
        let packed_keys =
            0b000000001_000000010_000000100_000001000_000010000_000100000_001000000_010000000u128;
        let mut difference = tiled_query - packed_keys;

        // Isolate the spacer sentinel bits
        let sentinel_mask =
            0b100000000_100000000_100000000_100000000_100000000_100000000_100000000_100000000u128;
        difference &= sentinel_mask;

        // Count the number of spacer bits that are turned on
        difference.count_ones() as u8
    }

    /// Produces a number that is a result of replicating the query
    /// eight times. This uses 72 bits of space.
    pub fn parallel_tile_128(query: u8) -> u128 {
        let multiplier =
            0b100000000_100000000_100000000_100000000_100000000_100000000_100000000_1000000001u128;

        // Produce the provisional tiled number. We still need to set its
        // sentinel bits to 1
        let tiled_query = query as u128 * multiplier;

        // The bitmask to turn on  the sentinel bits
        let sentinel_mask =
            0b100000000_100000000_100000000_100000000_100000000_100000000_100000000_100000000u128;

        // Set the sentinel bits to 1 and return the tiled number
        tiled_query | sentinel_mask
    }
}

/// Returns the 0-based index of the query's most significant bit.
///
/// ```rust
///
/// use bit_parallelism::four_russians_msb::get_msb_idx_of;
///
/// let msb = get_msb_idx_of(873);
/// assert_eq!(9, msb);
/// let base: usize = 2;
/// let msb = get_msb_idx_of(base.pow(32) as u64);
/// assert_eq!(32, msb);
/// let msb = get_msb_idx_of(base.pow(55) as u64);
/// assert_eq!(55, msb);
/// let msb = get_msb_idx_of((base.pow(56) + 13) as u64);
/// assert_eq!(56, msb);
/// let msb = get_msb_idx_of((base.pow(61) + 31) as u64);
/// assert_eq!(61, msb);
/// ```
pub fn get_msb_idx_of(query: u64) -> u8 {
    FourRussiansMSB::build(query).get_msb()
}

/// `O(1) LCP(x, y)`
///
/// Finds the length of the longest common prefix between the bit-strings of the two numbers in constant time.
pub fn lcp_len_of(a: u64, b: u64) -> u64 {
    63 - get_msb_idx_of(a ^ b) as u64
}
