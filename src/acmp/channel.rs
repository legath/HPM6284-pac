#[doc = r"Register block"]
#[repr(C)]
pub struct CHANNEL {
    cfg: CFG,
    daccfg: DACCFG,
    _reserved2: [u8; 0x08],
    sr: SR,
    irqen: IRQEN,
    dmaen: DMAEN,
}
impl CHANNEL {
    #[doc = "0x00 - Configure Register"]
    #[inline(always)]
    pub const fn cfg(&self) -> &CFG {
        &self.cfg
    }
    #[doc = "0x04 - DAC configure register"]
    #[inline(always)]
    pub const fn daccfg(&self) -> &DACCFG {
        &self.daccfg
    }
    #[doc = "0x10 - Status register"]
    #[inline(always)]
    pub const fn sr(&self) -> &SR {
        &self.sr
    }
    #[doc = "0x14 - Interrupt request enable register"]
    #[inline(always)]
    pub const fn irqen(&self) -> &IRQEN {
        &self.irqen
    }
    #[doc = "0x18 - DMA request enable register"]
    #[inline(always)]
    pub const fn dmaen(&self) -> &DMAEN {
        &self.dmaen
    }
}
#[doc = "cfg (rw) register accessor: Configure Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub type CFG = crate::Reg<cfg::CFG_SPEC>;
#[doc = "Configure Register"]
pub mod cfg;
#[doc = "daccfg (rw) register accessor: DAC configure register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`daccfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`daccfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@daccfg`]
module"]
pub type DACCFG = crate::Reg<daccfg::DACCFG_SPEC>;
#[doc = "DAC configure register"]
pub mod daccfg;
#[doc = "sr (rw) register accessor: Status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sr`]
module"]
pub type SR = crate::Reg<sr::SR_SPEC>;
#[doc = "Status register"]
pub mod sr;
#[doc = "irqen (rw) register accessor: Interrupt request enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`irqen::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`irqen::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@irqen`]
module"]
pub type IRQEN = crate::Reg<irqen::IRQEN_SPEC>;
#[doc = "Interrupt request enable register"]
pub mod irqen;
#[doc = "dmaen (rw) register accessor: DMA request enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmaen::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmaen::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmaen`]
module"]
pub type DMAEN = crate::Reg<dmaen::DMAEN_SPEC>;
#[doc = "DMA request enable register"]
pub mod dmaen;
