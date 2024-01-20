#[doc = r"Register block"]
#[repr(C)]
pub struct CHN {
    pre_set: PRE_SET,
    clr: CLR,
    poly: POLY,
    init_data: INIT_DATA,
    xorout: XOROUT,
    misc_setting: MISC_SETTING,
    data: DATA,
    result: RESULT,
}
impl CHN {
    #[doc = "0x00 - &amp;index0 pre set for crc setting"]
    #[inline(always)]
    pub const fn pre_set(&self) -> &PRE_SET {
        &self.pre_set
    }
    #[doc = "0x04 - chn&amp;index0 clear crc result and setting"]
    #[inline(always)]
    pub const fn clr(&self) -> &CLR {
        &self.clr
    }
    #[doc = "0x08 - chn&amp;index0 poly"]
    #[inline(always)]
    pub const fn poly(&self) -> &POLY {
        &self.poly
    }
    #[doc = "0x0c - chn&amp;index0 init_data"]
    #[inline(always)]
    pub const fn init_data(&self) -> &INIT_DATA {
        &self.init_data
    }
    #[doc = "0x10 - chn&amp;index0 xorout"]
    #[inline(always)]
    pub const fn xorout(&self) -> &XOROUT {
        &self.xorout
    }
    #[doc = "0x14 - chn&amp;index0 misc_setting"]
    #[inline(always)]
    pub const fn misc_setting(&self) -> &MISC_SETTING {
        &self.misc_setting
    }
    #[doc = "0x18 - chn&amp;index0 data"]
    #[inline(always)]
    pub const fn data(&self) -> &DATA {
        &self.data
    }
    #[doc = "0x1c - chn&amp;index0 result"]
    #[inline(always)]
    pub const fn result(&self) -> &RESULT {
        &self.result
    }
}
#[doc = "pre_set (rw) register accessor: &amp;index0 pre set for crc setting\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pre_set::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pre_set::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pre_set`]
module"]
pub type PRE_SET = crate::Reg<pre_set::PRE_SET_SPEC>;
#[doc = "&amp;index0 pre set for crc setting"]
pub mod pre_set;
#[doc = "clr (rw) register accessor: chn&amp;index0 clear crc result and setting\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clr`]
module"]
pub type CLR = crate::Reg<clr::CLR_SPEC>;
#[doc = "chn&amp;index0 clear crc result and setting"]
pub mod clr;
#[doc = "poly (rw) register accessor: chn&amp;index0 poly\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`poly::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`poly::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@poly`]
module"]
pub type POLY = crate::Reg<poly::POLY_SPEC>;
#[doc = "chn&amp;index0 poly"]
pub mod poly;
#[doc = "init_data (rw) register accessor: chn&amp;index0 init_data\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`init_data::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`init_data::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@init_data`]
module"]
pub type INIT_DATA = crate::Reg<init_data::INIT_DATA_SPEC>;
#[doc = "chn&amp;index0 init_data"]
pub mod init_data;
#[doc = "xorout (rw) register accessor: chn&amp;index0 xorout\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`xorout::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`xorout::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@xorout`]
module"]
pub type XOROUT = crate::Reg<xorout::XOROUT_SPEC>;
#[doc = "chn&amp;index0 xorout"]
pub mod xorout;
#[doc = "misc_setting (rw) register accessor: chn&amp;index0 misc_setting\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`misc_setting::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`misc_setting::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@misc_setting`]
module"]
pub type MISC_SETTING = crate::Reg<misc_setting::MISC_SETTING_SPEC>;
#[doc = "chn&amp;index0 misc_setting"]
pub mod misc_setting;
#[doc = "data (rw) register accessor: chn&amp;index0 data\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`data::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`data::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@data`]
module"]
pub type DATA = crate::Reg<data::DATA_SPEC>;
#[doc = "chn&amp;index0 data"]
pub mod data;
#[doc = "result (rw) register accessor: chn&amp;index0 result\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`result::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`result::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@result`]
module"]
pub type RESULT = crate::Reg<result::RESULT_SPEC>;
#[doc = "chn&amp;index0 result"]
pub mod result;
