#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x10],
    ctrl: CTRL,
    restart: RESTART,
    wr_en: WR_EN,
    st: ST,
}
impl RegisterBlock {
    #[doc = "0x10 - Control Register"]
    #[inline(always)]
    pub const fn ctrl(&self) -> &CTRL {
        &self.ctrl
    }
    #[doc = "0x14 - Restart Register"]
    #[inline(always)]
    pub const fn restart(&self) -> &RESTART {
        &self.restart
    }
    #[doc = "0x18 - Write Protection Register"]
    #[inline(always)]
    pub const fn wr_en(&self) -> &WR_EN {
        &self.wr_en
    }
    #[doc = "0x1c - Status Register"]
    #[inline(always)]
    pub const fn st(&self) -> &ST {
        &self.st
    }
}
#[doc = "CTRL (rw) register accessor: Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl`]
module"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "Control Register"]
pub mod ctrl;
#[doc = "Restart (rw) register accessor: Restart Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`restart::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`restart::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@restart`]
module"]
pub type RESTART = crate::Reg<restart::RESTART_SPEC>;
#[doc = "Restart Register"]
pub mod restart;
#[doc = "WrEn (rw) register accessor: Write Protection Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wr_en::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wr_en::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wr_en`]
module"]
pub type WR_EN = crate::Reg<wr_en::WR_EN_SPEC>;
#[doc = "Write Protection Register"]
pub mod wr_en;
#[doc = "St (rw) register accessor: Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`st::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`st::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@st`]
module"]
pub type ST = crate::Reg<st::ST_SPEC>;
#[doc = "Status Register"]
pub mod st;
