#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    secure_state: SECURE_STATE,
    secure_state_config: SECURE_STATE_CONFIG,
    violation_config: VIOLATION_CONFIG,
    escalate_config: ESCALATE_CONFIG,
    event: EVENT,
}
impl RegisterBlock {
    #[doc = "0x00 - Secure state"]
    #[inline(always)]
    pub const fn secure_state(&self) -> &SECURE_STATE {
        &self.secure_state
    }
    #[doc = "0x04 - secure state configuration"]
    #[inline(always)]
    pub const fn secure_state_config(&self) -> &SECURE_STATE_CONFIG {
        &self.secure_state_config
    }
    #[doc = "0x08 - Security violation config"]
    #[inline(always)]
    pub const fn violation_config(&self) -> &VIOLATION_CONFIG {
        &self.violation_config
    }
    #[doc = "0x0c - Escalate behavior on security event"]
    #[inline(always)]
    pub const fn escalate_config(&self) -> &ESCALATE_CONFIG {
        &self.escalate_config
    }
    #[doc = "0x10 - Event and escalate status"]
    #[inline(always)]
    pub const fn event(&self) -> &EVENT {
        &self.event
    }
}
#[doc = "SECURE_STATE (rw) register accessor: Secure state\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`secure_state::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`secure_state::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@secure_state`]
module"]
pub type SECURE_STATE = crate::Reg<secure_state::SECURE_STATE_SPEC>;
#[doc = "Secure state"]
pub mod secure_state;
#[doc = "SECURE_STATE_CONFIG (rw) register accessor: secure state configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`secure_state_config::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`secure_state_config::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@secure_state_config`]
module"]
pub type SECURE_STATE_CONFIG = crate::Reg<secure_state_config::SECURE_STATE_CONFIG_SPEC>;
#[doc = "secure state configuration"]
pub mod secure_state_config;
#[doc = "VIOLATION_CONFIG (rw) register accessor: Security violation config\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`violation_config::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`violation_config::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@violation_config`]
module"]
pub type VIOLATION_CONFIG = crate::Reg<violation_config::VIOLATION_CONFIG_SPEC>;
#[doc = "Security violation config"]
pub mod violation_config;
#[doc = "ESCALATE_CONFIG (rw) register accessor: Escalate behavior on security event\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`escalate_config::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`escalate_config::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@escalate_config`]
module"]
pub type ESCALATE_CONFIG = crate::Reg<escalate_config::ESCALATE_CONFIG_SPEC>;
#[doc = "Escalate behavior on security event"]
pub mod escalate_config;
#[doc = "EVENT (rw) register accessor: Event and escalate status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`event::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`event::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@event`]
module"]
pub type EVENT = crate::Reg<event::EVENT_SPEC>;
#[doc = "Event and escalate status"]
pub mod event;
