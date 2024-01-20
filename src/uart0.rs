#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x04],
    idle_cfg: IDLE_CFG,
    _reserved1: [u8; 0x08],
    cfg: CFG,
    oscr: OSCR,
    _reserved3: [u8; 0x08],
    _reserved_3_dll: [u8; 0x04],
    _reserved_4_dlm: [u8; 0x04],
    _reserved_5_fcr: [u8; 0x04],
    lcr: LCR,
    mcr: MCR,
    lsr: LSR,
    msr: MSR,
}
impl RegisterBlock {
    #[doc = "0x04 - Idle Configuration Register"]
    #[inline(always)]
    pub const fn idle_cfg(&self) -> &IDLE_CFG {
        &self.idle_cfg
    }
    #[doc = "0x10 - Configuration Register"]
    #[inline(always)]
    pub const fn cfg(&self) -> &CFG {
        &self.cfg
    }
    #[doc = "0x14 - Over Sample Control Register"]
    #[inline(always)]
    pub const fn oscr(&self) -> &OSCR {
        &self.oscr
    }
    #[doc = "0x20 - Divisor Latch LSB (when DLAB = 1)"]
    #[inline(always)]
    pub const fn dll(&self) -> &DLL {
        unsafe { &*(self as *const Self).cast::<u8>().add(32).cast() }
    }
    #[doc = "0x20 - Transmitter Holding Register (when DLAB = 0)"]
    #[inline(always)]
    pub const fn thr(&self) -> &THR {
        unsafe { &*(self as *const Self).cast::<u8>().add(32).cast() }
    }
    #[doc = "0x20 - Receiver Buffer Register (when DLAB = 0)"]
    #[inline(always)]
    pub const fn rbr(&self) -> &RBR {
        unsafe { &*(self as *const Self).cast::<u8>().add(32).cast() }
    }
    #[doc = "0x24 - Divisor Latch MSB (when DLAB = 1)"]
    #[inline(always)]
    pub const fn dlm(&self) -> &DLM {
        unsafe { &*(self as *const Self).cast::<u8>().add(36).cast() }
    }
    #[doc = "0x24 - Interrupt Enable Register (when DLAB = 0)"]
    #[inline(always)]
    pub const fn ier(&self) -> &IER {
        unsafe { &*(self as *const Self).cast::<u8>().add(36).cast() }
    }
    #[doc = "0x28 - FIFO Control Register"]
    #[inline(always)]
    pub const fn fcr(&self) -> &FCR {
        unsafe { &*(self as *const Self).cast::<u8>().add(40).cast() }
    }
    #[doc = "0x28 - Interrupt Identification Register"]
    #[inline(always)]
    pub const fn iir(&self) -> &IIR {
        unsafe { &*(self as *const Self).cast::<u8>().add(40).cast() }
    }
    #[doc = "0x2c - Line Control Register"]
    #[inline(always)]
    pub const fn lcr(&self) -> &LCR {
        &self.lcr
    }
    #[doc = "0x30 - Modem Control Register ("]
    #[inline(always)]
    pub const fn mcr(&self) -> &MCR {
        &self.mcr
    }
    #[doc = "0x34 - Line Status Register"]
    #[inline(always)]
    pub const fn lsr(&self) -> &LSR {
        &self.lsr
    }
    #[doc = "0x38 - Modem Status Register"]
    #[inline(always)]
    pub const fn msr(&self) -> &MSR {
        &self.msr
    }
}
#[doc = "IDLE_CFG (rw) register accessor: Idle Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`idle_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`idle_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@idle_cfg`]
module"]
pub type IDLE_CFG = crate::Reg<idle_cfg::IDLE_CFG_SPEC>;
#[doc = "Idle Configuration Register"]
pub mod idle_cfg;
#[doc = "Cfg (rw) register accessor: Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub type CFG = crate::Reg<cfg::CFG_SPEC>;
#[doc = "Configuration Register"]
pub mod cfg;
#[doc = "OSCR (rw) register accessor: Over Sample Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`oscr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`oscr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@oscr`]
module"]
pub type OSCR = crate::Reg<oscr::OSCR_SPEC>;
#[doc = "Over Sample Control Register"]
pub mod oscr;
#[doc = "RBR (rw) register accessor: Receiver Buffer Register (when DLAB = 0)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rbr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rbr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rbr`]
module"]
pub type RBR = crate::Reg<rbr::RBR_SPEC>;
#[doc = "Receiver Buffer Register (when DLAB = 0)"]
pub mod rbr;
#[doc = "THR (rw) register accessor: Transmitter Holding Register (when DLAB = 0)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`thr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`thr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@thr`]
module"]
pub type THR = crate::Reg<thr::THR_SPEC>;
#[doc = "Transmitter Holding Register (when DLAB = 0)"]
pub mod thr;
#[doc = "DLL (rw) register accessor: Divisor Latch LSB (when DLAB = 1)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dll::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dll::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dll`]
module"]
pub type DLL = crate::Reg<dll::DLL_SPEC>;
#[doc = "Divisor Latch LSB (when DLAB = 1)"]
pub mod dll;
#[doc = "IER (rw) register accessor: Interrupt Enable Register (when DLAB = 0)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ier::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ier::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ier`]
module"]
pub type IER = crate::Reg<ier::IER_SPEC>;
#[doc = "Interrupt Enable Register (when DLAB = 0)"]
pub mod ier;
#[doc = "DLM (rw) register accessor: Divisor Latch MSB (when DLAB = 1)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dlm::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dlm::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dlm`]
module"]
pub type DLM = crate::Reg<dlm::DLM_SPEC>;
#[doc = "Divisor Latch MSB (when DLAB = 1)"]
pub mod dlm;
#[doc = "IIR (rw) register accessor: Interrupt Identification Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iir::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iir::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iir`]
module"]
pub type IIR = crate::Reg<iir::IIR_SPEC>;
#[doc = "Interrupt Identification Register"]
pub mod iir;
#[doc = "FCR (rw) register accessor: FIFO Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fcr`]
module"]
pub type FCR = crate::Reg<fcr::FCR_SPEC>;
#[doc = "FIFO Control Register"]
pub mod fcr;
#[doc = "LCR (rw) register accessor: Line Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lcr`]
module"]
pub type LCR = crate::Reg<lcr::LCR_SPEC>;
#[doc = "Line Control Register"]
pub mod lcr;
#[doc = "MCR (rw) register accessor: Modem Control Register (\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcr`]
module"]
pub type MCR = crate::Reg<mcr::MCR_SPEC>;
#[doc = "Modem Control Register ("]
pub mod mcr;
#[doc = "LSR (rw) register accessor: Line Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lsr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lsr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lsr`]
module"]
pub type LSR = crate::Reg<lsr::LSR_SPEC>;
#[doc = "Line Status Register"]
pub mod lsr;
#[doc = "MSR (rw) register accessor: Modem Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`msr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`msr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@msr`]
module"]
pub type MSR = crate::Reg<msr::MSR_SPEC>;
#[doc = "Modem Status Register"]
pub mod msr;
