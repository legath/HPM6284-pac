#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    reset_flag: RESET_FLAG,
    reset_status: RESET_STATUS,
    reset_hold: RESET_HOLD,
    reset_enable: RESET_ENABLE,
    reset_hot: RESET_HOT,
    reset_cold: RESET_COLD,
    _reserved6: [u8; 0x04],
    software_reset: SOFTWARE_RESET,
}
impl RegisterBlock {
    #[doc = "0x00 - flag indicate reset source"]
    #[inline(always)]
    pub const fn reset_flag(&self) -> &RESET_FLAG {
        &self.reset_flag
    }
    #[doc = "0x04 - reset source status"]
    #[inline(always)]
    pub const fn reset_status(&self) -> &RESET_STATUS {
        &self.reset_status
    }
    #[doc = "0x08 - reset hold attribute"]
    #[inline(always)]
    pub const fn reset_hold(&self) -> &RESET_HOLD {
        &self.reset_hold
    }
    #[doc = "0x0c - reset source enable"]
    #[inline(always)]
    pub const fn reset_enable(&self) -> &RESET_ENABLE {
        &self.reset_enable
    }
    #[doc = "0x10 - reset type triggered by reset"]
    #[inline(always)]
    pub const fn reset_hot(&self) -> &RESET_HOT {
        &self.reset_hot
    }
    #[doc = "0x14 - reset type attribute"]
    #[inline(always)]
    pub const fn reset_cold(&self) -> &RESET_COLD {
        &self.reset_cold
    }
    #[doc = "0x1c - Software reset counter"]
    #[inline(always)]
    pub const fn software_reset(&self) -> &SOFTWARE_RESET {
        &self.software_reset
    }
}
#[doc = "RESET_FLAG (rw) register accessor: flag indicate reset source\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`reset_flag::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`reset_flag::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reset_flag`]
module"]
pub type RESET_FLAG = crate::Reg<reset_flag::RESET_FLAG_SPEC>;
#[doc = "flag indicate reset source"]
pub mod reset_flag;
#[doc = "RESET_STATUS (rw) register accessor: reset source status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`reset_status::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`reset_status::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reset_status`]
module"]
pub type RESET_STATUS = crate::Reg<reset_status::RESET_STATUS_SPEC>;
#[doc = "reset source status"]
pub mod reset_status;
#[doc = "RESET_HOLD (rw) register accessor: reset hold attribute\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`reset_hold::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`reset_hold::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reset_hold`]
module"]
pub type RESET_HOLD = crate::Reg<reset_hold::RESET_HOLD_SPEC>;
#[doc = "reset hold attribute"]
pub mod reset_hold;
#[doc = "RESET_ENABLE (rw) register accessor: reset source enable\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`reset_enable::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`reset_enable::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reset_enable`]
module"]
pub type RESET_ENABLE = crate::Reg<reset_enable::RESET_ENABLE_SPEC>;
#[doc = "reset source enable"]
pub mod reset_enable;
#[doc = "RESET_HOT (rw) register accessor: reset type triggered by reset\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`reset_hot::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`reset_hot::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reset_hot`]
module"]
pub type RESET_HOT = crate::Reg<reset_hot::RESET_HOT_SPEC>;
#[doc = "reset type triggered by reset"]
pub mod reset_hot;
#[doc = "RESET_COLD (rw) register accessor: reset type attribute\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`reset_cold::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`reset_cold::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reset_cold`]
module"]
pub type RESET_COLD = crate::Reg<reset_cold::RESET_COLD_SPEC>;
#[doc = "reset type attribute"]
pub mod reset_cold;
#[doc = "SOFTWARE_RESET (rw) register accessor: Software reset counter\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`software_reset::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`software_reset::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@software_reset`]
module"]
pub type SOFTWARE_RESET = crate::Reg<software_reset::SOFTWARE_RESET_SPEC>;
#[doc = "Software reset counter"]
pub mod software_reset;
