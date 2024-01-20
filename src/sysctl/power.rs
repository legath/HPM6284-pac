#[doc = r"Register block"]
#[repr(C)]
pub struct POWER {
    status: STATUS,
    lf_wait: LF_WAIT,
    _reserved2: [u8; 0x04],
    off_wait: OFF_WAIT,
}
impl POWER {
    #[doc = "0x00 - Power Setting"]
    #[inline(always)]
    pub const fn status(&self) -> &STATUS {
        &self.status
    }
    #[doc = "0x04 - Power Setting"]
    #[inline(always)]
    pub const fn lf_wait(&self) -> &LF_WAIT {
        &self.lf_wait
    }
    #[doc = "0x0c - Power Setting"]
    #[inline(always)]
    pub const fn off_wait(&self) -> &OFF_WAIT {
        &self.off_wait
    }
}
#[doc = "status (rw) register accessor: Power Setting\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`status::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`status::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@status`]
module"]
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
#[doc = "Power Setting"]
pub mod status;
#[doc = "lf_wait (rw) register accessor: Power Setting\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lf_wait::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lf_wait::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lf_wait`]
module"]
pub type LF_WAIT = crate::Reg<lf_wait::LF_WAIT_SPEC>;
#[doc = "Power Setting"]
pub mod lf_wait;
#[doc = "off_wait (rw) register accessor: Power Setting\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`off_wait::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`off_wait::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@off_wait`]
module"]
pub type OFF_WAIT = crate::Reg<off_wait::OFF_WAIT_SPEC>;
#[doc = "Power Setting"]
pub mod off_wait;
