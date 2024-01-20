#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    por_cause: POR_CAUSE,
    por_select: POR_SELECT,
    por_config: POR_CONFIG,
    por_control: POR_CONTROL,
}
impl RegisterBlock {
    #[doc = "0x00 - Power on cause"]
    #[inline(always)]
    pub const fn por_cause(&self) -> &POR_CAUSE {
        &self.por_cause
    }
    #[doc = "0x04 - Power on select"]
    #[inline(always)]
    pub const fn por_select(&self) -> &POR_SELECT {
        &self.por_select
    }
    #[doc = "0x08 - Power on reset config"]
    #[inline(always)]
    pub const fn por_config(&self) -> &POR_CONFIG {
        &self.por_config
    }
    #[doc = "0x0c - Power down control"]
    #[inline(always)]
    pub const fn por_control(&self) -> &POR_CONTROL {
        &self.por_control
    }
}
#[doc = "POR_CAUSE (rw) register accessor: Power on cause\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`por_cause::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`por_cause::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@por_cause`]
module"]
pub type POR_CAUSE = crate::Reg<por_cause::POR_CAUSE_SPEC>;
#[doc = "Power on cause"]
pub mod por_cause;
#[doc = "POR_SELECT (rw) register accessor: Power on select\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`por_select::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`por_select::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@por_select`]
module"]
pub type POR_SELECT = crate::Reg<por_select::POR_SELECT_SPEC>;
#[doc = "Power on select"]
pub mod por_select;
#[doc = "POR_CONFIG (rw) register accessor: Power on reset config\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`por_config::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`por_config::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@por_config`]
module"]
pub type POR_CONFIG = crate::Reg<por_config::POR_CONFIG_SPEC>;
#[doc = "Power on reset config"]
pub mod por_config;
#[doc = "POR_CONTROL (rw) register accessor: Power down control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`por_control::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`por_control::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@por_control`]
module"]
pub type POR_CONTROL = crate::Reg<por_control::POR_CONTROL_SPEC>;
#[doc = "Power down control"]
pub mod por_control;
