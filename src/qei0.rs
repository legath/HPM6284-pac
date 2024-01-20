#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    cr: CR,
    phcfg: PHCFG,
    wdgcfg: WDGCFG,
    phidx: PHIDX,
    trgoen: TRGOEN,
    readen: READEN,
    zcmp: ZCMP,
    phcmp: PHCMP,
    spdcmp: SPDCMP,
    dmaen: DMAEN,
    sr: SR,
    irqen: IRQEN,
    count: [COUNT; 4],
    spdhis: [SPDHIS; 4],
}
impl RegisterBlock {
    #[doc = "0x00 - Control register"]
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
    #[doc = "0x0c - Phase index register"]
    #[inline(always)]
    pub const fn phidx(&self) -> &PHIDX {
        &self.phidx
    }
    #[doc = "0x10 - Tigger output enable register"]
    #[inline(always)]
    pub const fn trgoen(&self) -> &TRGOEN {
        &self.trgoen
    }
    #[doc = "0x14 - Read event enable register"]
    #[inline(always)]
    pub const fn readen(&self) -> &READEN {
        &self.readen
    }
    #[doc = "0x18 - Z comparator"]
    #[inline(always)]
    pub const fn zcmp(&self) -> &ZCMP {
        &self.zcmp
    }
    #[doc = "0x1c - Phase comparator"]
    #[inline(always)]
    pub const fn phcmp(&self) -> &PHCMP {
        &self.phcmp
    }
    #[doc = "0x20 - Speed comparator"]
    #[inline(always)]
    pub const fn spdcmp(&self) -> &SPDCMP {
        &self.spdcmp
    }
    #[doc = "0x24 - DMA request enable register"]
    #[inline(always)]
    pub const fn dmaen(&self) -> &DMAEN {
        &self.dmaen
    }
    #[doc = "0x28 - Status register"]
    #[inline(always)]
    pub const fn sr(&self) -> &SR {
        &self.sr
    }
    #[doc = "0x2c - Interrupt request register"]
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
    #[doc = "0x70..0x80 - no description available"]
    #[inline(always)]
    pub const fn spdhis(&self, n: usize) -> &SPDHIS {
        &self.spdhis[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x70..0x80 - no description available"]
    #[inline(always)]
    pub fn spdhis_iter(&self) -> impl Iterator<Item = &SPDHIS> {
        self.spdhis.iter()
    }
    #[doc = "0x70 - no description available"]
    #[inline(always)]
    pub const fn spdhisspdhis0(&self) -> &SPDHIS {
        self.spdhis(0)
    }
    #[doc = "0x74 - no description available"]
    #[inline(always)]
    pub const fn spdhisspdhis1(&self) -> &SPDHIS {
        self.spdhis(1)
    }
    #[doc = "0x78 - no description available"]
    #[inline(always)]
    pub const fn spdhisspdhis2(&self) -> &SPDHIS {
        self.spdhis(2)
    }
    #[doc = "0x7c - no description available"]
    #[inline(always)]
    pub const fn spdhisspdhis3(&self) -> &SPDHIS {
        self.spdhis(3)
    }
}
#[doc = "cr (rw) register accessor: Control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr`]
module"]
pub type CR = crate::Reg<cr::CR_SPEC>;
#[doc = "Control register"]
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
#[doc = "phidx (rw) register accessor: Phase index register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`phidx::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`phidx::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@phidx`]
module"]
pub type PHIDX = crate::Reg<phidx::PHIDX_SPEC>;
#[doc = "Phase index register"]
pub mod phidx;
#[doc = "trgoen (rw) register accessor: Tigger output enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`trgoen::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`trgoen::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@trgoen`]
module"]
pub type TRGOEN = crate::Reg<trgoen::TRGOEN_SPEC>;
#[doc = "Tigger output enable register"]
pub mod trgoen;
#[doc = "readen (rw) register accessor: Read event enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`readen::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`readen::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@readen`]
module"]
pub type READEN = crate::Reg<readen::READEN_SPEC>;
#[doc = "Read event enable register"]
pub mod readen;
#[doc = "zcmp (rw) register accessor: Z comparator\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`zcmp::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`zcmp::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@zcmp`]
module"]
pub type ZCMP = crate::Reg<zcmp::ZCMP_SPEC>;
#[doc = "Z comparator"]
pub mod zcmp;
#[doc = "phcmp (rw) register accessor: Phase comparator\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`phcmp::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`phcmp::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@phcmp`]
module"]
pub type PHCMP = crate::Reg<phcmp::PHCMP_SPEC>;
#[doc = "Phase comparator"]
pub mod phcmp;
#[doc = "spdcmp (rw) register accessor: Speed comparator\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spdcmp::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spdcmp::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spdcmp`]
module"]
pub type SPDCMP = crate::Reg<spdcmp::SPDCMP_SPEC>;
#[doc = "Speed comparator"]
pub mod spdcmp;
#[doc = "dmaen (rw) register accessor: DMA request enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmaen::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmaen::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmaen`]
module"]
pub type DMAEN = crate::Reg<dmaen::DMAEN_SPEC>;
#[doc = "DMA request enable register"]
pub mod dmaen;
#[doc = "sr (rw) register accessor: Status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sr`]
module"]
pub type SR = crate::Reg<sr::SR_SPEC>;
#[doc = "Status register"]
pub mod sr;
#[doc = "irqen (rw) register accessor: Interrupt request register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`irqen::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`irqen::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@irqen`]
module"]
pub type IRQEN = crate::Reg<irqen::IRQEN_SPEC>;
#[doc = "Interrupt request register"]
pub mod irqen;
#[doc = "no description available"]
pub use self::count::COUNT;
#[doc = r"Cluster"]
#[doc = "no description available"]
pub mod count;
#[doc = "SPDHIS (rw) register accessor: no description available\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spdhis::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spdhis::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spdhis`]
module"]
pub type SPDHIS = crate::Reg<spdhis::SPDHIS_SPEC>;
#[doc = "no description available"]
pub mod spdhis;
