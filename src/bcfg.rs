#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    vbg_cfg: VBG_CFG,
    _reserved1: [u8; 0x04],
    irc32k_cfg: IRC32K_CFG,
    xtal32k_cfg: XTAL32K_CFG,
    clk_cfg: CLK_CFG,
}
impl RegisterBlock {
    #[doc = "0x00 - Bandgap config"]
    #[inline(always)]
    pub const fn vbg_cfg(&self) -> &VBG_CFG {
        &self.vbg_cfg
    }
    #[doc = "0x08 - On-chip 32k oscillator config"]
    #[inline(always)]
    pub const fn irc32k_cfg(&self) -> &IRC32K_CFG {
        &self.irc32k_cfg
    }
    #[doc = "0x0c - XTAL 32K config"]
    #[inline(always)]
    pub const fn xtal32k_cfg(&self) -> &XTAL32K_CFG {
        &self.xtal32k_cfg
    }
    #[doc = "0x10 - Clock config"]
    #[inline(always)]
    pub const fn clk_cfg(&self) -> &CLK_CFG {
        &self.clk_cfg
    }
}
#[doc = "VBG_CFG (rw) register accessor: Bandgap config\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vbg_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vbg_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vbg_cfg`]
module"]
pub type VBG_CFG = crate::Reg<vbg_cfg::VBG_CFG_SPEC>;
#[doc = "Bandgap config"]
pub mod vbg_cfg;
#[doc = "IRC32K_CFG (rw) register accessor: On-chip 32k oscillator config\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`irc32k_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`irc32k_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@irc32k_cfg`]
module"]
pub type IRC32K_CFG = crate::Reg<irc32k_cfg::IRC32K_CFG_SPEC>;
#[doc = "On-chip 32k oscillator config"]
pub mod irc32k_cfg;
#[doc = "XTAL32K_CFG (rw) register accessor: XTAL 32K config\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`xtal32k_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`xtal32k_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@xtal32k_cfg`]
module"]
pub type XTAL32K_CFG = crate::Reg<xtal32k_cfg::XTAL32K_CFG_SPEC>;
#[doc = "XTAL 32K config"]
pub mod xtal32k_cfg;
#[doc = "CLK_CFG (rw) register accessor: Clock config\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_cfg`]
module"]
pub type CLK_CFG = crate::Reg<clk_cfg::CLK_CFG_SPEC>;
#[doc = "Clock config"]
pub mod clk_cfg;
