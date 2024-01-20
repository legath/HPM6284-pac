#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    cfg0: CFG0,
    cfg1: CFG1,
    cfg2: CFG2,
    _reserved3: [u8; 0x04],
    step_cfg: [STEP_CFG; 4],
    buf_addr: [BUF_ADDR; 2],
    buf_length: BUF_LENGTH,
    _reserved6: [u8; 0x04],
    irq_sts: IRQ_STS,
    irq_en: IRQ_EN,
    dma_en: DMA_EN,
    _reserved9: [u8; 0x04],
    ana_cfg0: ANA_CFG0,
    cfg0_bak: CFG0_BAK,
    status0: STATUS0,
}
impl RegisterBlock {
    #[doc = "0x00 - No description avaiable"]
    #[inline(always)]
    pub const fn cfg0(&self) -> &CFG0 {
        &self.cfg0
    }
    #[doc = "0x04 - No description avaiable"]
    #[inline(always)]
    pub const fn cfg1(&self) -> &CFG1 {
        &self.cfg1
    }
    #[doc = "0x08 - No description avaiable"]
    #[inline(always)]
    pub const fn cfg2(&self) -> &CFG2 {
        &self.cfg2
    }
    #[doc = "0x10..0x20 - no description available"]
    #[inline(always)]
    pub const fn step_cfg(&self, n: usize) -> &STEP_CFG {
        &self.step_cfg[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x10..0x20 - no description available"]
    #[inline(always)]
    pub fn step_cfg_iter(&self) -> impl Iterator<Item = &STEP_CFG> {
        self.step_cfg.iter()
    }
    #[doc = "0x10 - no description available"]
    #[inline(always)]
    pub const fn step_cfgstep0(&self) -> &STEP_CFG {
        self.step_cfg(0)
    }
    #[doc = "0x14 - no description available"]
    #[inline(always)]
    pub const fn step_cfgstep1(&self) -> &STEP_CFG {
        self.step_cfg(1)
    }
    #[doc = "0x18 - no description available"]
    #[inline(always)]
    pub const fn step_cfgstep2(&self) -> &STEP_CFG {
        self.step_cfg(2)
    }
    #[doc = "0x1c - no description available"]
    #[inline(always)]
    pub const fn step_cfgstep3(&self) -> &STEP_CFG {
        self.step_cfg(3)
    }
    #[doc = "0x20..0x28 - no description available"]
    #[inline(always)]
    pub const fn buf_addr(&self, n: usize) -> &BUF_ADDR {
        &self.buf_addr[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x20..0x28 - no description available"]
    #[inline(always)]
    pub fn buf_addr_iter(&self) -> impl Iterator<Item = &BUF_ADDR> {
        self.buf_addr.iter()
    }
    #[doc = "0x20 - no description available"]
    #[inline(always)]
    pub const fn buf_addrbuf0(&self) -> &BUF_ADDR {
        self.buf_addr(0)
    }
    #[doc = "0x24 - no description available"]
    #[inline(always)]
    pub const fn buf_addrbuf1(&self) -> &BUF_ADDR {
        self.buf_addr(1)
    }
    #[doc = "0x28 - No description avaiable"]
    #[inline(always)]
    pub const fn buf_length(&self) -> &BUF_LENGTH {
        &self.buf_length
    }
    #[doc = "0x30 - No description avaiable"]
    #[inline(always)]
    pub const fn irq_sts(&self) -> &IRQ_STS {
        &self.irq_sts
    }
    #[doc = "0x34 - No description avaiable"]
    #[inline(always)]
    pub const fn irq_en(&self) -> &IRQ_EN {
        &self.irq_en
    }
    #[doc = "0x38 - No description avaiable"]
    #[inline(always)]
    pub const fn dma_en(&self) -> &DMA_EN {
        &self.dma_en
    }
    #[doc = "0x40 - No description avaiable"]
    #[inline(always)]
    pub const fn ana_cfg0(&self) -> &ANA_CFG0 {
        &self.ana_cfg0
    }
    #[doc = "0x44 - No description avaiable"]
    #[inline(always)]
    pub const fn cfg0_bak(&self) -> &CFG0_BAK {
        &self.cfg0_bak
    }
    #[doc = "0x48 - No description avaiable"]
    #[inline(always)]
    pub const fn status0(&self) -> &STATUS0 {
        &self.status0
    }
}
#[doc = "cfg0 (rw) register accessor: No description avaiable\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0`]
module"]
pub type CFG0 = crate::Reg<cfg0::CFG0_SPEC>;
#[doc = "No description avaiable"]
pub mod cfg0;
#[doc = "cfg1 (rw) register accessor: No description avaiable\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg1`]
module"]
pub type CFG1 = crate::Reg<cfg1::CFG1_SPEC>;
#[doc = "No description avaiable"]
pub mod cfg1;
#[doc = "cfg2 (rw) register accessor: No description avaiable\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg2`]
module"]
pub type CFG2 = crate::Reg<cfg2::CFG2_SPEC>;
#[doc = "No description avaiable"]
pub mod cfg2;
#[doc = "STEP_CFG (rw) register accessor: no description available\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`step_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`step_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@step_cfg`]
module"]
pub type STEP_CFG = crate::Reg<step_cfg::STEP_CFG_SPEC>;
#[doc = "no description available"]
pub mod step_cfg;
#[doc = "BUF_ADDR (rw) register accessor: no description available\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`buf_addr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`buf_addr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@buf_addr`]
module"]
pub type BUF_ADDR = crate::Reg<buf_addr::BUF_ADDR_SPEC>;
#[doc = "no description available"]
pub mod buf_addr;
#[doc = "buf_length (rw) register accessor: No description avaiable\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`buf_length::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`buf_length::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@buf_length`]
module"]
pub type BUF_LENGTH = crate::Reg<buf_length::BUF_LENGTH_SPEC>;
#[doc = "No description avaiable"]
pub mod buf_length;
#[doc = "irq_sts (rw) register accessor: No description avaiable\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`irq_sts::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`irq_sts::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@irq_sts`]
module"]
pub type IRQ_STS = crate::Reg<irq_sts::IRQ_STS_SPEC>;
#[doc = "No description avaiable"]
pub mod irq_sts;
#[doc = "irq_en (rw) register accessor: No description avaiable\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`irq_en::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`irq_en::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@irq_en`]
module"]
pub type IRQ_EN = crate::Reg<irq_en::IRQ_EN_SPEC>;
#[doc = "No description avaiable"]
pub mod irq_en;
#[doc = "dma_en (rw) register accessor: No description avaiable\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma_en::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma_en::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_en`]
module"]
pub type DMA_EN = crate::Reg<dma_en::DMA_EN_SPEC>;
#[doc = "No description avaiable"]
pub mod dma_en;
#[doc = "ana_cfg0 (rw) register accessor: No description avaiable\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ana_cfg0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ana_cfg0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ana_cfg0`]
module"]
pub type ANA_CFG0 = crate::Reg<ana_cfg0::ANA_CFG0_SPEC>;
#[doc = "No description avaiable"]
pub mod ana_cfg0;
#[doc = "cfg0_bak (rw) register accessor: No description avaiable\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_bak::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_bak::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0_bak`]
module"]
pub type CFG0_BAK = crate::Reg<cfg0_bak::CFG0_BAK_SPEC>;
#[doc = "No description avaiable"]
pub mod cfg0_bak;
#[doc = "status0 (rw) register accessor: No description avaiable\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`status0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`status0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@status0`]
module"]
pub type STATUS0 = crate::Reg<status0::STATUS0_SPEC>;
#[doc = "No description avaiable"]
pub mod status0;
