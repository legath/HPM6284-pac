#[doc = r"Register block"]
#[repr(C)]
pub struct CH {
    sdfifoctrl: SDFIFOCTRL,
    sdctrlp: SDCTRLP,
    sdctrle: SDCTRLE,
    sdst: SDST,
    sdata: SDATA,
    sdfifo: SDFIFO,
    scamp: SCAMP,
    schtl: SCHTL,
    schtlz: SCHTLZ,
    scllt: SCLLT,
    scctrl: SCCTRL,
    scst: SCST,
}
impl CH {
    #[doc = "0x00 - Data FIFO Path Control Register"]
    #[inline(always)]
    pub const fn sdfifoctrl(&self) -> &SDFIFOCTRL {
        &self.sdfifoctrl
    }
    #[doc = "0x04 - Data Path Control Primary Register"]
    #[inline(always)]
    pub const fn sdctrlp(&self) -> &SDCTRLP {
        &self.sdctrlp
    }
    #[doc = "0x08 - Data Path Control Extra Register"]
    #[inline(always)]
    pub const fn sdctrle(&self) -> &SDCTRLE {
        &self.sdctrle
    }
    #[doc = "0x0c - Data Path Status"]
    #[inline(always)]
    pub const fn sdst(&self) -> &SDST {
        &self.sdst
    }
    #[doc = "0x10 - Data"]
    #[inline(always)]
    pub const fn sdata(&self) -> &SDATA {
        &self.sdata
    }
    #[doc = "0x14 - FIFO Data"]
    #[inline(always)]
    pub const fn sdfifo(&self) -> &SDFIFO {
        &self.sdfifo
    }
    #[doc = "0x18 - instant Amplitude Results"]
    #[inline(always)]
    pub const fn scamp(&self) -> &SCAMP {
        &self.scamp
    }
    #[doc = "0x1c - Amplitude Threshold for High Limit"]
    #[inline(always)]
    pub const fn schtl(&self) -> &SCHTL {
        &self.schtl
    }
    #[doc = "0x20 - Amplitude Threshold for zero crossing"]
    #[inline(always)]
    pub const fn schtlz(&self) -> &SCHTLZ {
        &self.schtlz
    }
    #[doc = "0x24 - Amplitude Threshold for low limit"]
    #[inline(always)]
    pub const fn scllt(&self) -> &SCLLT {
        &self.scllt
    }
    #[doc = "0x28 - Amplitude Path Control"]
    #[inline(always)]
    pub const fn scctrl(&self) -> &SCCTRL {
        &self.scctrl
    }
    #[doc = "0x2c - Amplitude Path Status"]
    #[inline(always)]
    pub const fn scst(&self) -> &SCST {
        &self.scst
    }
}
#[doc = "SDFIFOCTRL (rw) register accessor: Data FIFO Path Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdfifoctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdfifoctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sdfifoctrl`]
module"]
pub type SDFIFOCTRL = crate::Reg<sdfifoctrl::SDFIFOCTRL_SPEC>;
#[doc = "Data FIFO Path Control Register"]
pub mod sdfifoctrl;
#[doc = "SDCTRLP (rw) register accessor: Data Path Control Primary Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdctrlp::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdctrlp::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sdctrlp`]
module"]
pub type SDCTRLP = crate::Reg<sdctrlp::SDCTRLP_SPEC>;
#[doc = "Data Path Control Primary Register"]
pub mod sdctrlp;
#[doc = "SDCTRLE (rw) register accessor: Data Path Control Extra Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdctrle::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdctrle::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sdctrle`]
module"]
pub type SDCTRLE = crate::Reg<sdctrle::SDCTRLE_SPEC>;
#[doc = "Data Path Control Extra Register"]
pub mod sdctrle;
#[doc = "SDST (rw) register accessor: Data Path Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdst::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdst::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sdst`]
module"]
pub type SDST = crate::Reg<sdst::SDST_SPEC>;
#[doc = "Data Path Status"]
pub mod sdst;
#[doc = "SDATA (rw) register accessor: Data\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdata::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdata::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sdata`]
module"]
pub type SDATA = crate::Reg<sdata::SDATA_SPEC>;
#[doc = "Data"]
pub mod sdata;
#[doc = "SDFIFO (rw) register accessor: FIFO Data\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdfifo::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdfifo::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sdfifo`]
module"]
pub type SDFIFO = crate::Reg<sdfifo::SDFIFO_SPEC>;
#[doc = "FIFO Data"]
pub mod sdfifo;
#[doc = "SCAMP (rw) register accessor: instant Amplitude Results\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`scamp::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`scamp::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scamp`]
module"]
pub type SCAMP = crate::Reg<scamp::SCAMP_SPEC>;
#[doc = "instant Amplitude Results"]
pub mod scamp;
#[doc = "SCHTL (rw) register accessor: Amplitude Threshold for High Limit\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`schtl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`schtl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@schtl`]
module"]
pub type SCHTL = crate::Reg<schtl::SCHTL_SPEC>;
#[doc = "Amplitude Threshold for High Limit"]
pub mod schtl;
#[doc = "SCHTLZ (rw) register accessor: Amplitude Threshold for zero crossing\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`schtlz::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`schtlz::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@schtlz`]
module"]
pub type SCHTLZ = crate::Reg<schtlz::SCHTLZ_SPEC>;
#[doc = "Amplitude Threshold for zero crossing"]
pub mod schtlz;
#[doc = "SCLLT (rw) register accessor: Amplitude Threshold for low limit\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`scllt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`scllt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scllt`]
module"]
pub type SCLLT = crate::Reg<scllt::SCLLT_SPEC>;
#[doc = "Amplitude Threshold for low limit"]
pub mod scllt;
#[doc = "SCCTRL (rw) register accessor: Amplitude Path Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`scctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`scctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scctrl`]
module"]
pub type SCCTRL = crate::Reg<scctrl::SCCTRL_SPEC>;
#[doc = "Amplitude Path Control"]
pub mod scctrl;
#[doc = "SCST (rw) register accessor: Amplitude Path Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`scst::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`scst::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scst`]
module"]
pub type SCST = crate::Reg<scst::SCST_SPEC>;
#[doc = "Amplitude Path Status"]
pub mod scst;
