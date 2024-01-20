#[doc = r"Register block"]
#[repr(C)]
pub struct RESET {
    control: CONTROL,
    config: CONFIG,
    _reserved2: [u8; 0x04],
    counter: COUNTER,
}
impl RESET {
    #[doc = "0x00 - Reset Setting"]
    #[inline(always)]
    pub const fn control(&self) -> &CONTROL {
        &self.control
    }
    #[doc = "0x04 - Reset Setting"]
    #[inline(always)]
    pub const fn config(&self) -> &CONFIG {
        &self.config
    }
    #[doc = "0x0c - Reset Setting"]
    #[inline(always)]
    pub const fn counter(&self) -> &COUNTER {
        &self.counter
    }
}
#[doc = "control (rw) register accessor: Reset Setting\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`control::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`control::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@control`]
module"]
pub type CONTROL = crate::Reg<control::CONTROL_SPEC>;
#[doc = "Reset Setting"]
pub mod control;
#[doc = "config (rw) register accessor: Reset Setting\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`config::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`config::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@config`]
module"]
pub type CONFIG = crate::Reg<config::CONFIG_SPEC>;
#[doc = "Reset Setting"]
pub mod config;
#[doc = "counter (rw) register accessor: Reset Setting\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`counter::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`counter::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@counter`]
module"]
pub type COUNTER = crate::Reg<counter::COUNTER_SPEC>;
#[doc = "Reset Setting"]
pub mod counter;
