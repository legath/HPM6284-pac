#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    bandgap: BANDGAP,
    ldo1p1: LDO1P1,
    ldo2p5: LDO2P5,
    _reserved3: [u8; 0x04],
    dcdc_mode: DCDC_MODE,
    dcdc_lpmode: DCDC_LPMODE,
    dcdc_prot: DCDC_PROT,
    dcdc_current: DCDC_CURRENT,
    dcdc_advmode: DCDC_ADVMODE,
    dcdc_advparam: DCDC_ADVPARAM,
    dcdc_misc: DCDC_MISC,
    dcdc_debug: DCDC_DEBUG,
    dcdc_start_time: DCDC_START_TIME,
    dcdc_resume_time: DCDC_RESUME_TIME,
    _reserved13: [u8; 0x08],
    power_trap: POWER_TRAP,
    wake_cause: WAKE_CAUSE,
    wake_mask: WAKE_MASK,
    scg_ctrl: SCG_CTRL,
    debug_stop: DEBUG_STOP,
    _reserved18: [u8; 0x0c],
    rc24m: RC24M,
    rc24m_track: RC24M_TRACK,
    track_target: TRACK_TARGET,
    status: STATUS,
}
impl RegisterBlock {
    #[doc = "0x00 - BANGGAP control"]
    #[inline(always)]
    pub const fn bandgap(&self) -> &BANDGAP {
        &self.bandgap
    }
    #[doc = "0x04 - 1V LDO config"]
    #[inline(always)]
    pub const fn ldo1p1(&self) -> &LDO1P1 {
        &self.ldo1p1
    }
    #[doc = "0x08 - 2.5V LDO config"]
    #[inline(always)]
    pub const fn ldo2p5(&self) -> &LDO2P5 {
        &self.ldo2p5
    }
    #[doc = "0x10 - DCDC mode select"]
    #[inline(always)]
    pub const fn dcdc_mode(&self) -> &DCDC_MODE {
        &self.dcdc_mode
    }
    #[doc = "0x14 - DCDC low power mode"]
    #[inline(always)]
    pub const fn dcdc_lpmode(&self) -> &DCDC_LPMODE {
        &self.dcdc_lpmode
    }
    #[doc = "0x18 - DCDC protection"]
    #[inline(always)]
    pub const fn dcdc_prot(&self) -> &DCDC_PROT {
        &self.dcdc_prot
    }
    #[doc = "0x1c - DCDC current estimation"]
    #[inline(always)]
    pub const fn dcdc_current(&self) -> &DCDC_CURRENT {
        &self.dcdc_current
    }
    #[doc = "0x20 - DCDC advance setting"]
    #[inline(always)]
    pub const fn dcdc_advmode(&self) -> &DCDC_ADVMODE {
        &self.dcdc_advmode
    }
    #[doc = "0x24 - DCDC advance parameter"]
    #[inline(always)]
    pub const fn dcdc_advparam(&self) -> &DCDC_ADVPARAM {
        &self.dcdc_advparam
    }
    #[doc = "0x28 - DCDC misc parameter"]
    #[inline(always)]
    pub const fn dcdc_misc(&self) -> &DCDC_MISC {
        &self.dcdc_misc
    }
    #[doc = "0x2c - DCDC Debug"]
    #[inline(always)]
    pub const fn dcdc_debug(&self) -> &DCDC_DEBUG {
        &self.dcdc_debug
    }
    #[doc = "0x30 - DCDC ramp time"]
    #[inline(always)]
    pub const fn dcdc_start_time(&self) -> &DCDC_START_TIME {
        &self.dcdc_start_time
    }
    #[doc = "0x34 - DCDC resume time"]
    #[inline(always)]
    pub const fn dcdc_resume_time(&self) -> &DCDC_RESUME_TIME {
        &self.dcdc_resume_time
    }
    #[doc = "0x40 - SOC power trap"]
    #[inline(always)]
    pub const fn power_trap(&self) -> &POWER_TRAP {
        &self.power_trap
    }
    #[doc = "0x44 - Wake up source"]
    #[inline(always)]
    pub const fn wake_cause(&self) -> &WAKE_CAUSE {
        &self.wake_cause
    }
    #[doc = "0x48 - Wake up mask"]
    #[inline(always)]
    pub const fn wake_mask(&self) -> &WAKE_MASK {
        &self.wake_mask
    }
    #[doc = "0x4c - Clock gate control in PMIC"]
    #[inline(always)]
    pub const fn scg_ctrl(&self) -> &SCG_CTRL {
        &self.scg_ctrl
    }
    #[doc = "0x50 - Debug stop config"]
    #[inline(always)]
    pub const fn debug_stop(&self) -> &DEBUG_STOP {
        &self.debug_stop
    }
    #[doc = "0x60 - RC 24M config"]
    #[inline(always)]
    pub const fn rc24m(&self) -> &RC24M {
        &self.rc24m
    }
    #[doc = "0x64 - RC 24M track mode"]
    #[inline(always)]
    pub const fn rc24m_track(&self) -> &RC24M_TRACK {
        &self.rc24m_track
    }
    #[doc = "0x68 - RC 24M track target"]
    #[inline(always)]
    pub const fn track_target(&self) -> &TRACK_TARGET {
        &self.track_target
    }
    #[doc = "0x6c - RC 24M track status"]
    #[inline(always)]
    pub const fn status(&self) -> &STATUS {
        &self.status
    }
}
#[doc = "BANDGAP (rw) register accessor: BANGGAP control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bandgap::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bandgap::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bandgap`]
module"]
pub type BANDGAP = crate::Reg<bandgap::BANDGAP_SPEC>;
#[doc = "BANGGAP control"]
pub mod bandgap;
#[doc = "LDO1P1 (rw) register accessor: 1V LDO config\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ldo1p1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ldo1p1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ldo1p1`]
module"]
pub type LDO1P1 = crate::Reg<ldo1p1::LDO1P1_SPEC>;
#[doc = "1V LDO config"]
pub mod ldo1p1;
#[doc = "LDO2P5 (rw) register accessor: 2.5V LDO config\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ldo2p5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ldo2p5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ldo2p5`]
module"]
pub type LDO2P5 = crate::Reg<ldo2p5::LDO2P5_SPEC>;
#[doc = "2.5V LDO config"]
pub mod ldo2p5;
#[doc = "DCDC_MODE (rw) register accessor: DCDC mode select\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dcdc_mode::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dcdc_mode::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dcdc_mode`]
module"]
pub type DCDC_MODE = crate::Reg<dcdc_mode::DCDC_MODE_SPEC>;
#[doc = "DCDC mode select"]
pub mod dcdc_mode;
#[doc = "DCDC_LPMODE (rw) register accessor: DCDC low power mode\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dcdc_lpmode::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dcdc_lpmode::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dcdc_lpmode`]
module"]
pub type DCDC_LPMODE = crate::Reg<dcdc_lpmode::DCDC_LPMODE_SPEC>;
#[doc = "DCDC low power mode"]
pub mod dcdc_lpmode;
#[doc = "DCDC_PROT (rw) register accessor: DCDC protection\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dcdc_prot::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dcdc_prot::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dcdc_prot`]
module"]
pub type DCDC_PROT = crate::Reg<dcdc_prot::DCDC_PROT_SPEC>;
#[doc = "DCDC protection"]
pub mod dcdc_prot;
#[doc = "DCDC_CURRENT (rw) register accessor: DCDC current estimation\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dcdc_current::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dcdc_current::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dcdc_current`]
module"]
pub type DCDC_CURRENT = crate::Reg<dcdc_current::DCDC_CURRENT_SPEC>;
#[doc = "DCDC current estimation"]
pub mod dcdc_current;
#[doc = "DCDC_ADVMODE (rw) register accessor: DCDC advance setting\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dcdc_advmode::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dcdc_advmode::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dcdc_advmode`]
module"]
pub type DCDC_ADVMODE = crate::Reg<dcdc_advmode::DCDC_ADVMODE_SPEC>;
#[doc = "DCDC advance setting"]
pub mod dcdc_advmode;
#[doc = "DCDC_ADVPARAM (rw) register accessor: DCDC advance parameter\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dcdc_advparam::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dcdc_advparam::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dcdc_advparam`]
module"]
pub type DCDC_ADVPARAM = crate::Reg<dcdc_advparam::DCDC_ADVPARAM_SPEC>;
#[doc = "DCDC advance parameter"]
pub mod dcdc_advparam;
#[doc = "DCDC_MISC (rw) register accessor: DCDC misc parameter\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dcdc_misc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dcdc_misc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dcdc_misc`]
module"]
pub type DCDC_MISC = crate::Reg<dcdc_misc::DCDC_MISC_SPEC>;
#[doc = "DCDC misc parameter"]
pub mod dcdc_misc;
#[doc = "DCDC_DEBUG (rw) register accessor: DCDC Debug\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dcdc_debug::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dcdc_debug::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dcdc_debug`]
module"]
pub type DCDC_DEBUG = crate::Reg<dcdc_debug::DCDC_DEBUG_SPEC>;
#[doc = "DCDC Debug"]
pub mod dcdc_debug;
#[doc = "DCDC_START_TIME (rw) register accessor: DCDC ramp time\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dcdc_start_time::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dcdc_start_time::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dcdc_start_time`]
module"]
pub type DCDC_START_TIME = crate::Reg<dcdc_start_time::DCDC_START_TIME_SPEC>;
#[doc = "DCDC ramp time"]
pub mod dcdc_start_time;
#[doc = "DCDC_RESUME_TIME (rw) register accessor: DCDC resume time\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dcdc_resume_time::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dcdc_resume_time::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dcdc_resume_time`]
module"]
pub type DCDC_RESUME_TIME = crate::Reg<dcdc_resume_time::DCDC_RESUME_TIME_SPEC>;
#[doc = "DCDC resume time"]
pub mod dcdc_resume_time;
#[doc = "POWER_TRAP (rw) register accessor: SOC power trap\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`power_trap::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`power_trap::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@power_trap`]
module"]
pub type POWER_TRAP = crate::Reg<power_trap::POWER_TRAP_SPEC>;
#[doc = "SOC power trap"]
pub mod power_trap;
#[doc = "WAKE_CAUSE (rw) register accessor: Wake up source\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wake_cause::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wake_cause::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wake_cause`]
module"]
pub type WAKE_CAUSE = crate::Reg<wake_cause::WAKE_CAUSE_SPEC>;
#[doc = "Wake up source"]
pub mod wake_cause;
#[doc = "WAKE_MASK (rw) register accessor: Wake up mask\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wake_mask::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wake_mask::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wake_mask`]
module"]
pub type WAKE_MASK = crate::Reg<wake_mask::WAKE_MASK_SPEC>;
#[doc = "Wake up mask"]
pub mod wake_mask;
#[doc = "SCG_CTRL (rw) register accessor: Clock gate control in PMIC\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`scg_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`scg_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scg_ctrl`]
module"]
pub type SCG_CTRL = crate::Reg<scg_ctrl::SCG_CTRL_SPEC>;
#[doc = "Clock gate control in PMIC"]
pub mod scg_ctrl;
#[doc = "DEBUG_STOP (rw) register accessor: Debug stop config\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`debug_stop::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`debug_stop::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@debug_stop`]
module"]
pub type DEBUG_STOP = crate::Reg<debug_stop::DEBUG_STOP_SPEC>;
#[doc = "Debug stop config"]
pub mod debug_stop;
#[doc = "RC24M (rw) register accessor: RC 24M config\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rc24m::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rc24m::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rc24m`]
module"]
pub type RC24M = crate::Reg<rc24m::RC24M_SPEC>;
#[doc = "RC 24M config"]
pub mod rc24m;
#[doc = "RC24M_TRACK (rw) register accessor: RC 24M track mode\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rc24m_track::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rc24m_track::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rc24m_track`]
module"]
pub type RC24M_TRACK = crate::Reg<rc24m_track::RC24M_TRACK_SPEC>;
#[doc = "RC 24M track mode"]
pub mod rc24m_track;
#[doc = "TRACK_TARGET (rw) register accessor: RC 24M track target\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`track_target::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`track_target::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@track_target`]
module"]
pub type TRACK_TARGET = crate::Reg<track_target::TRACK_TARGET_SPEC>;
#[doc = "RC 24M track target"]
pub mod track_target;
#[doc = "STATUS (rw) register accessor: RC 24M track status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`status::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`status::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@status`]
module"]
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
#[doc = "RC 24M track status"]
pub mod status;
