#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    pmic_gpr00: PMIC_GPR00,
    pmic_gpr01: PMIC_GPR01,
    pmic_gpr02: PMIC_GPR02,
    pmic_gpr03: PMIC_GPR03,
    pmic_gpr04: PMIC_GPR04,
    pmic_gpr05: PMIC_GPR05,
    pmic_gpr06: PMIC_GPR06,
    pmic_gpr07: PMIC_GPR07,
    pmic_gpr08: PMIC_GPR08,
    pmic_gpr09: PMIC_GPR09,
    pmic_gpr10: PMIC_GPR10,
    pmic_gpr11: PMIC_GPR11,
    pmic_gpr12: PMIC_GPR12,
    pmic_gpr13: PMIC_GPR13,
    pmic_gpr14: PMIC_GPR14,
    pmic_gpr15: PMIC_GPR15,
}
impl RegisterBlock {
    #[doc = "0x00 - Generic control"]
    #[inline(always)]
    pub const fn pmic_gpr00(&self) -> &PMIC_GPR00 {
        &self.pmic_gpr00
    }
    #[doc = "0x04 - Generic control"]
    #[inline(always)]
    pub const fn pmic_gpr01(&self) -> &PMIC_GPR01 {
        &self.pmic_gpr01
    }
    #[doc = "0x08 - Generic control"]
    #[inline(always)]
    pub const fn pmic_gpr02(&self) -> &PMIC_GPR02 {
        &self.pmic_gpr02
    }
    #[doc = "0x0c - Generic control"]
    #[inline(always)]
    pub const fn pmic_gpr03(&self) -> &PMIC_GPR03 {
        &self.pmic_gpr03
    }
    #[doc = "0x10 - Generic control"]
    #[inline(always)]
    pub const fn pmic_gpr04(&self) -> &PMIC_GPR04 {
        &self.pmic_gpr04
    }
    #[doc = "0x14 - Generic control"]
    #[inline(always)]
    pub const fn pmic_gpr05(&self) -> &PMIC_GPR05 {
        &self.pmic_gpr05
    }
    #[doc = "0x18 - Generic control"]
    #[inline(always)]
    pub const fn pmic_gpr06(&self) -> &PMIC_GPR06 {
        &self.pmic_gpr06
    }
    #[doc = "0x1c - Generic control"]
    #[inline(always)]
    pub const fn pmic_gpr07(&self) -> &PMIC_GPR07 {
        &self.pmic_gpr07
    }
    #[doc = "0x20 - Generic control"]
    #[inline(always)]
    pub const fn pmic_gpr08(&self) -> &PMIC_GPR08 {
        &self.pmic_gpr08
    }
    #[doc = "0x24 - Generic control"]
    #[inline(always)]
    pub const fn pmic_gpr09(&self) -> &PMIC_GPR09 {
        &self.pmic_gpr09
    }
    #[doc = "0x28 - Generic control"]
    #[inline(always)]
    pub const fn pmic_gpr10(&self) -> &PMIC_GPR10 {
        &self.pmic_gpr10
    }
    #[doc = "0x2c - Generic control"]
    #[inline(always)]
    pub const fn pmic_gpr11(&self) -> &PMIC_GPR11 {
        &self.pmic_gpr11
    }
    #[doc = "0x30 - Generic control"]
    #[inline(always)]
    pub const fn pmic_gpr12(&self) -> &PMIC_GPR12 {
        &self.pmic_gpr12
    }
    #[doc = "0x34 - Generic control"]
    #[inline(always)]
    pub const fn pmic_gpr13(&self) -> &PMIC_GPR13 {
        &self.pmic_gpr13
    }
    #[doc = "0x38 - Generic control"]
    #[inline(always)]
    pub const fn pmic_gpr14(&self) -> &PMIC_GPR14 {
        &self.pmic_gpr14
    }
    #[doc = "0x3c - Generic control"]
    #[inline(always)]
    pub const fn pmic_gpr15(&self) -> &PMIC_GPR15 {
        &self.pmic_gpr15
    }
}
#[doc = "PMIC_GPR00 (rw) register accessor: Generic control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pmic_gpr00::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pmic_gpr00::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pmic_gpr00`]
module"]
pub type PMIC_GPR00 = crate::Reg<pmic_gpr00::PMIC_GPR00_SPEC>;
#[doc = "Generic control"]
pub mod pmic_gpr00;
#[doc = "PMIC_GPR01 (rw) register accessor: Generic control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pmic_gpr01::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pmic_gpr01::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pmic_gpr01`]
module"]
pub type PMIC_GPR01 = crate::Reg<pmic_gpr01::PMIC_GPR01_SPEC>;
#[doc = "Generic control"]
pub mod pmic_gpr01;
#[doc = "PMIC_GPR02 (rw) register accessor: Generic control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pmic_gpr02::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pmic_gpr02::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pmic_gpr02`]
module"]
pub type PMIC_GPR02 = crate::Reg<pmic_gpr02::PMIC_GPR02_SPEC>;
#[doc = "Generic control"]
pub mod pmic_gpr02;
#[doc = "PMIC_GPR03 (rw) register accessor: Generic control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pmic_gpr03::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pmic_gpr03::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pmic_gpr03`]
module"]
pub type PMIC_GPR03 = crate::Reg<pmic_gpr03::PMIC_GPR03_SPEC>;
#[doc = "Generic control"]
pub mod pmic_gpr03;
#[doc = "PMIC_GPR04 (rw) register accessor: Generic control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pmic_gpr04::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pmic_gpr04::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pmic_gpr04`]
module"]
pub type PMIC_GPR04 = crate::Reg<pmic_gpr04::PMIC_GPR04_SPEC>;
#[doc = "Generic control"]
pub mod pmic_gpr04;
#[doc = "PMIC_GPR05 (rw) register accessor: Generic control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pmic_gpr05::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pmic_gpr05::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pmic_gpr05`]
module"]
pub type PMIC_GPR05 = crate::Reg<pmic_gpr05::PMIC_GPR05_SPEC>;
#[doc = "Generic control"]
pub mod pmic_gpr05;
#[doc = "PMIC_GPR06 (rw) register accessor: Generic control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pmic_gpr06::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pmic_gpr06::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pmic_gpr06`]
module"]
pub type PMIC_GPR06 = crate::Reg<pmic_gpr06::PMIC_GPR06_SPEC>;
#[doc = "Generic control"]
pub mod pmic_gpr06;
#[doc = "PMIC_GPR07 (rw) register accessor: Generic control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pmic_gpr07::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pmic_gpr07::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pmic_gpr07`]
module"]
pub type PMIC_GPR07 = crate::Reg<pmic_gpr07::PMIC_GPR07_SPEC>;
#[doc = "Generic control"]
pub mod pmic_gpr07;
#[doc = "PMIC_GPR08 (rw) register accessor: Generic control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pmic_gpr08::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pmic_gpr08::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pmic_gpr08`]
module"]
pub type PMIC_GPR08 = crate::Reg<pmic_gpr08::PMIC_GPR08_SPEC>;
#[doc = "Generic control"]
pub mod pmic_gpr08;
#[doc = "PMIC_GPR09 (rw) register accessor: Generic control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pmic_gpr09::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pmic_gpr09::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pmic_gpr09`]
module"]
pub type PMIC_GPR09 = crate::Reg<pmic_gpr09::PMIC_GPR09_SPEC>;
#[doc = "Generic control"]
pub mod pmic_gpr09;
#[doc = "PMIC_GPR10 (rw) register accessor: Generic control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pmic_gpr10::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pmic_gpr10::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pmic_gpr10`]
module"]
pub type PMIC_GPR10 = crate::Reg<pmic_gpr10::PMIC_GPR10_SPEC>;
#[doc = "Generic control"]
pub mod pmic_gpr10;
#[doc = "PMIC_GPR11 (rw) register accessor: Generic control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pmic_gpr11::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pmic_gpr11::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pmic_gpr11`]
module"]
pub type PMIC_GPR11 = crate::Reg<pmic_gpr11::PMIC_GPR11_SPEC>;
#[doc = "Generic control"]
pub mod pmic_gpr11;
#[doc = "PMIC_GPR12 (rw) register accessor: Generic control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pmic_gpr12::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pmic_gpr12::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pmic_gpr12`]
module"]
pub type PMIC_GPR12 = crate::Reg<pmic_gpr12::PMIC_GPR12_SPEC>;
#[doc = "Generic control"]
pub mod pmic_gpr12;
#[doc = "PMIC_GPR13 (rw) register accessor: Generic control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pmic_gpr13::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pmic_gpr13::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pmic_gpr13`]
module"]
pub type PMIC_GPR13 = crate::Reg<pmic_gpr13::PMIC_GPR13_SPEC>;
#[doc = "Generic control"]
pub mod pmic_gpr13;
#[doc = "PMIC_GPR14 (rw) register accessor: Generic control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pmic_gpr14::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pmic_gpr14::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pmic_gpr14`]
module"]
pub type PMIC_GPR14 = crate::Reg<pmic_gpr14::PMIC_GPR14_SPEC>;
#[doc = "Generic control"]
pub mod pmic_gpr14;
#[doc = "PMIC_GPR15 (rw) register accessor: Generic control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pmic_gpr15::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pmic_gpr15::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pmic_gpr15`]
module"]
pub type PMIC_GPR15 = crate::Reg<pmic_gpr15::PMIC_GPR15_SPEC>;
#[doc = "Generic control"]
pub mod pmic_gpr15;
