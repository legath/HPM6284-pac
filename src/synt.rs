#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    gcr: GCR,
    rld: RLD,
    _reserved2: [u8; 0x04],
    cnt: CNT,
    _reserved3: [u8; 0x10],
    cmp: [CMP; 4],
}
impl RegisterBlock {
    #[doc = "0x00 - Global control register"]
    #[inline(always)]
    pub const fn gcr(&self) -> &GCR {
        &self.gcr
    }
    #[doc = "0x04 - Counter reload register"]
    #[inline(always)]
    pub const fn rld(&self) -> &RLD {
        &self.rld
    }
    #[doc = "0x0c - Counter"]
    #[inline(always)]
    pub const fn cnt(&self) -> &CNT {
        &self.cnt
    }
    #[doc = "0x20..0x30 - no description available"]
    #[inline(always)]
    pub const fn cmp(&self, n: usize) -> &CMP {
        &self.cmp[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x20..0x30 - no description available"]
    #[inline(always)]
    pub fn cmp_iter(&self) -> impl Iterator<Item = &CMP> {
        self.cmp.iter()
    }
}
#[doc = "gcr (rw) register accessor: Global control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gcr`]
module"]
pub type GCR = crate::Reg<gcr::GCR_SPEC>;
#[doc = "Global control register"]
pub mod gcr;
#[doc = "rld (rw) register accessor: Counter reload register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rld::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rld::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rld`]
module"]
pub type RLD = crate::Reg<rld::RLD_SPEC>;
#[doc = "Counter reload register"]
pub mod rld;
#[doc = "cnt (rw) register accessor: Counter\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cnt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cnt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cnt`]
module"]
pub type CNT = crate::Reg<cnt::CNT_SPEC>;
#[doc = "Counter"]
pub mod cnt;
#[doc = "CMP (rw) register accessor: no description available\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmp::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmp::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmp`]
module"]
pub type CMP = crate::Reg<cmp::CMP_SPEC>;
#[doc = "no description available"]
pub mod cmp;
