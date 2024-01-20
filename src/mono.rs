#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    monol: MONOL,
    monoh: MONOH,
}
impl RegisterBlock {
    #[doc = "0x00 - Low part of monotonic counter"]
    #[inline(always)]
    pub const fn monol(&self) -> &MONOL {
        &self.monol
    }
    #[doc = "0x04 - High part of monotonic counter"]
    #[inline(always)]
    pub const fn monoh(&self) -> &MONOH {
        &self.monoh
    }
}
#[doc = "MONOL (rw) register accessor: Low part of monotonic counter\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`monol::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`monol::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@monol`]
module"]
pub type MONOL = crate::Reg<monol::MONOL_SPEC>;
#[doc = "Low part of monotonic counter"]
pub mod monol;
#[doc = "MONOH (rw) register accessor: High part of monotonic counter\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`monoh::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`monoh::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@monoh`]
module"]
pub type MONOH = crate::Reg<monoh::MONOH_SPEC>;
#[doc = "High part of monotonic counter"]
pub mod monoh;
