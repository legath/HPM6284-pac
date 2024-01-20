#[doc = r"Register block"]
#[repr(C)]
pub struct PTPC {
    ctrl0: CTRL0,
    ctrl1: CTRL1,
    timeh: TIMEH,
    timel: TIMEL,
    ts_updth: TS_UPDTH,
    ts_updtl: TS_UPDTL,
    addend: ADDEND,
    tarh: TARH,
    tarl: TARL,
    _reserved9: [u8; 0x08],
    pps_ctrl: PPS_CTRL,
    capt_snaph: CAPT_SNAPH,
    capt_snapl: CAPT_SNAPL,
}
impl PTPC {
    #[doc = "0x00 - Control Register 0"]
    #[inline(always)]
    pub const fn ctrl0(&self) -> &CTRL0 {
        &self.ctrl0
    }
    #[doc = "0x04 - Control Register 1"]
    #[inline(always)]
    pub const fn ctrl1(&self) -> &CTRL1 {
        &self.ctrl1
    }
    #[doc = "0x08 - timestamp high"]
    #[inline(always)]
    pub const fn timeh(&self) -> &TIMEH {
        &self.timeh
    }
    #[doc = "0x0c - timestamp low"]
    #[inline(always)]
    pub const fn timel(&self) -> &TIMEL {
        &self.timel
    }
    #[doc = "0x10 - timestamp update high"]
    #[inline(always)]
    pub const fn ts_updth(&self) -> &TS_UPDTH {
        &self.ts_updth
    }
    #[doc = "0x14 - timestamp update low"]
    #[inline(always)]
    pub const fn ts_updtl(&self) -> &TS_UPDTL {
        &self.ts_updtl
    }
    #[doc = "0x18 - No description avaiable"]
    #[inline(always)]
    pub const fn addend(&self) -> &ADDEND {
        &self.addend
    }
    #[doc = "0x1c - No description avaiable"]
    #[inline(always)]
    pub const fn tarh(&self) -> &TARH {
        &self.tarh
    }
    #[doc = "0x20 - No description avaiable"]
    #[inline(always)]
    pub const fn tarl(&self) -> &TARL {
        &self.tarl
    }
    #[doc = "0x2c - No description avaiable"]
    #[inline(always)]
    pub const fn pps_ctrl(&self) -> &PPS_CTRL {
        &self.pps_ctrl
    }
    #[doc = "0x30 - No description avaiable"]
    #[inline(always)]
    pub const fn capt_snaph(&self) -> &CAPT_SNAPH {
        &self.capt_snaph
    }
    #[doc = "0x34 - No description avaiable"]
    #[inline(always)]
    pub const fn capt_snapl(&self) -> &CAPT_SNAPL {
        &self.capt_snapl
    }
}
#[doc = "Ctrl0 (rw) register accessor: Control Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl0`]
module"]
pub type CTRL0 = crate::Reg<ctrl0::CTRL0_SPEC>;
#[doc = "Control Register 0"]
pub mod ctrl0;
#[doc = "ctrl1 (rw) register accessor: Control Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl1`]
module"]
pub type CTRL1 = crate::Reg<ctrl1::CTRL1_SPEC>;
#[doc = "Control Register 1"]
pub mod ctrl1;
#[doc = "timeh (rw) register accessor: timestamp high\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timeh::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timeh::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timeh`]
module"]
pub type TIMEH = crate::Reg<timeh::TIMEH_SPEC>;
#[doc = "timestamp high"]
pub mod timeh;
#[doc = "timel (rw) register accessor: timestamp low\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timel`]
module"]
pub type TIMEL = crate::Reg<timel::TIMEL_SPEC>;
#[doc = "timestamp low"]
pub mod timel;
#[doc = "ts_updth (rw) register accessor: timestamp update high\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ts_updth::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ts_updth::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ts_updth`]
module"]
pub type TS_UPDTH = crate::Reg<ts_updth::TS_UPDTH_SPEC>;
#[doc = "timestamp update high"]
pub mod ts_updth;
#[doc = "ts_updtl (rw) register accessor: timestamp update low\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ts_updtl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ts_updtl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ts_updtl`]
module"]
pub type TS_UPDTL = crate::Reg<ts_updtl::TS_UPDTL_SPEC>;
#[doc = "timestamp update low"]
pub mod ts_updtl;
#[doc = "addend (rw) register accessor: No description avaiable\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`addend::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`addend::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@addend`]
module"]
pub type ADDEND = crate::Reg<addend::ADDEND_SPEC>;
#[doc = "No description avaiable"]
pub mod addend;
#[doc = "tarh (rw) register accessor: No description avaiable\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tarh::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tarh::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tarh`]
module"]
pub type TARH = crate::Reg<tarh::TARH_SPEC>;
#[doc = "No description avaiable"]
pub mod tarh;
#[doc = "tarl (rw) register accessor: No description avaiable\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tarl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tarl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tarl`]
module"]
pub type TARL = crate::Reg<tarl::TARL_SPEC>;
#[doc = "No description avaiable"]
pub mod tarl;
#[doc = "pps_ctrl (rw) register accessor: No description avaiable\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pps_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pps_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pps_ctrl`]
module"]
pub type PPS_CTRL = crate::Reg<pps_ctrl::PPS_CTRL_SPEC>;
#[doc = "No description avaiable"]
pub mod pps_ctrl;
#[doc = "capt_snaph (rw) register accessor: No description avaiable\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`capt_snaph::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`capt_snaph::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@capt_snaph`]
module"]
pub type CAPT_SNAPH = crate::Reg<capt_snaph::CAPT_SNAPH_SPEC>;
#[doc = "No description avaiable"]
pub mod capt_snaph;
#[doc = "capt_snapl (rw) register accessor: No description avaiable\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`capt_snapl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`capt_snapl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@capt_snapl`]
module"]
pub type CAPT_SNAPL = crate::Reg<capt_snapl::CAPT_SNAPL_SPEC>;
#[doc = "No description avaiable"]
pub mod capt_snapl;
