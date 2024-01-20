#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    key: [KEY; 2],
    ecc: [ECC; 2],
    select: SELECT,
}
impl RegisterBlock {
    #[doc = "0x00..0x40 - no description available"]
    #[inline(always)]
    pub const fn key(&self, n: usize) -> &KEY {
        &self.key[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x00..0x40 - no description available"]
    #[inline(always)]
    pub fn key_iter(&self) -> impl Iterator<Item = &KEY> {
        self.key.iter()
    }
    #[doc = "0x40..0x48 - no description available"]
    #[inline(always)]
    pub const fn ecc(&self, n: usize) -> &ECC {
        &self.ecc[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x40..0x48 - no description available"]
    #[inline(always)]
    pub fn ecc_iter(&self) -> impl Iterator<Item = &ECC> {
        self.ecc.iter()
    }
    #[doc = "0x40 - no description available"]
    #[inline(always)]
    pub const fn ecckey0(&self) -> &ECC {
        self.ecc(0)
    }
    #[doc = "0x44 - no description available"]
    #[inline(always)]
    pub const fn ecckey1(&self) -> &ECC {
        self.ecc(1)
    }
    #[doc = "0x48 - Key selection"]
    #[inline(always)]
    pub const fn select(&self) -> &SELECT {
        &self.select
    }
}
#[doc = "no description available"]
pub use self::key::KEY;
#[doc = r"Cluster"]
#[doc = "no description available"]
pub mod key;
#[doc = "ECC (rw) register accessor: no description available\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ecc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ecc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ecc`]
module"]
pub type ECC = crate::Reg<ecc::ECC_SPEC>;
#[doc = "no description available"]
pub mod ecc;
#[doc = "SELECT (rw) register accessor: Key selection\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`select::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`select::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@select`]
module"]
pub type SELECT = crate::Reg<select::SELECT_SPEC>;
#[doc = "Key selection"]
pub mod select;
