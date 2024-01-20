#[doc = r"Register block"]
#[repr(C)]
pub struct PLL {
    mfi: MFI,
    mfn: MFN,
    mfd: MFD,
    ss_step: SS_STEP,
    ss_stop: SS_STOP,
    config: CONFIG,
    locktime: LOCKTIME,
    steptime: STEPTIME,
    advanced: ADVANCED,
    _reserved9: [u8; 0x1c],
    div: [DIV; 4],
}
impl PLL {
    #[doc = "0x00 - PLL0 multiple register"]
    #[inline(always)]
    pub const fn mfi(&self) -> &MFI {
        &self.mfi
    }
    #[doc = "0x04 - PLL0 fraction numerator register"]
    #[inline(always)]
    pub const fn mfn(&self) -> &MFN {
        &self.mfn
    }
    #[doc = "0x08 - PLL0 fraction demoninator register"]
    #[inline(always)]
    pub const fn mfd(&self) -> &MFD {
        &self.mfd
    }
    #[doc = "0x0c - PLL0 spread spectrum step register"]
    #[inline(always)]
    pub const fn ss_step(&self) -> &SS_STEP {
        &self.ss_step
    }
    #[doc = "0x10 - PLL0 spread spectrum stop register"]
    #[inline(always)]
    pub const fn ss_stop(&self) -> &SS_STOP {
        &self.ss_stop
    }
    #[doc = "0x14 - PLL0 confguration register"]
    #[inline(always)]
    pub const fn config(&self) -> &CONFIG {
        &self.config
    }
    #[doc = "0x18 - PLL0 lock time register"]
    #[inline(always)]
    pub const fn locktime(&self) -> &LOCKTIME {
        &self.locktime
    }
    #[doc = "0x1c - PLL0 step time register"]
    #[inline(always)]
    pub const fn steptime(&self) -> &STEPTIME {
        &self.steptime
    }
    #[doc = "0x20 - PLL0 advance configuration register"]
    #[inline(always)]
    pub const fn advanced(&self) -> &ADVANCED {
        &self.advanced
    }
    #[doc = "0x40..0x50 - no description available"]
    #[inline(always)]
    pub const fn div(&self, n: usize) -> &DIV {
        &self.div[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x40..0x50 - no description available"]
    #[inline(always)]
    pub fn div_iter(&self) -> impl Iterator<Item = &DIV> {
        self.div.iter()
    }
    #[doc = "0x40 - no description available"]
    #[inline(always)]
    pub const fn divdiv0(&self) -> &DIV {
        self.div(0)
    }
    #[doc = "0x44 - no description available"]
    #[inline(always)]
    pub const fn divdiv1(&self) -> &DIV {
        self.div(1)
    }
    #[doc = "0x48 - no description available"]
    #[inline(always)]
    pub const fn divdiv2(&self) -> &DIV {
        self.div(2)
    }
    #[doc = "0x4c - no description available"]
    #[inline(always)]
    pub const fn divdiv3(&self) -> &DIV {
        self.div(3)
    }
}
#[doc = "MFI (rw) register accessor: PLL0 multiple register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mfi::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mfi::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mfi`]
module"]
pub type MFI = crate::Reg<mfi::MFI_SPEC>;
#[doc = "PLL0 multiple register"]
pub mod mfi;
#[doc = "MFN (rw) register accessor: PLL0 fraction numerator register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mfn::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mfn::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mfn`]
module"]
pub type MFN = crate::Reg<mfn::MFN_SPEC>;
#[doc = "PLL0 fraction numerator register"]
pub mod mfn;
#[doc = "MFD (rw) register accessor: PLL0 fraction demoninator register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mfd::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mfd::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mfd`]
module"]
pub type MFD = crate::Reg<mfd::MFD_SPEC>;
#[doc = "PLL0 fraction demoninator register"]
pub mod mfd;
#[doc = "SS_STEP (rw) register accessor: PLL0 spread spectrum step register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ss_step::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ss_step::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ss_step`]
module"]
pub type SS_STEP = crate::Reg<ss_step::SS_STEP_SPEC>;
#[doc = "PLL0 spread spectrum step register"]
pub mod ss_step;
#[doc = "SS_STOP (rw) register accessor: PLL0 spread spectrum stop register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ss_stop::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ss_stop::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ss_stop`]
module"]
pub type SS_STOP = crate::Reg<ss_stop::SS_STOP_SPEC>;
#[doc = "PLL0 spread spectrum stop register"]
pub mod ss_stop;
#[doc = "CONFIG (rw) register accessor: PLL0 confguration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`config::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`config::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@config`]
module"]
pub type CONFIG = crate::Reg<config::CONFIG_SPEC>;
#[doc = "PLL0 confguration register"]
pub mod config;
#[doc = "LOCKTIME (rw) register accessor: PLL0 lock time register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`locktime::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`locktime::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@locktime`]
module"]
pub type LOCKTIME = crate::Reg<locktime::LOCKTIME_SPEC>;
#[doc = "PLL0 lock time register"]
pub mod locktime;
#[doc = "STEPTIME (rw) register accessor: PLL0 step time register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`steptime::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`steptime::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@steptime`]
module"]
pub type STEPTIME = crate::Reg<steptime::STEPTIME_SPEC>;
#[doc = "PLL0 step time register"]
pub mod steptime;
#[doc = "ADVANCED (rw) register accessor: PLL0 advance configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`advanced::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`advanced::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@advanced`]
module"]
pub type ADVANCED = crate::Reg<advanced::ADVANCED_SPEC>;
#[doc = "PLL0 advance configuration register"]
pub mod advanced;
#[doc = "DIV (rw) register accessor: no description available\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`div::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`div::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@div`]
module"]
pub type DIV = crate::Reg<div::DIV_SPEC>;
#[doc = "no description available"]
pub mod div;
