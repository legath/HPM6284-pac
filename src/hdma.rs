#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x04],
    idmisc: IDMISC,
    _reserved1: [u8; 0x08],
    dmacfg: DMACFG,
    _reserved2: [u8; 0x0c],
    dmactrl: DMACTRL,
    ch_abort: CH_ABORT,
    _reserved4: [u8; 0x08],
    int_status: INT_STATUS,
    ch_en: CH_EN,
    _reserved6: [u8; 0x08],
    chctrl: [CHCTRL; 8],
}
impl RegisterBlock {
    #[doc = "0x04 - ID Misc"]
    #[inline(always)]
    pub const fn idmisc(&self) -> &IDMISC {
        &self.idmisc
    }
    #[doc = "0x10 - DMAC Configuration Register"]
    #[inline(always)]
    pub const fn dmacfg(&self) -> &DMACFG {
        &self.dmacfg
    }
    #[doc = "0x20 - DMAC Control Register"]
    #[inline(always)]
    pub const fn dmactrl(&self) -> &DMACTRL {
        &self.dmactrl
    }
    #[doc = "0x24 - Channel Abort Register"]
    #[inline(always)]
    pub const fn ch_abort(&self) -> &CH_ABORT {
        &self.ch_abort
    }
    #[doc = "0x30 - Interrupt Status Register"]
    #[inline(always)]
    pub const fn int_status(&self) -> &INT_STATUS {
        &self.int_status
    }
    #[doc = "0x34 - Channel Enable Register"]
    #[inline(always)]
    pub const fn ch_en(&self) -> &CH_EN {
        &self.ch_en
    }
    #[doc = "0x40..0x140 - no description available"]
    #[inline(always)]
    pub const fn chctrl(&self, n: usize) -> &CHCTRL {
        &self.chctrl[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x40..0x140 - no description available"]
    #[inline(always)]
    pub fn chctrl_iter(&self) -> impl Iterator<Item = &CHCTRL> {
        self.chctrl.iter()
    }
    #[doc = "0x40..0x60 - no description available"]
    #[inline(always)]
    pub const fn chctrlch0(&self) -> &CHCTRL {
        self.chctrl(0)
    }
    #[doc = "0x60..0x80 - no description available"]
    #[inline(always)]
    pub const fn chctrlch1(&self) -> &CHCTRL {
        self.chctrl(1)
    }
    #[doc = "0x80..0xa0 - no description available"]
    #[inline(always)]
    pub const fn chctrlch2(&self) -> &CHCTRL {
        self.chctrl(2)
    }
    #[doc = "0xa0..0xc0 - no description available"]
    #[inline(always)]
    pub const fn chctrlch3(&self) -> &CHCTRL {
        self.chctrl(3)
    }
    #[doc = "0xc0..0xe0 - no description available"]
    #[inline(always)]
    pub const fn chctrlch4(&self) -> &CHCTRL {
        self.chctrl(4)
    }
    #[doc = "0xe0..0x100 - no description available"]
    #[inline(always)]
    pub const fn chctrlch5(&self) -> &CHCTRL {
        self.chctrl(5)
    }
    #[doc = "0x100..0x120 - no description available"]
    #[inline(always)]
    pub const fn chctrlch6(&self) -> &CHCTRL {
        self.chctrl(6)
    }
    #[doc = "0x120..0x140 - no description available"]
    #[inline(always)]
    pub const fn chctrlch7(&self) -> &CHCTRL {
        self.chctrl(7)
    }
}
#[doc = "IDMisc (rw) register accessor: ID Misc\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`idmisc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`idmisc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@idmisc`]
module"]
pub type IDMISC = crate::Reg<idmisc::IDMISC_SPEC>;
#[doc = "ID Misc"]
pub mod idmisc;
#[doc = "DMACfg (rw) register accessor: DMAC Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmacfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmacfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmacfg`]
module"]
pub type DMACFG = crate::Reg<dmacfg::DMACFG_SPEC>;
#[doc = "DMAC Configuration Register"]
pub mod dmacfg;
#[doc = "DMACtrl (rw) register accessor: DMAC Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmactrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmactrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmactrl`]
module"]
pub type DMACTRL = crate::Reg<dmactrl::DMACTRL_SPEC>;
#[doc = "DMAC Control Register"]
pub mod dmactrl;
#[doc = "ChAbort (rw) register accessor: Channel Abort Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch_abort::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch_abort::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch_abort`]
module"]
pub type CH_ABORT = crate::Reg<ch_abort::CH_ABORT_SPEC>;
#[doc = "Channel Abort Register"]
pub mod ch_abort;
#[doc = "IntStatus (rw) register accessor: Interrupt Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_status::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_status::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_status`]
module"]
pub type INT_STATUS = crate::Reg<int_status::INT_STATUS_SPEC>;
#[doc = "Interrupt Status Register"]
pub mod int_status;
#[doc = "ChEN (rw) register accessor: Channel Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch_en::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch_en::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch_en`]
module"]
pub type CH_EN = crate::Reg<ch_en::CH_EN_SPEC>;
#[doc = "Channel Enable Register"]
pub mod ch_en;
#[doc = "no description available"]
pub use self::chctrl::CHCTRL;
#[doc = r"Cluster"]
#[doc = "no description available"]
pub mod chctrl;
