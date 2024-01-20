#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    unlk: UNLK,
    _reserved_1_sta: [u8; 0x04],
    _reserved_2_rld: [u8; 0x04],
    _reserved_3_cmp: [u8; 0x60],
    _reserved4: [u8; 0x0c],
    frcmd: FRCMD,
    shlk: SHLK,
    chcfg: [CHCFG; 24],
    _reserved7: [u8; 0x10],
    gcr: GCR,
    shcr: SHCR,
    _reserved9: [u8; 0x08],
    cappos: [CAPPOS; 24],
    _reserved10: [u8; 0x10],
    cnt: CNT,
    _reserved11: [u8; 0x0c],
    capneg: [CAPNEG; 24],
    _reserved12: [u8; 0x10],
    cntcopy: CNTCOPY,
    _reserved13: [u8; 0x0c],
    pwmcfg: [PWMCFG; 8],
    sr: SR,
    irqen: IRQEN,
    _reserved16: [u8; 0x04],
    dmaen: DMAEN,
    cmpcfg: [CMPCFG; 24],
    _reserved18: [u8; 0x0170],
    anasts: [ANASTS; 8],
    hrpwm_cfg: HRPWM_CFG,
}
impl RegisterBlock {
    #[doc = "0x00 - Shadow registers unlock register"]
    #[inline(always)]
    pub const fn unlk(&self) -> &UNLK {
        &self.unlk
    }
    #[doc = "0x04 - Counter start register"]
    #[inline(always)]
    pub const fn sta_hrpwm(&self) -> &STA_HRPWM {
        unsafe { &*(self as *const Self).cast::<u8>().add(4).cast() }
    }
    #[doc = "0x04 - Counter start register"]
    #[inline(always)]
    pub const fn sta(&self) -> &STA {
        unsafe { &*(self as *const Self).cast::<u8>().add(4).cast() }
    }
    #[doc = "0x08 - Counter reload register"]
    #[inline(always)]
    pub const fn rld_hrpwm(&self) -> &RLD_HRPWM {
        unsafe { &*(self as *const Self).cast::<u8>().add(8).cast() }
    }
    #[doc = "0x08 - Counter reload register"]
    #[inline(always)]
    pub const fn rld(&self) -> &RLD {
        unsafe { &*(self as *const Self).cast::<u8>().add(8).cast() }
    }
    #[doc = "0x0c..0x6c - no description available"]
    #[inline(always)]
    pub const fn cmp_hrpwm(&self, n: usize) -> &CMP_HRPWM {
        #[allow(clippy::no_effect)]
        [(); 24][n];
        unsafe { &*(self as *const Self).cast::<u8>().add(12).add(4 * n).cast() }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x0c..0x6c - no description available"]
    #[inline(always)]
    pub fn cmp_hrpwm_iter(&self) -> impl Iterator<Item = &CMP_HRPWM> {
        (0..24)
            .map(move |n| unsafe { &*(self as *const Self).cast::<u8>().add(12).add(4 * n).cast() })
    }
    #[doc = "0x0c..0x6c - no description available"]
    #[inline(always)]
    pub const fn cmp(&self, n: usize) -> &CMP {
        #[allow(clippy::no_effect)]
        [(); 24][n];
        unsafe { &*(self as *const Self).cast::<u8>().add(12).add(4 * n).cast() }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x0c..0x6c - no description available"]
    #[inline(always)]
    pub fn cmp_iter(&self) -> impl Iterator<Item = &CMP> {
        (0..24)
            .map(move |n| unsafe { &*(self as *const Self).cast::<u8>().add(12).add(4 * n).cast() })
    }
    #[doc = "0x78 - Force output mode register"]
    #[inline(always)]
    pub const fn frcmd(&self) -> &FRCMD {
        &self.frcmd
    }
    #[doc = "0x7c - Shadow registers lock register"]
    #[inline(always)]
    pub const fn shlk(&self) -> &SHLK {
        &self.shlk
    }
    #[doc = "0x80..0xe0 - no description available"]
    #[inline(always)]
    pub const fn chcfg(&self, n: usize) -> &CHCFG {
        &self.chcfg[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x80..0xe0 - no description available"]
    #[inline(always)]
    pub fn chcfg_iter(&self) -> impl Iterator<Item = &CHCFG> {
        self.chcfg.iter()
    }
    #[doc = "0xf0 - Global control register"]
    #[inline(always)]
    pub const fn gcr(&self) -> &GCR {
        &self.gcr
    }
    #[doc = "0xf4 - Shadow register control register"]
    #[inline(always)]
    pub const fn shcr(&self) -> &SHCR {
        &self.shcr
    }
    #[doc = "0x100..0x160 - no description available"]
    #[inline(always)]
    pub const fn cappos(&self, n: usize) -> &CAPPOS {
        &self.cappos[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x100..0x160 - no description available"]
    #[inline(always)]
    pub fn cappos_iter(&self) -> impl Iterator<Item = &CAPPOS> {
        self.cappos.iter()
    }
    #[doc = "0x170 - Counter"]
    #[inline(always)]
    pub const fn cnt(&self) -> &CNT {
        &self.cnt
    }
    #[doc = "0x180..0x1e0 - no description available"]
    #[inline(always)]
    pub const fn capneg(&self, n: usize) -> &CAPNEG {
        &self.capneg[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x180..0x1e0 - no description available"]
    #[inline(always)]
    pub fn capneg_iter(&self) -> impl Iterator<Item = &CAPNEG> {
        self.capneg.iter()
    }
    #[doc = "0x1f0 - Counter copy"]
    #[inline(always)]
    pub const fn cntcopy(&self) -> &CNTCOPY {
        &self.cntcopy
    }
    #[doc = "0x200..0x220 - no description available"]
    #[inline(always)]
    pub const fn pwmcfg(&self, n: usize) -> &PWMCFG {
        &self.pwmcfg[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x200..0x220 - no description available"]
    #[inline(always)]
    pub fn pwmcfg_iter(&self) -> impl Iterator<Item = &PWMCFG> {
        self.pwmcfg.iter()
    }
    #[doc = "0x220 - Status register"]
    #[inline(always)]
    pub const fn sr(&self) -> &SR {
        &self.sr
    }
    #[doc = "0x224 - Interrupt request enable register"]
    #[inline(always)]
    pub const fn irqen(&self) -> &IRQEN {
        &self.irqen
    }
    #[doc = "0x22c - DMA request enable register"]
    #[inline(always)]
    pub const fn dmaen(&self) -> &DMAEN {
        &self.dmaen
    }
    #[doc = "0x230..0x290 - no description available"]
    #[inline(always)]
    pub const fn cmpcfg(&self, n: usize) -> &CMPCFG {
        &self.cmpcfg[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x230..0x290 - no description available"]
    #[inline(always)]
    pub fn cmpcfg_iter(&self) -> impl Iterator<Item = &CMPCFG> {
        self.cmpcfg.iter()
    }
    #[doc = "0x230 - no description available"]
    #[inline(always)]
    pub const fn cmpcfgcmpcfg0(&self) -> &CMPCFG {
        self.cmpcfg(0)
    }
    #[doc = "0x234 - no description available"]
    #[inline(always)]
    pub const fn cmpcfg1(&self) -> &CMPCFG {
        self.cmpcfg(1)
    }
    #[doc = "0x238 - no description available"]
    #[inline(always)]
    pub const fn cmpcfg2(&self) -> &CMPCFG {
        self.cmpcfg(2)
    }
    #[doc = "0x23c - no description available"]
    #[inline(always)]
    pub const fn cmpcfg3(&self) -> &CMPCFG {
        self.cmpcfg(3)
    }
    #[doc = "0x240 - no description available"]
    #[inline(always)]
    pub const fn cmpcfg4(&self) -> &CMPCFG {
        self.cmpcfg(4)
    }
    #[doc = "0x244 - no description available"]
    #[inline(always)]
    pub const fn cmpcfg5(&self) -> &CMPCFG {
        self.cmpcfg(5)
    }
    #[doc = "0x248 - no description available"]
    #[inline(always)]
    pub const fn cmpcfg6(&self) -> &CMPCFG {
        self.cmpcfg(6)
    }
    #[doc = "0x24c - no description available"]
    #[inline(always)]
    pub const fn cmpcfg7(&self) -> &CMPCFG {
        self.cmpcfg(7)
    }
    #[doc = "0x250 - no description available"]
    #[inline(always)]
    pub const fn cmpcfg8(&self) -> &CMPCFG {
        self.cmpcfg(8)
    }
    #[doc = "0x254 - no description available"]
    #[inline(always)]
    pub const fn cmpcfg9(&self) -> &CMPCFG {
        self.cmpcfg(9)
    }
    #[doc = "0x258 - no description available"]
    #[inline(always)]
    pub const fn cmpcfg10(&self) -> &CMPCFG {
        self.cmpcfg(10)
    }
    #[doc = "0x25c - no description available"]
    #[inline(always)]
    pub const fn cmpcfg11(&self) -> &CMPCFG {
        self.cmpcfg(11)
    }
    #[doc = "0x260 - no description available"]
    #[inline(always)]
    pub const fn cmpcfg12(&self) -> &CMPCFG {
        self.cmpcfg(12)
    }
    #[doc = "0x264 - no description available"]
    #[inline(always)]
    pub const fn cmpcfg13(&self) -> &CMPCFG {
        self.cmpcfg(13)
    }
    #[doc = "0x268 - no description available"]
    #[inline(always)]
    pub const fn cmpcfg14(&self) -> &CMPCFG {
        self.cmpcfg(14)
    }
    #[doc = "0x26c - no description available"]
    #[inline(always)]
    pub const fn cmpcfg15(&self) -> &CMPCFG {
        self.cmpcfg(15)
    }
    #[doc = "0x270 - no description available"]
    #[inline(always)]
    pub const fn cmpcfg16(&self) -> &CMPCFG {
        self.cmpcfg(16)
    }
    #[doc = "0x274 - no description available"]
    #[inline(always)]
    pub const fn cmpcfg17(&self) -> &CMPCFG {
        self.cmpcfg(17)
    }
    #[doc = "0x278 - no description available"]
    #[inline(always)]
    pub const fn cmpcfg18(&self) -> &CMPCFG {
        self.cmpcfg(18)
    }
    #[doc = "0x27c - no description available"]
    #[inline(always)]
    pub const fn cmpcfg19(&self) -> &CMPCFG {
        self.cmpcfg(19)
    }
    #[doc = "0x280 - no description available"]
    #[inline(always)]
    pub const fn cmpcfg20(&self) -> &CMPCFG {
        self.cmpcfg(20)
    }
    #[doc = "0x284 - no description available"]
    #[inline(always)]
    pub const fn cmpcfg21(&self) -> &CMPCFG {
        self.cmpcfg(21)
    }
    #[doc = "0x288 - no description available"]
    #[inline(always)]
    pub const fn cmpcfg22(&self) -> &CMPCFG {
        self.cmpcfg(22)
    }
    #[doc = "0x28c - no description available"]
    #[inline(always)]
    pub const fn cmpcfg23(&self) -> &CMPCFG {
        self.cmpcfg(23)
    }
    #[doc = "0x400..0x420 - no description available"]
    #[inline(always)]
    pub const fn anasts(&self, n: usize) -> &ANASTS {
        &self.anasts[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x400..0x420 - no description available"]
    #[inline(always)]
    pub fn anasts_iter(&self) -> impl Iterator<Item = &ANASTS> {
        self.anasts.iter()
    }
    #[doc = "0x420 - hrpwm config register"]
    #[inline(always)]
    pub const fn hrpwm_cfg(&self) -> &HRPWM_CFG {
        &self.hrpwm_cfg
    }
}
#[doc = "unlk (rw) register accessor: Shadow registers unlock register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`unlk::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`unlk::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@unlk`]
module"]
pub type UNLK = crate::Reg<unlk::UNLK_SPEC>;
#[doc = "Shadow registers unlock register"]
pub mod unlk;
#[doc = "sta (rw) register accessor: Counter start register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sta::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sta::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sta`]
module"]
pub type STA = crate::Reg<sta::STA_SPEC>;
#[doc = "Counter start register"]
pub mod sta;
#[doc = "sta_hrpwm (rw) register accessor: Counter start register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sta_hrpwm::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sta_hrpwm::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sta_hrpwm`]
module"]
pub type STA_HRPWM = crate::Reg<sta_hrpwm::STA_HRPWM_SPEC>;
#[doc = "Counter start register"]
pub mod sta_hrpwm;
#[doc = "rld (rw) register accessor: Counter reload register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rld::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rld::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rld`]
module"]
pub type RLD = crate::Reg<rld::RLD_SPEC>;
#[doc = "Counter reload register"]
pub mod rld;
#[doc = "rld_hrpwm (rw) register accessor: Counter reload register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rld_hrpwm::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rld_hrpwm::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rld_hrpwm`]
module"]
pub type RLD_HRPWM = crate::Reg<rld_hrpwm::RLD_HRPWM_SPEC>;
#[doc = "Counter reload register"]
pub mod rld_hrpwm;
#[doc = "CMP (rw) register accessor: no description available\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmp::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmp::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmp`]
module"]
pub type CMP = crate::Reg<cmp::CMP_SPEC>;
#[doc = "no description available"]
pub mod cmp;
#[doc = "CMP_HRPWM (rw) register accessor: no description available\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmp_hrpwm::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmp_hrpwm::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmp_hrpwm`]
module"]
pub type CMP_HRPWM = crate::Reg<cmp_hrpwm::CMP_HRPWM_SPEC>;
#[doc = "no description available"]
pub mod cmp_hrpwm;
#[doc = "frcmd (rw) register accessor: Force output mode register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`frcmd::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`frcmd::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@frcmd`]
module"]
pub type FRCMD = crate::Reg<frcmd::FRCMD_SPEC>;
#[doc = "Force output mode register"]
pub mod frcmd;
#[doc = "shlk (rw) register accessor: Shadow registers lock register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`shlk::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`shlk::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@shlk`]
module"]
pub type SHLK = crate::Reg<shlk::SHLK_SPEC>;
#[doc = "Shadow registers lock register"]
pub mod shlk;
#[doc = "CHCFG (rw) register accessor: no description available\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`chcfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`chcfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chcfg`]
module"]
pub type CHCFG = crate::Reg<chcfg::CHCFG_SPEC>;
#[doc = "no description available"]
pub mod chcfg;
#[doc = "gcr (rw) register accessor: Global control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gcr`]
module"]
pub type GCR = crate::Reg<gcr::GCR_SPEC>;
#[doc = "Global control register"]
pub mod gcr;
#[doc = "shcr (rw) register accessor: Shadow register control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`shcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`shcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@shcr`]
module"]
pub type SHCR = crate::Reg<shcr::SHCR_SPEC>;
#[doc = "Shadow register control register"]
pub mod shcr;
#[doc = "CAPPOS (rw) register accessor: no description available\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cappos::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cappos::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cappos`]
module"]
pub type CAPPOS = crate::Reg<cappos::CAPPOS_SPEC>;
#[doc = "no description available"]
pub mod cappos;
#[doc = "cnt (rw) register accessor: Counter\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cnt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cnt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cnt`]
module"]
pub type CNT = crate::Reg<cnt::CNT_SPEC>;
#[doc = "Counter"]
pub mod cnt;
#[doc = "CAPNEG (rw) register accessor: no description available\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`capneg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`capneg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@capneg`]
module"]
pub type CAPNEG = crate::Reg<capneg::CAPNEG_SPEC>;
#[doc = "no description available"]
pub mod capneg;
#[doc = "cntcopy (rw) register accessor: Counter copy\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cntcopy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cntcopy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cntcopy`]
module"]
pub type CNTCOPY = crate::Reg<cntcopy::CNTCOPY_SPEC>;
#[doc = "Counter copy"]
pub mod cntcopy;
#[doc = "PWMCFG (rw) register accessor: no description available\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pwmcfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pwmcfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwmcfg`]
module"]
pub type PWMCFG = crate::Reg<pwmcfg::PWMCFG_SPEC>;
#[doc = "no description available"]
pub mod pwmcfg;
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
#[doc = "CMPCFG (rw) register accessor: no description available\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmpcfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmpcfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmpcfg`]
module"]
pub type CMPCFG = crate::Reg<cmpcfg::CMPCFG_SPEC>;
#[doc = "no description available"]
pub mod cmpcfg;
#[doc = "ANASTS (rw) register accessor: no description available\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`anasts::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`anasts::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@anasts`]
module"]
pub type ANASTS = crate::Reg<anasts::ANASTS_SPEC>;
#[doc = "no description available"]
pub mod anasts;
#[doc = "hrpwm_cfg (rw) register accessor: hrpwm config register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hrpwm_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hrpwm_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hrpwm_cfg`]
module"]
pub type HRPWM_CFG = crate::Reg<hrpwm_cfg::HRPWM_CFG_SPEC>;
#[doc = "hrpwm config register"]
pub mod hrpwm_cfg;
