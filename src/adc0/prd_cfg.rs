#[doc = r"Register block"]
#[repr(C)]
pub struct PRD_CFG {
    prd_cfg: PRD_CFG,
    prd_thshd_cfg: PRD_THSHD_CFG,
    prd_result: PRD_RESULT,
}
impl PRD_CFG {
    #[doc = "0x00 - No description avaiable"]
    #[inline(always)]
    pub const fn prd_cfg(&self) -> &PRD_CFG {
        &self.prd_cfg
    }
    #[doc = "0x04 - No description avaiable"]
    #[inline(always)]
    pub const fn prd_thshd_cfg(&self) -> &PRD_THSHD_CFG {
        &self.prd_thshd_cfg
    }
    #[doc = "0x08 - No description avaiable"]
    #[inline(always)]
    pub const fn prd_result(&self) -> &PRD_RESULT {
        &self.prd_result
    }
}
#[doc = "prd_cfg (rw) register accessor: No description avaiable\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`prd_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`prd_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prd_cfg`]
module"]
pub type PRD_CFG = crate::Reg<prd_cfg::PRD_CFG_SPEC>;
#[doc = "No description avaiable"]
pub mod prd_cfg;
#[doc = "prd_thshd_cfg (rw) register accessor: No description avaiable\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`prd_thshd_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`prd_thshd_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prd_thshd_cfg`]
module"]
pub type PRD_THSHD_CFG = crate::Reg<prd_thshd_cfg::PRD_THSHD_CFG_SPEC>;
#[doc = "No description avaiable"]
pub mod prd_thshd_cfg;
#[doc = "prd_result (rw) register accessor: No description avaiable\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`prd_result::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`prd_result::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prd_result`]
module"]
pub type PRD_RESULT = crate::Reg<prd_result::PRD_RESULT_SPEC>;
#[doc = "No description avaiable"]
pub mod prd_result;
