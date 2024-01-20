#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    cr: CR,
    phcfg: PHCFG,
    wdgcfg: WDGCFG,
    uvwcfg: UVWCFG,
    trgoen: TRGOEN,
    readen: READEN,
    _reserved6: [u8; 0x0c],
    dmaen: DMAEN,
    sr: SR,
    irqen: IRQEN,
    count: [COUNT; 4],
    his: [HIS; 3],
}
impl RegisterBlock {
    #[doc = "0x00 - Control Register"]
    #[inline(always)]
    pub const fn cr(&self) -> &CR {
        &self.cr
    }
    #[doc = "0x04 - Phase configure register"]
    #[inline(always)]
    pub const fn phcfg(&self) -> &PHCFG {
        &self.phcfg
    }
    #[doc = "0x08 - Watchdog configure register"]
    #[inline(always)]
    pub const fn wdgcfg(&self) -> &WDGCFG {
        &self.wdgcfg
    }
    #[doc = "0x0c - U,V,W configure register"]
    #[inline(always)]
    pub const fn uvwcfg(&self) -> &UVWCFG {
        &self.uvwcfg
    }
    #[doc = "0x10 - Trigger output enable register"]
    #[inline(always)]
    pub const fn trgoen(&self) -> &TRGOEN {
        &self.trgoen
    }
    #[doc = "0x14 - Read event enable register"]
    #[inline(always)]
    pub const fn readen(&self) -> &READEN {
        &self.readen
    }
    #[doc = "0x24 - DMA enable register"]
    #[inline(always)]
    pub const fn dmaen(&self) -> &DMAEN {
        &self.dmaen
    }
    #[doc = "0x28 - Status register"]
    #[inline(always)]
    pub const fn sr(&self) -> &SR {
        &self.sr
    }
    #[doc = "0x2c - Interrupt request enable register"]
    #[inline(always)]
    pub const fn irqen(&self) -> &IRQEN {
        &self.irqen
    }
    #[doc = "0x30..0x70 - no description available"]
    #[inline(always)]
    pub const fn count(&self, n: usize) -> &COUNT {
        &self.count[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x30..0x70 - no description available"]
    #[inline(always)]
    pub fn count_iter(&self) -> impl Iterator<Item = &COUNT> {
        self.count.iter()
    }
    #[doc = "0x30..0x40 - no description available"]
    #[inline(always)]
    pub const fn countcurrent(&self) -> &COUNT {
        self.count(0)
    }
    #[doc = "0x40..0x50 - no description available"]
    #[inline(always)]
    pub const fn countread(&self) -> &COUNT {
        self.count(1)
    }
    #[doc = "0x50..0x60 - no description available"]
    #[inline(always)]
    pub const fn countsnap0(&self) -> &COUNT {
        self.count(2)
    }
    #[doc = "0x60..0x70 - no description available"]
    #[inline(always)]
    pub const fn countsnap1(&self) -> &COUNT {
        self.count(3)
    }
    #[doc = "0x70..0x88 - no description available"]
    #[inline(always)]
    pub const fn his(&self, n: usize) -> &HIS {
        &self.his[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x70..0x88 - no description available"]
    #[inline(always)]
    pub fn his_iter(&self) -> impl Iterator<Item = &HIS> {
        self.his.iter()
    }
    #[doc = "0x70..0x78 - no description available"]
    #[inline(always)]
    pub const fn hisu(&self) -> &HIS {
        self.his(0)
    }
    #[doc = "0x78..0x80 - no description available"]
    #[inline(always)]
    pub const fn hisv(&self) -> &HIS {
        self.his(1)
    }
    #[doc = "0x80..0x88 - no description available"]
    #[inline(always)]
    pub const fn hisw(&self) -> &HIS {
        self.his(2)
    }
}
#[doc = "cr (rw) register accessor: Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr`]
module"]
pub type CR = crate::Reg<cr::CR_SPEC>;
#[doc = "Control Register"]
pub mod cr;
#[doc = "phcfg (rw) register accessor: Phase configure register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`phcfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`phcfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@phcfg`]
module"]
pub type PHCFG = crate::Reg<phcfg::PHCFG_SPEC>;
#[doc = "Phase configure register"]
pub mod phcfg;
#[doc = "wdgcfg (rw) register accessor: Watchdog configure register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wdgcfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wdgcfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wdgcfg`]
module"]
pub type WDGCFG = crate::Reg<wdgcfg::WDGCFG_SPEC>;
#[doc = "Watchdog configure register"]
pub mod wdgcfg;
#[doc = "uvwcfg (rw) register accessor: U,V,W configure register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uvwcfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uvwcfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uvwcfg`]
module"]
pub type UVWCFG = crate::Reg<uvwcfg::UVWCFG_SPEC>;
#[doc = "U,V,W configure register"]
pub mod uvwcfg;
#[doc = "trgoen (rw) register accessor: Trigger output enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`trgoen::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`trgoen::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@trgoen`]
module"]
pub type TRGOEN = crate::Reg<trgoen::TRGOEN_SPEC>;
#[doc = "Trigger output enable register"]
pub mod trgoen;
#[doc = "readen (rw) register accessor: Read event enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`readen::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`readen::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@readen`]
module"]
pub type READEN = crate::Reg<readen::READEN_SPEC>;
#[doc = "Read event enable register"]
pub mod readen;
#[doc = "dmaen (rw) register accessor: DMA enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmaen::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmaen::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmaen`]
module"]
pub type DMAEN = crate::Reg<dmaen::DMAEN_SPEC>;
#[doc = "DMA enable register"]
pub mod dmaen;
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
#[doc = "no description available"]
pub use self::count::COUNT;
#[doc = r"Cluster"]
#[doc = "no description available"]
pub mod count;
#[doc = "no description available"]
pub use self::his::HIS;
#[doc = r"Cluster"]
#[doc = "no description available"]
pub mod his;
