//! Reference: https://github.com/0xmenna/nitro/blob/master/arbitrator/prover/src/programs/memory.rs

pub const INITIAL_FREE_PAGES: u16 = 2;
pub const INITIAL_PAGE_GAS: u16 = 1000;

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct MemoryModel {
    /// Number of pages a tx gets for free
    pub free_pages: u16,
    /// Base cost of each additional wasm page
    pub page_gas: u16,
}

impl Default for MemoryModel {
    fn default() -> Self {
        Self {
            free_pages: INITIAL_FREE_PAGES,
            page_gas: INITIAL_PAGE_GAS,
        }
    }
}

impl MemoryModel {
    pub const fn new(free_pages: u16, page_gas: u16) -> Self {
        Self {
            free_pages,
            page_gas,
        }
    }

    /// Determines the gas cost of allocating `new` pages given `open` are active and `ever` have ever been.
    pub fn gas_cost(&self, new: u16, open: u16, ever: u16) -> u64 {
        let new_open = open.saturating_add(new);
        let new_ever = ever.max(new_open);

        // free until expansion beyond the first few
        if new_ever <= self.free_pages {
            return 0;
        }

        let credit = |pages: u16| pages.saturating_sub(self.free_pages);
        let adding = credit(new_open).saturating_sub(credit(open)) as u64;
        let linear = adding.saturating_mul(self.page_gas.into());
        let expand = Self::exp(new_ever) - Self::exp(ever);
        linear.saturating_add(expand)
    }

    fn exp(pages: u16) -> u64 {
        MEMORY_EXPONENTS
            .get(pages as usize)
            .map(|&x| x.into())
            .unwrap_or(u64::MAX)
    }
}

const MEMORY_EXPONENTS: [u32; 129] = [
    1, 1, 1, 1, 1, 1, 2, 2, 2, 3, 3, 4, 5, 5, 6, 7, 8, 9, 11, 12, 14, 17, 19, 22, 25, 29, 33, 38,
    43, 50, 57, 65, 75, 85, 98, 112, 128, 147, 168, 193, 221, 253, 289, 331, 379, 434, 497, 569,
    651, 745, 853, 976, 1117, 1279, 1463, 1675, 1917, 2194, 2511, 2874, 3290, 3765, 4309, 4932,
    5645, 6461, 7395, 8464, 9687, 11087, 12689, 14523, 16621, 19024, 21773, 24919, 28521, 32642,
    37359, 42758, 48938, 56010, 64104, 73368, 83971, 96106, 109994, 125890, 144082, 164904, 188735,
    216010, 247226, 282953, 323844, 370643, 424206, 485509, 555672, 635973, 727880, 833067, 953456,
    1091243, 1248941, 1429429, 1636000, 1872423, 2143012, 2452704, 2807151, 3212820, 3677113,
    4208502, 4816684, 5512756, 6309419, 7221210, 8264766, 9459129, 10826093, 12390601, 14181199,
    16230562, 18576084, 21260563, 24332984, 27849408, 31873999,
];
