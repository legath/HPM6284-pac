#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    batt_gpr0: BATT_GPR0,
    batt_gpr1: BATT_GPR1,
    batt_gpr2: BATT_GPR2,
    batt_gpr3: BATT_GPR3,
    batt_gpr4: BATT_GPR4,
    batt_gpr5: BATT_GPR5,
    batt_gpr6: BATT_GPR6,
    batt_gpr7: BATT_GPR7,
}
impl RegisterBlock {
    #[doc = "0x00 - Generic control"]
    #[inline(always)]
    pub const fn batt_gpr0(&self) -> &BATT_GPR0 {
        &self.batt_gpr0
    }
    #[doc = "0x04 - Generic control"]
    #[inline(always)]
    pub const fn batt_gpr1(&self) -> &BATT_GPR1 {
        &self.batt_gpr1
    }
    #[doc = "0x08 - Generic control"]
    #[inline(always)]
    pub const fn batt_gpr2(&self) -> &BATT_GPR2 {
        &self.batt_gpr2
    }
    #[doc = "0x0c - Generic control"]
    #[inline(always)]
    pub const fn batt_gpr3(&self) -> &BATT_GPR3 {
        &self.batt_gpr3
    }
    #[doc = "0x10 - Generic control"]
    #[inline(always)]
    pub const fn batt_gpr4(&self) -> &BATT_GPR4 {
        &self.batt_gpr4
    }
    #[doc = "0x14 - Generic control"]
    #[inline(always)]
    pub const fn batt_gpr5(&self) -> &BATT_GPR5 {
        &self.batt_gpr5
    }
    #[doc = "0x18 - Generic control"]
    #[inline(always)]
    pub const fn batt_gpr6(&self) -> &BATT_GPR6 {
        &self.batt_gpr6
    }
    #[doc = "0x1c - Generic control"]
    #[inline(always)]
    pub const fn batt_gpr7(&self) -> &BATT_GPR7 {
        &self.batt_gpr7
    }
}
#[doc = "BATT_GPR0 (rw) register accessor: Generic control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`batt_gpr0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`batt_gpr0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@batt_gpr0`]
module"]
pub type BATT_GPR0 = crate::Reg<batt_gpr0::BATT_GPR0_SPEC>;
#[doc = "Generic control"]
pub mod batt_gpr0;
#[doc = "BATT_GPR1 (rw) register accessor: Generic control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`batt_gpr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`batt_gpr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@batt_gpr1`]
module"]
pub type BATT_GPR1 = crate::Reg<batt_gpr1::BATT_GPR1_SPEC>;
#[doc = "Generic control"]
pub mod batt_gpr1;
#[doc = "BATT_GPR2 (rw) register accessor: Generic control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`batt_gpr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`batt_gpr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@batt_gpr2`]
module"]
pub type BATT_GPR2 = crate::Reg<batt_gpr2::BATT_GPR2_SPEC>;
#[doc = "Generic control"]
pub mod batt_gpr2;
#[doc = "BATT_GPR3 (rw) register accessor: Generic control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`batt_gpr3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`batt_gpr3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@batt_gpr3`]
module"]
pub type BATT_GPR3 = crate::Reg<batt_gpr3::BATT_GPR3_SPEC>;
#[doc = "Generic control"]
pub mod batt_gpr3;
#[doc = "BATT_GPR4 (rw) register accessor: Generic control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`batt_gpr4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`batt_gpr4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@batt_gpr4`]
module"]
pub type BATT_GPR4 = crate::Reg<batt_gpr4::BATT_GPR4_SPEC>;
#[doc = "Generic control"]
pub mod batt_gpr4;
#[doc = "BATT_GPR5 (rw) register accessor: Generic control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`batt_gpr5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`batt_gpr5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@batt_gpr5`]
module"]
pub type BATT_GPR5 = crate::Reg<batt_gpr5::BATT_GPR5_SPEC>;
#[doc = "Generic control"]
pub mod batt_gpr5;
#[doc = "BATT_GPR6 (rw) register accessor: Generic control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`batt_gpr6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`batt_gpr6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@batt_gpr6`]
module"]
pub type BATT_GPR6 = crate::Reg<batt_gpr6::BATT_GPR6_SPEC>;
#[doc = "Generic control"]
pub mod batt_gpr6;
#[doc = "BATT_GPR7 (rw) register accessor: Generic control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`batt_gpr7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`batt_gpr7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@batt_gpr7`]
module"]
pub type BATT_GPR7 = crate::Reg<batt_gpr7::BATT_GPR7_SPEC>;
#[doc = "Generic control"]
pub mod batt_gpr7;
