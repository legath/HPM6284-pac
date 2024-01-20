#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x10],
    trans_fmt: TRANS_FMT,
    _reserved1: [u8; 0x0c],
    trans_ctrl: TRANS_CTRL,
    cmd: CMD,
    addr: ADDR,
    data: DATA,
    ctrl: CTRL,
    status: STATUS,
    intr_en: INTR_EN,
    intr_st: INTR_ST,
    timing: TIMING,
    _reserved10: [u8; 0x1c],
    slv_st: SLV_ST,
    slv_data_cnt: SLV_DATA_CNT,
    _reserved12: [u8; 0x14],
    config: CONFIG,
}
impl RegisterBlock {
    #[doc = "0x10 - Transfer Format Register"]
    #[inline(always)]
    pub const fn trans_fmt(&self) -> &TRANS_FMT {
        &self.trans_fmt
    }
    #[doc = "0x20 - Transfer Control Register"]
    #[inline(always)]
    pub const fn trans_ctrl(&self) -> &TRANS_CTRL {
        &self.trans_ctrl
    }
    #[doc = "0x24 - Command Register"]
    #[inline(always)]
    pub const fn cmd(&self) -> &CMD {
        &self.cmd
    }
    #[doc = "0x28 - Address Register"]
    #[inline(always)]
    pub const fn addr(&self) -> &ADDR {
        &self.addr
    }
    #[doc = "0x2c - Data Register"]
    #[inline(always)]
    pub const fn data(&self) -> &DATA {
        &self.data
    }
    #[doc = "0x30 - Control Register"]
    #[inline(always)]
    pub const fn ctrl(&self) -> &CTRL {
        &self.ctrl
    }
    #[doc = "0x34 - Status Register"]
    #[inline(always)]
    pub const fn status(&self) -> &STATUS {
        &self.status
    }
    #[doc = "0x38 - Interrupt Enable Register"]
    #[inline(always)]
    pub const fn intr_en(&self) -> &INTR_EN {
        &self.intr_en
    }
    #[doc = "0x3c - Interrupt Status Register"]
    #[inline(always)]
    pub const fn intr_st(&self) -> &INTR_ST {
        &self.intr_st
    }
    #[doc = "0x40 - Interface Timing Register"]
    #[inline(always)]
    pub const fn timing(&self) -> &TIMING {
        &self.timing
    }
    #[doc = "0x60 - Slave Status Register"]
    #[inline(always)]
    pub const fn slv_st(&self) -> &SLV_ST {
        &self.slv_st
    }
    #[doc = "0x64 - Slave Data Count Register"]
    #[inline(always)]
    pub const fn slv_data_cnt(&self) -> &SLV_DATA_CNT {
        &self.slv_data_cnt
    }
    #[doc = "0x7c - Configuration Register"]
    #[inline(always)]
    pub const fn config(&self) -> &CONFIG {
        &self.config
    }
}
#[doc = "TransFmt (rw) register accessor: Transfer Format Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`trans_fmt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`trans_fmt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@trans_fmt`]
module"]
pub type TRANS_FMT = crate::Reg<trans_fmt::TRANS_FMT_SPEC>;
#[doc = "Transfer Format Register"]
pub mod trans_fmt;
#[doc = "TransCtrl (rw) register accessor: Transfer Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`trans_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`trans_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@trans_ctrl`]
module"]
pub type TRANS_CTRL = crate::Reg<trans_ctrl::TRANS_CTRL_SPEC>;
#[doc = "Transfer Control Register"]
pub mod trans_ctrl;
#[doc = "Cmd (rw) register accessor: Command Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmd::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmd::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmd`]
module"]
pub type CMD = crate::Reg<cmd::CMD_SPEC>;
#[doc = "Command Register"]
pub mod cmd;
#[doc = "Addr (rw) register accessor: Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`addr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`addr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@addr`]
module"]
pub type ADDR = crate::Reg<addr::ADDR_SPEC>;
#[doc = "Address Register"]
pub mod addr;
#[doc = "Data (rw) register accessor: Data Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`data::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`data::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@data`]
module"]
pub type DATA = crate::Reg<data::DATA_SPEC>;
#[doc = "Data Register"]
pub mod data;
#[doc = "Ctrl (rw) register accessor: Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl`]
module"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "Control Register"]
pub mod ctrl;
#[doc = "Status (rw) register accessor: Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`status::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`status::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@status`]
module"]
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
#[doc = "Status Register"]
pub mod status;
#[doc = "IntrEn (rw) register accessor: Interrupt Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intr_en::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intr_en::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intr_en`]
module"]
pub type INTR_EN = crate::Reg<intr_en::INTR_EN_SPEC>;
#[doc = "Interrupt Enable Register"]
pub mod intr_en;
#[doc = "IntrSt (rw) register accessor: Interrupt Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intr_st::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intr_st::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intr_st`]
module"]
pub type INTR_ST = crate::Reg<intr_st::INTR_ST_SPEC>;
#[doc = "Interrupt Status Register"]
pub mod intr_st;
#[doc = "Timing (rw) register accessor: Interface Timing Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timing::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timing::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timing`]
module"]
pub type TIMING = crate::Reg<timing::TIMING_SPEC>;
#[doc = "Interface Timing Register"]
pub mod timing;
#[doc = "SlvSt (rw) register accessor: Slave Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`slv_st::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`slv_st::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@slv_st`]
module"]
pub type SLV_ST = crate::Reg<slv_st::SLV_ST_SPEC>;
#[doc = "Slave Status Register"]
pub mod slv_st;
#[doc = "SlvDataCnt (rw) register accessor: Slave Data Count Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`slv_data_cnt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`slv_data_cnt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@slv_data_cnt`]
module"]
pub type SLV_DATA_CNT = crate::Reg<slv_data_cnt::SLV_DATA_CNT_SPEC>;
#[doc = "Slave Data Count Register"]
pub mod slv_data_cnt;
#[doc = "Config (rw) register accessor: Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`config::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`config::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@config`]
module"]
pub type CONFIG = crate::Reg<config::CONFIG_SPEC>;
#[doc = "Configuration Register"]
pub mod config;
