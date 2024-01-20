#[doc = r"Register block"]
#[repr(C)]
pub struct MONITOR {
    control: CONTROL,
    status: STATUS,
}
impl MONITOR {
    #[doc = "0x00 - Glitch and clock monitor control"]
    #[inline(always)]
    pub const fn control(&self) -> &CONTROL {
        &self.control
    }
    #[doc = "0x04 - Glitch and clock monitor status"]
    #[inline(always)]
    pub const fn status(&self) -> &STATUS {
        &self.status
    }
}
#[doc = "CONTROL (rw) register accessor: Glitch and clock monitor control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`control::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`control::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@control`]
module"]
pub type CONTROL = crate::Reg<control::CONTROL_SPEC>;
#[doc = "Glitch and clock monitor control"]
pub mod control;
#[doc = "STATUS (rw) register accessor: Glitch and clock monitor status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`status::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`status::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@status`]
module"]
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
#[doc = "Glitch and clock monitor status"]
pub mod status;
