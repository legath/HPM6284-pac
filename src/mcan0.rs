#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x04],
    endn: ENDN,
    _reserved1: [u8; 0x04],
    dbtp: DBTP,
    test: TEST,
    rwd: RWD,
    cccr: CCCR,
    nbtp: NBTP,
    tscc: TSCC,
    tscv: TSCV,
    tocc: TOCC,
    tocv: TOCV,
    _reserved10: [u8; 0x10],
    ecr: ECR,
    psr: PSR,
    tdcr: TDCR,
    _reserved13: [u8; 0x04],
    ir: IR,
    ie: IE,
    ils: ILS,
    ile: ILE,
    _reserved17: [u8; 0x20],
    gfc: GFC,
    sidfc: SIDFC,
    xidfc: XIDFC,
    _reserved20: [u8; 0x04],
    xidam: XIDAM,
    hpms: HPMS,
    ndat1: NDAT1,
    ndat2: NDAT2,
    rxf0c: RXF0C,
    rxf0s: RXF0S,
    rxf0a: RXF0A,
    rxbc: RXBC,
    rxf1c: RXF1C,
    rxf1s: RXF1S,
    rxf1a: RXF1A,
    rxesc: RXESC,
    txbc: TXBC,
    txfqs: TXFQS,
    txesc: TXESC,
    txbrp: TXBRP,
    txbar: TXBAR,
    txbcr: TXBCR,
    txbto: TXBTO,
    txbcf: TXBCF,
    txbtie: TXBTIE,
    txbcie: TXBCIE,
    _reserved42: [u8; 0x08],
    txefc: TXEFC,
    txefs: TXEFS,
    txefa: TXEFA,
    _reserved45: [u8; 0x0104],
    ts_sel: [TS_SEL; 16],
    crel: CREL,
    tscfg: TSCFG,
    tss1: TSS1,
    tss2: TSS2,
    atb: ATB,
    atbh: ATBH,
    _reserved52: [u8; 0x01a8],
    glb_ctl: GLB_CTL,
    glb_status: GLB_STATUS,
    _reserved54: [u8; 0x1bf8],
    message_buff: [MESSAGE_BUFF; 640],
}
impl RegisterBlock {
    #[doc = "0x04 - endian register"]
    #[inline(always)]
    pub const fn endn(&self) -> &ENDN {
        &self.endn
    }
    #[doc = "0x0c - data bit timing and prescaler, writeable when CCCR.CCE and CCCR.INT are set"]
    #[inline(always)]
    pub const fn dbtp(&self) -> &DBTP {
        &self.dbtp
    }
    #[doc = "0x10 - test register"]
    #[inline(always)]
    pub const fn test(&self) -> &TEST {
        &self.test
    }
    #[doc = "0x14 - ram watchdog"]
    #[inline(always)]
    pub const fn rwd(&self) -> &RWD {
        &self.rwd
    }
    #[doc = "0x18 - CC control register"]
    #[inline(always)]
    pub const fn cccr(&self) -> &CCCR {
        &self.cccr
    }
    #[doc = "0x1c - nominal bit timing and prescaler register"]
    #[inline(always)]
    pub const fn nbtp(&self) -> &NBTP {
        &self.nbtp
    }
    #[doc = "0x20 - timestamp counter configuration"]
    #[inline(always)]
    pub const fn tscc(&self) -> &TSCC {
        &self.tscc
    }
    #[doc = "0x24 - timestamp counter value"]
    #[inline(always)]
    pub const fn tscv(&self) -> &TSCV {
        &self.tscv
    }
    #[doc = "0x28 - timeout counter configuration"]
    #[inline(always)]
    pub const fn tocc(&self) -> &TOCC {
        &self.tocc
    }
    #[doc = "0x2c - timeout counter value"]
    #[inline(always)]
    pub const fn tocv(&self) -> &TOCV {
        &self.tocv
    }
    #[doc = "0x40 - error counter register"]
    #[inline(always)]
    pub const fn ecr(&self) -> &ECR {
        &self.ecr
    }
    #[doc = "0x44 - protocol status register"]
    #[inline(always)]
    pub const fn psr(&self) -> &PSR {
        &self.psr
    }
    #[doc = "0x48 - transmitter delay compensation"]
    #[inline(always)]
    pub const fn tdcr(&self) -> &TDCR {
        &self.tdcr
    }
    #[doc = "0x50 - interrupt register"]
    #[inline(always)]
    pub const fn ir(&self) -> &IR {
        &self.ir
    }
    #[doc = "0x54 - interrupt enable"]
    #[inline(always)]
    pub const fn ie(&self) -> &IE {
        &self.ie
    }
    #[doc = "0x58 - interrupt line select"]
    #[inline(always)]
    pub const fn ils(&self) -> &ILS {
        &self.ils
    }
    #[doc = "0x5c - interrupt line enable"]
    #[inline(always)]
    pub const fn ile(&self) -> &ILE {
        &self.ile
    }
    #[doc = "0x80 - global filter configuration"]
    #[inline(always)]
    pub const fn gfc(&self) -> &GFC {
        &self.gfc
    }
    #[doc = "0x84 - standard ID filter configuration"]
    #[inline(always)]
    pub const fn sidfc(&self) -> &SIDFC {
        &self.sidfc
    }
    #[doc = "0x88 - extended ID filter configuration"]
    #[inline(always)]
    pub const fn xidfc(&self) -> &XIDFC {
        &self.xidfc
    }
    #[doc = "0x90 - extended id and mask"]
    #[inline(always)]
    pub const fn xidam(&self) -> &XIDAM {
        &self.xidam
    }
    #[doc = "0x94 - high priority message status"]
    #[inline(always)]
    pub const fn hpms(&self) -> &HPMS {
        &self.hpms
    }
    #[doc = "0x98 - new data1"]
    #[inline(always)]
    pub const fn ndat1(&self) -> &NDAT1 {
        &self.ndat1
    }
    #[doc = "0x9c - new data2"]
    #[inline(always)]
    pub const fn ndat2(&self) -> &NDAT2 {
        &self.ndat2
    }
    #[doc = "0xa0 - rx fifo 0 configuration"]
    #[inline(always)]
    pub const fn rxf0c(&self) -> &RXF0C {
        &self.rxf0c
    }
    #[doc = "0xa4 - rx fifo 0 status"]
    #[inline(always)]
    pub const fn rxf0s(&self) -> &RXF0S {
        &self.rxf0s
    }
    #[doc = "0xa8 - rx fifo0 acknowledge"]
    #[inline(always)]
    pub const fn rxf0a(&self) -> &RXF0A {
        &self.rxf0a
    }
    #[doc = "0xac - rx buffer configuration"]
    #[inline(always)]
    pub const fn rxbc(&self) -> &RXBC {
        &self.rxbc
    }
    #[doc = "0xb0 - rx fifo1 configuration"]
    #[inline(always)]
    pub const fn rxf1c(&self) -> &RXF1C {
        &self.rxf1c
    }
    #[doc = "0xb4 - rx fifo1 status"]
    #[inline(always)]
    pub const fn rxf1s(&self) -> &RXF1S {
        &self.rxf1s
    }
    #[doc = "0xb8 - rx fifo 1 acknowledge"]
    #[inline(always)]
    pub const fn rxf1a(&self) -> &RXF1A {
        &self.rxf1a
    }
    #[doc = "0xbc - rx buffer/fifo element size configuration"]
    #[inline(always)]
    pub const fn rxesc(&self) -> &RXESC {
        &self.rxesc
    }
    #[doc = "0xc0 - tx buffer configuration"]
    #[inline(always)]
    pub const fn txbc(&self) -> &TXBC {
        &self.txbc
    }
    #[doc = "0xc4 - tx fifo/queue status"]
    #[inline(always)]
    pub const fn txfqs(&self) -> &TXFQS {
        &self.txfqs
    }
    #[doc = "0xc8 - tx buffer element size configuration"]
    #[inline(always)]
    pub const fn txesc(&self) -> &TXESC {
        &self.txesc
    }
    #[doc = "0xcc - tx buffer request pending"]
    #[inline(always)]
    pub const fn txbrp(&self) -> &TXBRP {
        &self.txbrp
    }
    #[doc = "0xd0 - tx buffer add request"]
    #[inline(always)]
    pub const fn txbar(&self) -> &TXBAR {
        &self.txbar
    }
    #[doc = "0xd4 - tx buffer cancellation request"]
    #[inline(always)]
    pub const fn txbcr(&self) -> &TXBCR {
        &self.txbcr
    }
    #[doc = "0xd8 - tx buffer transmission occurred"]
    #[inline(always)]
    pub const fn txbto(&self) -> &TXBTO {
        &self.txbto
    }
    #[doc = "0xdc - tx buffer cancellation finished"]
    #[inline(always)]
    pub const fn txbcf(&self) -> &TXBCF {
        &self.txbcf
    }
    #[doc = "0xe0 - tx buffer transmission interrupt enable"]
    #[inline(always)]
    pub const fn txbtie(&self) -> &TXBTIE {
        &self.txbtie
    }
    #[doc = "0xe4 - tx buffer cancellation finished interrupt enable"]
    #[inline(always)]
    pub const fn txbcie(&self) -> &TXBCIE {
        &self.txbcie
    }
    #[doc = "0xf0 - tx event fifo configuration"]
    #[inline(always)]
    pub const fn txefc(&self) -> &TXEFC {
        &self.txefc
    }
    #[doc = "0xf4 - tx event fifo status"]
    #[inline(always)]
    pub const fn txefs(&self) -> &TXEFS {
        &self.txefs
    }
    #[doc = "0xf8 - tx event fifo acknowledge"]
    #[inline(always)]
    pub const fn txefa(&self) -> &TXEFA {
        &self.txefa
    }
    #[doc = "0x200..0x240 - no description available"]
    #[inline(always)]
    pub const fn ts_sel(&self, n: usize) -> &TS_SEL {
        &self.ts_sel[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x200..0x240 - no description available"]
    #[inline(always)]
    pub fn ts_sel_iter(&self) -> impl Iterator<Item = &TS_SEL> {
        self.ts_sel.iter()
    }
    #[doc = "0x200 - no description available"]
    #[inline(always)]
    pub const fn ts_selts_sel0(&self) -> &TS_SEL {
        self.ts_sel(0)
    }
    #[doc = "0x204 - no description available"]
    #[inline(always)]
    pub const fn ts_selts_sel1(&self) -> &TS_SEL {
        self.ts_sel(1)
    }
    #[doc = "0x208 - no description available"]
    #[inline(always)]
    pub const fn ts_selts_sel2(&self) -> &TS_SEL {
        self.ts_sel(2)
    }
    #[doc = "0x20c - no description available"]
    #[inline(always)]
    pub const fn ts_selts_sel3(&self) -> &TS_SEL {
        self.ts_sel(3)
    }
    #[doc = "0x210 - no description available"]
    #[inline(always)]
    pub const fn ts_selts_sel4(&self) -> &TS_SEL {
        self.ts_sel(4)
    }
    #[doc = "0x214 - no description available"]
    #[inline(always)]
    pub const fn ts_selts_sel5(&self) -> &TS_SEL {
        self.ts_sel(5)
    }
    #[doc = "0x218 - no description available"]
    #[inline(always)]
    pub const fn ts_selts_sel6(&self) -> &TS_SEL {
        self.ts_sel(6)
    }
    #[doc = "0x21c - no description available"]
    #[inline(always)]
    pub const fn ts_selts_sel7(&self) -> &TS_SEL {
        self.ts_sel(7)
    }
    #[doc = "0x220 - no description available"]
    #[inline(always)]
    pub const fn ts_selts_sel8(&self) -> &TS_SEL {
        self.ts_sel(8)
    }
    #[doc = "0x224 - no description available"]
    #[inline(always)]
    pub const fn ts_selts_sel9(&self) -> &TS_SEL {
        self.ts_sel(9)
    }
    #[doc = "0x228 - no description available"]
    #[inline(always)]
    pub const fn ts_selts_sel10(&self) -> &TS_SEL {
        self.ts_sel(10)
    }
    #[doc = "0x22c - no description available"]
    #[inline(always)]
    pub const fn ts_selts_sel11(&self) -> &TS_SEL {
        self.ts_sel(11)
    }
    #[doc = "0x230 - no description available"]
    #[inline(always)]
    pub const fn ts_selts_sel12(&self) -> &TS_SEL {
        self.ts_sel(12)
    }
    #[doc = "0x234 - no description available"]
    #[inline(always)]
    pub const fn ts_selts_sel13(&self) -> &TS_SEL {
        self.ts_sel(13)
    }
    #[doc = "0x238 - no description available"]
    #[inline(always)]
    pub const fn ts_selts_sel14(&self) -> &TS_SEL {
        self.ts_sel(14)
    }
    #[doc = "0x23c - no description available"]
    #[inline(always)]
    pub const fn ts_selts_sel15(&self) -> &TS_SEL {
        self.ts_sel(15)
    }
    #[doc = "0x240 - core release register"]
    #[inline(always)]
    pub const fn crel(&self) -> &CREL {
        &self.crel
    }
    #[doc = "0x244 - timestamp configuration"]
    #[inline(always)]
    pub const fn tscfg(&self) -> &TSCFG {
        &self.tscfg
    }
    #[doc = "0x248 - timestamp status1"]
    #[inline(always)]
    pub const fn tss1(&self) -> &TSS1 {
        &self.tss1
    }
    #[doc = "0x24c - timestamp status2"]
    #[inline(always)]
    pub const fn tss2(&self) -> &TSS2 {
        &self.tss2
    }
    #[doc = "0x250 - actual timebase"]
    #[inline(always)]
    pub const fn atb(&self) -> &ATB {
        &self.atb
    }
    #[doc = "0x254 - actual timebase high"]
    #[inline(always)]
    pub const fn atbh(&self) -> &ATBH {
        &self.atbh
    }
    #[doc = "0x400 - global control"]
    #[inline(always)]
    pub const fn glb_ctl(&self) -> &GLB_CTL {
        &self.glb_ctl
    }
    #[doc = "0x404 - global status"]
    #[inline(always)]
    pub const fn glb_status(&self) -> &GLB_STATUS {
        &self.glb_status
    }
    #[doc = "0x2000..0x2a00 - no description available"]
    #[inline(always)]
    pub const fn message_buff(&self, n: usize) -> &MESSAGE_BUFF {
        &self.message_buff[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x2000..0x2a00 - no description available"]
    #[inline(always)]
    pub fn message_buff_iter(&self) -> impl Iterator<Item = &MESSAGE_BUFF> {
        self.message_buff.iter()
    }
}
#[doc = "ENDN (rw) register accessor: endian register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`endn::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`endn::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@endn`]
module"]
pub type ENDN = crate::Reg<endn::ENDN_SPEC>;
#[doc = "endian register"]
pub mod endn;
#[doc = "DBTP (rw) register accessor: data bit timing and prescaler, writeable when CCCR.CCE and CCCR.INT are set\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dbtp::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dbtp::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dbtp`]
module"]
pub type DBTP = crate::Reg<dbtp::DBTP_SPEC>;
#[doc = "data bit timing and prescaler, writeable when CCCR.CCE and CCCR.INT are set"]
pub mod dbtp;
#[doc = "TEST (rw) register accessor: test register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`test::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`test::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@test`]
module"]
pub type TEST = crate::Reg<test::TEST_SPEC>;
#[doc = "test register"]
pub mod test;
#[doc = "RWD (rw) register accessor: ram watchdog\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rwd::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rwd::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rwd`]
module"]
pub type RWD = crate::Reg<rwd::RWD_SPEC>;
#[doc = "ram watchdog"]
pub mod rwd;
#[doc = "CCCR (rw) register accessor: CC control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cccr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cccr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cccr`]
module"]
pub type CCCR = crate::Reg<cccr::CCCR_SPEC>;
#[doc = "CC control register"]
pub mod cccr;
#[doc = "NBTP (rw) register accessor: nominal bit timing and prescaler register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`nbtp::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`nbtp::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nbtp`]
module"]
pub type NBTP = crate::Reg<nbtp::NBTP_SPEC>;
#[doc = "nominal bit timing and prescaler register"]
pub mod nbtp;
#[doc = "TSCC (rw) register accessor: timestamp counter configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tscc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tscc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tscc`]
module"]
pub type TSCC = crate::Reg<tscc::TSCC_SPEC>;
#[doc = "timestamp counter configuration"]
pub mod tscc;
#[doc = "TSCV (rw) register accessor: timestamp counter value\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tscv::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tscv::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tscv`]
module"]
pub type TSCV = crate::Reg<tscv::TSCV_SPEC>;
#[doc = "timestamp counter value"]
pub mod tscv;
#[doc = "TOCC (rw) register accessor: timeout counter configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tocc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tocc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tocc`]
module"]
pub type TOCC = crate::Reg<tocc::TOCC_SPEC>;
#[doc = "timeout counter configuration"]
pub mod tocc;
#[doc = "TOCV (rw) register accessor: timeout counter value\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tocv::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tocv::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tocv`]
module"]
pub type TOCV = crate::Reg<tocv::TOCV_SPEC>;
#[doc = "timeout counter value"]
pub mod tocv;
#[doc = "ECR (rw) register accessor: error counter register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ecr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ecr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ecr`]
module"]
pub type ECR = crate::Reg<ecr::ECR_SPEC>;
#[doc = "error counter register"]
pub mod ecr;
#[doc = "PSR (rw) register accessor: protocol status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`psr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`psr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@psr`]
module"]
pub type PSR = crate::Reg<psr::PSR_SPEC>;
#[doc = "protocol status register"]
pub mod psr;
#[doc = "TDCR (rw) register accessor: transmitter delay compensation\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tdcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tdcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tdcr`]
module"]
pub type TDCR = crate::Reg<tdcr::TDCR_SPEC>;
#[doc = "transmitter delay compensation"]
pub mod tdcr;
#[doc = "IR (rw) register accessor: interrupt register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ir::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ir::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ir`]
module"]
pub type IR = crate::Reg<ir::IR_SPEC>;
#[doc = "interrupt register"]
pub mod ir;
#[doc = "IE (rw) register accessor: interrupt enable\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ie::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ie::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ie`]
module"]
pub type IE = crate::Reg<ie::IE_SPEC>;
#[doc = "interrupt enable"]
pub mod ie;
#[doc = "ILS (rw) register accessor: interrupt line select\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ils::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ils::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ils`]
module"]
pub type ILS = crate::Reg<ils::ILS_SPEC>;
#[doc = "interrupt line select"]
pub mod ils;
#[doc = "ILE (rw) register accessor: interrupt line enable\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ile::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ile::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ile`]
module"]
pub type ILE = crate::Reg<ile::ILE_SPEC>;
#[doc = "interrupt line enable"]
pub mod ile;
#[doc = "GFC (rw) register accessor: global filter configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gfc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gfc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gfc`]
module"]
pub type GFC = crate::Reg<gfc::GFC_SPEC>;
#[doc = "global filter configuration"]
pub mod gfc;
#[doc = "SIDFC (rw) register accessor: standard ID filter configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sidfc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sidfc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sidfc`]
module"]
pub type SIDFC = crate::Reg<sidfc::SIDFC_SPEC>;
#[doc = "standard ID filter configuration"]
pub mod sidfc;
#[doc = "XIDFC (rw) register accessor: extended ID filter configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`xidfc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`xidfc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@xidfc`]
module"]
pub type XIDFC = crate::Reg<xidfc::XIDFC_SPEC>;
#[doc = "extended ID filter configuration"]
pub mod xidfc;
#[doc = "XIDAM (rw) register accessor: extended id and mask\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`xidam::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`xidam::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@xidam`]
module"]
pub type XIDAM = crate::Reg<xidam::XIDAM_SPEC>;
#[doc = "extended id and mask"]
pub mod xidam;
#[doc = "HPMS (rw) register accessor: high priority message status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hpms::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hpms::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hpms`]
module"]
pub type HPMS = crate::Reg<hpms::HPMS_SPEC>;
#[doc = "high priority message status"]
pub mod hpms;
#[doc = "NDAT1 (rw) register accessor: new data1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ndat1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ndat1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ndat1`]
module"]
pub type NDAT1 = crate::Reg<ndat1::NDAT1_SPEC>;
#[doc = "new data1"]
pub mod ndat1;
#[doc = "NDAT2 (rw) register accessor: new data2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ndat2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ndat2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ndat2`]
module"]
pub type NDAT2 = crate::Reg<ndat2::NDAT2_SPEC>;
#[doc = "new data2"]
pub mod ndat2;
#[doc = "RXF0C (rw) register accessor: rx fifo 0 configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxf0c::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rxf0c::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxf0c`]
module"]
pub type RXF0C = crate::Reg<rxf0c::RXF0C_SPEC>;
#[doc = "rx fifo 0 configuration"]
pub mod rxf0c;
#[doc = "RXF0S (rw) register accessor: rx fifo 0 status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxf0s::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rxf0s::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxf0s`]
module"]
pub type RXF0S = crate::Reg<rxf0s::RXF0S_SPEC>;
#[doc = "rx fifo 0 status"]
pub mod rxf0s;
#[doc = "RXF0A (rw) register accessor: rx fifo0 acknowledge\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxf0a::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rxf0a::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxf0a`]
module"]
pub type RXF0A = crate::Reg<rxf0a::RXF0A_SPEC>;
#[doc = "rx fifo0 acknowledge"]
pub mod rxf0a;
#[doc = "RXBC (rw) register accessor: rx buffer configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxbc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rxbc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxbc`]
module"]
pub type RXBC = crate::Reg<rxbc::RXBC_SPEC>;
#[doc = "rx buffer configuration"]
pub mod rxbc;
#[doc = "RXF1C (rw) register accessor: rx fifo1 configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxf1c::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rxf1c::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxf1c`]
module"]
pub type RXF1C = crate::Reg<rxf1c::RXF1C_SPEC>;
#[doc = "rx fifo1 configuration"]
pub mod rxf1c;
#[doc = "RXF1S (rw) register accessor: rx fifo1 status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxf1s::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rxf1s::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxf1s`]
module"]
pub type RXF1S = crate::Reg<rxf1s::RXF1S_SPEC>;
#[doc = "rx fifo1 status"]
pub mod rxf1s;
#[doc = "RXF1A (rw) register accessor: rx fifo 1 acknowledge\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxf1a::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rxf1a::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxf1a`]
module"]
pub type RXF1A = crate::Reg<rxf1a::RXF1A_SPEC>;
#[doc = "rx fifo 1 acknowledge"]
pub mod rxf1a;
#[doc = "RXESC (rw) register accessor: rx buffer/fifo element size configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxesc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rxesc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxesc`]
module"]
pub type RXESC = crate::Reg<rxesc::RXESC_SPEC>;
#[doc = "rx buffer/fifo element size configuration"]
pub mod rxesc;
#[doc = "TXBC (rw) register accessor: tx buffer configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txbc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txbc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txbc`]
module"]
pub type TXBC = crate::Reg<txbc::TXBC_SPEC>;
#[doc = "tx buffer configuration"]
pub mod txbc;
#[doc = "TXFQS (rw) register accessor: tx fifo/queue status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txfqs::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txfqs::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txfqs`]
module"]
pub type TXFQS = crate::Reg<txfqs::TXFQS_SPEC>;
#[doc = "tx fifo/queue status"]
pub mod txfqs;
#[doc = "TXESC (rw) register accessor: tx buffer element size configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txesc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txesc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txesc`]
module"]
pub type TXESC = crate::Reg<txesc::TXESC_SPEC>;
#[doc = "tx buffer element size configuration"]
pub mod txesc;
#[doc = "TXBRP (rw) register accessor: tx buffer request pending\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txbrp::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txbrp::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txbrp`]
module"]
pub type TXBRP = crate::Reg<txbrp::TXBRP_SPEC>;
#[doc = "tx buffer request pending"]
pub mod txbrp;
#[doc = "TXBAR (rw) register accessor: tx buffer add request\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txbar::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txbar::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txbar`]
module"]
pub type TXBAR = crate::Reg<txbar::TXBAR_SPEC>;
#[doc = "tx buffer add request"]
pub mod txbar;
#[doc = "TXBCR (rw) register accessor: tx buffer cancellation request\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txbcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txbcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txbcr`]
module"]
pub type TXBCR = crate::Reg<txbcr::TXBCR_SPEC>;
#[doc = "tx buffer cancellation request"]
pub mod txbcr;
#[doc = "TXBTO (rw) register accessor: tx buffer transmission occurred\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txbto::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txbto::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txbto`]
module"]
pub type TXBTO = crate::Reg<txbto::TXBTO_SPEC>;
#[doc = "tx buffer transmission occurred"]
pub mod txbto;
#[doc = "TXBCF (rw) register accessor: tx buffer cancellation finished\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txbcf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txbcf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txbcf`]
module"]
pub type TXBCF = crate::Reg<txbcf::TXBCF_SPEC>;
#[doc = "tx buffer cancellation finished"]
pub mod txbcf;
#[doc = "TXBTIE (rw) register accessor: tx buffer transmission interrupt enable\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txbtie::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txbtie::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txbtie`]
module"]
pub type TXBTIE = crate::Reg<txbtie::TXBTIE_SPEC>;
#[doc = "tx buffer transmission interrupt enable"]
pub mod txbtie;
#[doc = "TXBCIE (rw) register accessor: tx buffer cancellation finished interrupt enable\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txbcie::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txbcie::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txbcie`]
module"]
pub type TXBCIE = crate::Reg<txbcie::TXBCIE_SPEC>;
#[doc = "tx buffer cancellation finished interrupt enable"]
pub mod txbcie;
#[doc = "TXEFC (rw) register accessor: tx event fifo configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txefc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txefc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txefc`]
module"]
pub type TXEFC = crate::Reg<txefc::TXEFC_SPEC>;
#[doc = "tx event fifo configuration"]
pub mod txefc;
#[doc = "TXEFS (rw) register accessor: tx event fifo status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txefs::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txefs::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txefs`]
module"]
pub type TXEFS = crate::Reg<txefs::TXEFS_SPEC>;
#[doc = "tx event fifo status"]
pub mod txefs;
#[doc = "TXEFA (rw) register accessor: tx event fifo acknowledge\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txefa::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txefa::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txefa`]
module"]
pub type TXEFA = crate::Reg<txefa::TXEFA_SPEC>;
#[doc = "tx event fifo acknowledge"]
pub mod txefa;
#[doc = "TS_SEL (rw) register accessor: no description available\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ts_sel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ts_sel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ts_sel`]
module"]
pub type TS_SEL = crate::Reg<ts_sel::TS_SEL_SPEC>;
#[doc = "no description available"]
pub mod ts_sel;
#[doc = "CREL (rw) register accessor: core release register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`crel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`crel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@crel`]
module"]
pub type CREL = crate::Reg<crel::CREL_SPEC>;
#[doc = "core release register"]
pub mod crel;
#[doc = "TSCFG (rw) register accessor: timestamp configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tscfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tscfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tscfg`]
module"]
pub type TSCFG = crate::Reg<tscfg::TSCFG_SPEC>;
#[doc = "timestamp configuration"]
pub mod tscfg;
#[doc = "TSS1 (rw) register accessor: timestamp status1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tss1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tss1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tss1`]
module"]
pub type TSS1 = crate::Reg<tss1::TSS1_SPEC>;
#[doc = "timestamp status1"]
pub mod tss1;
#[doc = "TSS2 (rw) register accessor: timestamp status2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tss2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tss2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tss2`]
module"]
pub type TSS2 = crate::Reg<tss2::TSS2_SPEC>;
#[doc = "timestamp status2"]
pub mod tss2;
#[doc = "ATB (rw) register accessor: actual timebase\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`atb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`atb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@atb`]
module"]
pub type ATB = crate::Reg<atb::ATB_SPEC>;
#[doc = "actual timebase"]
pub mod atb;
#[doc = "ATBH (rw) register accessor: actual timebase high\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`atbh::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`atbh::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@atbh`]
module"]
pub type ATBH = crate::Reg<atbh::ATBH_SPEC>;
#[doc = "actual timebase high"]
pub mod atbh;
#[doc = "GLB_CTL (rw) register accessor: global control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`glb_ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`glb_ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@glb_ctl`]
module"]
pub type GLB_CTL = crate::Reg<glb_ctl::GLB_CTL_SPEC>;
#[doc = "global control"]
pub mod glb_ctl;
#[doc = "GLB_STATUS (rw) register accessor: global status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`glb_status::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`glb_status::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@glb_status`]
module"]
pub type GLB_STATUS = crate::Reg<glb_status::GLB_STATUS_SPEC>;
#[doc = "global status"]
pub mod glb_status;
#[doc = "MESSAGE_BUFF (rw) register accessor: no description available\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`message_buff::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`message_buff::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@message_buff`]
module"]
pub type MESSAGE_BUFF = crate::Reg<message_buff::MESSAGE_BUFF_SPEC>;
#[doc = "no description available"]
pub mod message_buff;
