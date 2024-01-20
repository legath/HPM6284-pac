#[doc = r"Register block"]
#[repr(C)]
pub struct PAD {
    func_ctl: FUNC_CTL,
    pad_ctl: PAD_CTL,
}
impl PAD {
    #[doc = "0x00 - ALT SELECT"]
    #[inline(always)]
    pub const fn func_ctl(&self) -> &FUNC_CTL {
        &self.func_ctl
    }
    #[doc = "0x04 - PAD SETTINGS"]
    #[inline(always)]
    pub const fn pad_ctl(&self) -> &PAD_CTL {
        &self.pad_ctl
    }
}
#[doc = "FUNC_CTL (rw) register accessor: ALT SELECT\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`func_ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`func_ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func_ctl`]
module"]
pub type FUNC_CTL = crate::Reg<func_ctl::FUNC_CTL_SPEC>;
#[doc = "ALT SELECT"]
pub mod func_ctl;
#[doc = "PAD_CTL (rw) register accessor: PAD SETTINGS\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pad_ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pad_ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pad_ctl`]
module"]
pub type PAD_CTL = crate::Reg<pad_ctl::PAD_CTL_SPEC>;
#[doc = "PAD SETTINGS"]
pub mod pad_ctl;
