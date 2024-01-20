#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    mtime: MTIME,
    mtimecmp: MTIMECMP,
}
impl RegisterBlock {
    #[doc = "0x00..0x08 - Machine Time"]
    #[inline(always)]
    pub const fn mtime(&self) -> &MTIME {
        &self.mtime
    }
    #[doc = "0x08..0x10 - Machine Time Compare"]
    #[inline(always)]
    pub const fn mtimecmp(&self) -> &MTIMECMP {
        &self.mtimecmp
    }
}
#[doc = "MTIME (rw) register accessor: Machine Time\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mtime::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mtime::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mtime`]
module"]
pub type MTIME = crate::Reg<mtime::MTIME_SPEC>;
#[doc = "Machine Time"]
pub mod mtime;
#[doc = "MTIMECMP (rw) register accessor: Machine Time Compare\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mtimecmp::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mtimecmp::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mtimecmp`]
module"]
pub type MTIMECMP = crate::Reg<mtimecmp::MTIMECMP_SPEC>;
#[doc = "Machine Time Compare"]
pub mod mtimecmp;
