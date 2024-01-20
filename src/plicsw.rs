#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x1000],
    pending: PENDING,
    _reserved1: [u8; 0x0ffc],
    inten: INTEN,
    _reserved2: [u8; 0x001f_e000],
    claim: CLAIM,
}
impl RegisterBlock {
    #[doc = "0x1000 - Pending status"]
    #[inline(always)]
    pub const fn pending(&self) -> &PENDING {
        &self.pending
    }
    #[doc = "0x2000 - Interrupt enable"]
    #[inline(always)]
    pub const fn inten(&self) -> &INTEN {
        &self.inten
    }
    #[doc = "0x200004 - Claim and complete."]
    #[inline(always)]
    pub const fn claim(&self) -> &CLAIM {
        &self.claim
    }
}
#[doc = "PENDING (rw) register accessor: Pending status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pending::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pending::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pending`]
module"]
pub type PENDING = crate::Reg<pending::PENDING_SPEC>;
#[doc = "Pending status"]
pub mod pending;
#[doc = "INTEN (rw) register accessor: Interrupt enable\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`inten::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`inten::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@inten`]
module"]
pub type INTEN = crate::Reg<inten::INTEN_SPEC>;
#[doc = "Interrupt enable"]
pub mod inten;
#[doc = "CLAIM (rw) register accessor: Claim and complete.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`claim::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`claim::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@claim`]
module"]
pub type CLAIM = crate::Reg<claim::CLAIM_SPEC>;
#[doc = "Claim and complete."]
pub mod claim;
