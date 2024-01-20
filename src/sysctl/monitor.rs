#[doc = r"Register block"]
#[repr(C)]
pub struct MONITOR {
    control: CONTROL,
    current: CURRENT,
    low_limit: LOW_LIMIT,
    high_limit: HIGH_LIMIT,
}
impl MONITOR {
    #[doc = "0x00 - Clock measure and monitor control"]
    #[inline(always)]
    pub const fn control(&self) -> &CONTROL {
        &self.control
    }
    #[doc = "0x04 - Clock measure result"]
    #[inline(always)]
    pub const fn current(&self) -> &CURRENT {
        &self.current
    }
    #[doc = "0x08 - Clock lower limit"]
    #[inline(always)]
    pub const fn low_limit(&self) -> &LOW_LIMIT {
        &self.low_limit
    }
    #[doc = "0x0c - Clock upper limit"]
    #[inline(always)]
    pub const fn high_limit(&self) -> &HIGH_LIMIT {
        &self.high_limit
    }
}
#[doc = "control (rw) register accessor: Clock measure and monitor control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`control::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`control::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@control`]
module"]
pub type CONTROL = crate::Reg<control::CONTROL_SPEC>;
#[doc = "Clock measure and monitor control"]
pub mod control;
#[doc = "current (rw) register accessor: Clock measure result\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`current::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`current::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@current`]
module"]
pub type CURRENT = crate::Reg<current::CURRENT_SPEC>;
#[doc = "Clock measure result"]
pub mod current;
#[doc = "low_limit (rw) register accessor: Clock lower limit\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`low_limit::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`low_limit::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@low_limit`]
module"]
pub type LOW_LIMIT = crate::Reg<low_limit::LOW_LIMIT_SPEC>;
#[doc = "Clock lower limit"]
pub mod low_limit;
#[doc = "high_limit (rw) register accessor: Clock upper limit\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`high_limit::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`high_limit::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@high_limit`]
module"]
pub type HIGH_LIMIT = crate::Reg<high_limit::HIGH_LIMIT_SPEC>;
#[doc = "Clock upper limit"]
pub mod high_limit;
