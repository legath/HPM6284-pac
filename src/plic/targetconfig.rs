#[doc = r"Register block"]
#[repr(C)]
pub struct TARGETCONFIG {
    threshold: THRESHOLD,
    claim: CLAIM,
    _reserved2: [u8; 0x03f8],
    pps: PPS,
}
impl TARGETCONFIG {
    #[doc = "0x00 - Target0 priority threshold"]
    #[inline(always)]
    pub const fn threshold(&self) -> &THRESHOLD {
        &self.threshold
    }
    #[doc = "0x04 - Target claim and complete"]
    #[inline(always)]
    pub const fn claim(&self) -> &CLAIM {
        &self.claim
    }
    #[doc = "0x400 - Preempted priority stack"]
    #[inline(always)]
    pub const fn pps(&self) -> &PPS {
        &self.pps
    }
}
#[doc = "THRESHOLD (rw) register accessor: Target0 priority threshold\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`threshold::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`threshold::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@threshold`]
module"]
pub type THRESHOLD = crate::Reg<threshold::THRESHOLD_SPEC>;
#[doc = "Target0 priority threshold"]
pub mod threshold;
#[doc = "CLAIM (rw) register accessor: Target claim and complete\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`claim::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`claim::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@claim`]
module"]
pub type CLAIM = crate::Reg<claim::CLAIM_SPEC>;
#[doc = "Target claim and complete"]
pub mod claim;
#[doc = "PPS (rw) register accessor: Preempted priority stack\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pps::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pps::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pps`]
module"]
pub type PPS = crate::Reg<pps::PPS_SPEC>;
#[doc = "Preempted priority stack"]
pub mod pps;
