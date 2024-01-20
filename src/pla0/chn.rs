#[doc = r"Register block"]
#[repr(C)]
pub struct CHN {
    aoi_16to8: [AOI_16TO8; 8],
    aoi_8to7_00_01: AOI_8TO7_00_01,
    aoi_8to7_02_03: AOI_8TO7_02_03,
    aoi_8to7_04_05: AOI_8TO7_04_05,
    aoi_8to7_06: AOI_8TO7_06,
    filter_2nd: [FILTER_2ND; 8],
    filter_3rd: [FILTER_3RD; 7],
    cfg_ff: CFG_FF,
}
impl CHN {
    #[doc = "0x00..0x20 - no description available"]
    #[inline(always)]
    pub const fn aoi_16to8(&self, n: usize) -> &AOI_16TO8 {
        &self.aoi_16to8[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x00..0x20 - no description available"]
    #[inline(always)]
    pub fn aoi_16to8_iter(&self) -> impl Iterator<Item = &AOI_16TO8> {
        self.aoi_16to8.iter()
    }
    #[doc = "0x00 - no description available"]
    #[inline(always)]
    pub const fn aoi_16to8aoi_16to8_00(&self) -> &AOI_16TO8 {
        self.aoi_16to8(0)
    }
    #[doc = "0x04 - no description available"]
    #[inline(always)]
    pub const fn aoi_16to8aoi_16to8_01(&self) -> &AOI_16TO8 {
        self.aoi_16to8(1)
    }
    #[doc = "0x08 - no description available"]
    #[inline(always)]
    pub const fn aoi_16to8aoi_16to8_02(&self) -> &AOI_16TO8 {
        self.aoi_16to8(2)
    }
    #[doc = "0x0c - no description available"]
    #[inline(always)]
    pub const fn aoi_16to8aoi_16to8_03(&self) -> &AOI_16TO8 {
        self.aoi_16to8(3)
    }
    #[doc = "0x10 - no description available"]
    #[inline(always)]
    pub const fn aoi_16to8aoi_16to8_04(&self) -> &AOI_16TO8 {
        self.aoi_16to8(4)
    }
    #[doc = "0x14 - no description available"]
    #[inline(always)]
    pub const fn aoi_16to8aoi_16to8_05(&self) -> &AOI_16TO8 {
        self.aoi_16to8(5)
    }
    #[doc = "0x18 - no description available"]
    #[inline(always)]
    pub const fn aoi_16to8aoi_16to8_06(&self) -> &AOI_16TO8 {
        self.aoi_16to8(6)
    }
    #[doc = "0x1c - no description available"]
    #[inline(always)]
    pub const fn aoi_16to8aoi_16to8_07(&self) -> &AOI_16TO8 {
        self.aoi_16to8(7)
    }
    #[doc = "0x20 - CHN&amp;index0 AOI_16to8_00_01 OR logic cfg"]
    #[inline(always)]
    pub const fn aoi_8to7_00_01(&self) -> &AOI_8TO7_00_01 {
        &self.aoi_8to7_00_01
    }
    #[doc = "0x24 - CHN&amp;index0 AOI_16to8_02_03 OR logic cfg"]
    #[inline(always)]
    pub const fn aoi_8to7_02_03(&self) -> &AOI_8TO7_02_03 {
        &self.aoi_8to7_02_03
    }
    #[doc = "0x28 - CHN&amp;index0 AOI_16to8_04_05 OR logic cfg"]
    #[inline(always)]
    pub const fn aoi_8to7_04_05(&self) -> &AOI_8TO7_04_05 {
        &self.aoi_8to7_04_05
    }
    #[doc = "0x2c - CHN&amp;index0 AOI_16to8_06 OR logic cfg"]
    #[inline(always)]
    pub const fn aoi_8to7_06(&self) -> &AOI_8TO7_06 {
        &self.aoi_8to7_06
    }
    #[doc = "0x30..0x50 - no description available"]
    #[inline(always)]
    pub const fn filter_2nd(&self, n: usize) -> &FILTER_2ND {
        &self.filter_2nd[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x30..0x50 - no description available"]
    #[inline(always)]
    pub fn filter_2nd_iter(&self) -> impl Iterator<Item = &FILTER_2ND> {
        self.filter_2nd.iter()
    }
    #[doc = "0x30 - no description available"]
    #[inline(always)]
    pub const fn filter_2ndsecond_filter_0(&self) -> &FILTER_2ND {
        self.filter_2nd(0)
    }
    #[doc = "0x34 - no description available"]
    #[inline(always)]
    pub const fn filter_2ndsecond_filter_1(&self) -> &FILTER_2ND {
        self.filter_2nd(1)
    }
    #[doc = "0x38 - no description available"]
    #[inline(always)]
    pub const fn filter_2ndsecond_filter_2(&self) -> &FILTER_2ND {
        self.filter_2nd(2)
    }
    #[doc = "0x3c - no description available"]
    #[inline(always)]
    pub const fn filter_2ndsecond_filter_3(&self) -> &FILTER_2ND {
        self.filter_2nd(3)
    }
    #[doc = "0x40 - no description available"]
    #[inline(always)]
    pub const fn filter_2ndsecond_filter_4(&self) -> &FILTER_2ND {
        self.filter_2nd(4)
    }
    #[doc = "0x44 - no description available"]
    #[inline(always)]
    pub const fn filter_2ndsecond_filter_5(&self) -> &FILTER_2ND {
        self.filter_2nd(5)
    }
    #[doc = "0x48 - no description available"]
    #[inline(always)]
    pub const fn filter_2ndsecond_filter_6(&self) -> &FILTER_2ND {
        self.filter_2nd(6)
    }
    #[doc = "0x4c - no description available"]
    #[inline(always)]
    pub const fn filter_2ndsecond_filter_7(&self) -> &FILTER_2ND {
        self.filter_2nd(7)
    }
    #[doc = "0x50..0x6c - no description available"]
    #[inline(always)]
    pub const fn filter_3rd(&self, n: usize) -> &FILTER_3RD {
        &self.filter_3rd[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x50..0x6c - no description available"]
    #[inline(always)]
    pub fn filter_3rd_iter(&self) -> impl Iterator<Item = &FILTER_3RD> {
        self.filter_3rd.iter()
    }
    #[doc = "0x50 - no description available"]
    #[inline(always)]
    pub const fn filter_3rdthird_filter_0(&self) -> &FILTER_3RD {
        self.filter_3rd(0)
    }
    #[doc = "0x54 - no description available"]
    #[inline(always)]
    pub const fn filter_3rdthird_filter_1(&self) -> &FILTER_3RD {
        self.filter_3rd(1)
    }
    #[doc = "0x58 - no description available"]
    #[inline(always)]
    pub const fn filter_3rdthird_filter_2(&self) -> &FILTER_3RD {
        self.filter_3rd(2)
    }
    #[doc = "0x5c - no description available"]
    #[inline(always)]
    pub const fn filter_3rdthird_filter_3(&self) -> &FILTER_3RD {
        self.filter_3rd(3)
    }
    #[doc = "0x60 - no description available"]
    #[inline(always)]
    pub const fn filter_3rdthird_filter_4(&self) -> &FILTER_3RD {
        self.filter_3rd(4)
    }
    #[doc = "0x64 - no description available"]
    #[inline(always)]
    pub const fn filter_3rdthird_filter_5(&self) -> &FILTER_3RD {
        self.filter_3rd(5)
    }
    #[doc = "0x68 - no description available"]
    #[inline(always)]
    pub const fn filter_3rdthird_filter_6(&self) -> &FILTER_3RD {
        self.filter_3rd(6)
    }
    #[doc = "0x6c - CHN&amp;index0 cfg ff"]
    #[inline(always)]
    pub const fn cfg_ff(&self) -> &CFG_FF {
        &self.cfg_ff
    }
}
#[doc = "AOI_16TO8 (rw) register accessor: no description available\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aoi_16to8::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aoi_16to8::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aoi_16to8`]
module"]
pub type AOI_16TO8 = crate::Reg<aoi_16to8::AOI_16TO8_SPEC>;
#[doc = "no description available"]
pub mod aoi_16to8;
#[doc = "AOI_8to7_00_01 (rw) register accessor: CHN&amp;index0 AOI_16to8_00_01 OR logic cfg\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aoi_8to7_00_01::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aoi_8to7_00_01::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aoi_8to7_00_01`]
module"]
pub type AOI_8TO7_00_01 = crate::Reg<aoi_8to7_00_01::AOI_8TO7_00_01_SPEC>;
#[doc = "CHN&amp;index0 AOI_16to8_00_01 OR logic cfg"]
pub mod aoi_8to7_00_01;
#[doc = "AOI_8to7_02_03 (rw) register accessor: CHN&amp;index0 AOI_16to8_02_03 OR logic cfg\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aoi_8to7_02_03::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aoi_8to7_02_03::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aoi_8to7_02_03`]
module"]
pub type AOI_8TO7_02_03 = crate::Reg<aoi_8to7_02_03::AOI_8TO7_02_03_SPEC>;
#[doc = "CHN&amp;index0 AOI_16to8_02_03 OR logic cfg"]
pub mod aoi_8to7_02_03;
#[doc = "AOI_8to7_04_05 (rw) register accessor: CHN&amp;index0 AOI_16to8_04_05 OR logic cfg\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aoi_8to7_04_05::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aoi_8to7_04_05::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aoi_8to7_04_05`]
module"]
pub type AOI_8TO7_04_05 = crate::Reg<aoi_8to7_04_05::AOI_8TO7_04_05_SPEC>;
#[doc = "CHN&amp;index0 AOI_16to8_04_05 OR logic cfg"]
pub mod aoi_8to7_04_05;
#[doc = "AOI_8to7_06 (rw) register accessor: CHN&amp;index0 AOI_16to8_06 OR logic cfg\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aoi_8to7_06::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aoi_8to7_06::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aoi_8to7_06`]
module"]
pub type AOI_8TO7_06 = crate::Reg<aoi_8to7_06::AOI_8TO7_06_SPEC>;
#[doc = "CHN&amp;index0 AOI_16to8_06 OR logic cfg"]
pub mod aoi_8to7_06;
#[doc = "FILTER_2ND (rw) register accessor: no description available\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`filter_2nd::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`filter_2nd::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@filter_2nd`]
module"]
pub type FILTER_2ND = crate::Reg<filter_2nd::FILTER_2ND_SPEC>;
#[doc = "no description available"]
pub mod filter_2nd;
#[doc = "FILTER_3RD (rw) register accessor: no description available\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`filter_3rd::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`filter_3rd::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@filter_3rd`]
module"]
pub type FILTER_3RD = crate::Reg<filter_3rd::FILTER_3RD_SPEC>;
#[doc = "no description available"]
pub mod filter_3rd;
#[doc = "CFG_FF (rw) register accessor: CHN&amp;index0 cfg ff\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_ff::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_ff::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_ff`]
module"]
pub type CFG_FF = crate::Reg<cfg_ff::CFG_FF_SPEC>;
#[doc = "CHN&amp;index0 cfg ff"]
pub mod cfg_ff;
