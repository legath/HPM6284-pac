#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    chn: [CHN; 8],
    _reserved1: [u8; 0x40],
    filter_1st_pla_in: [FILTER_1ST_PLA_IN; 8],
    _reserved_2_chn_cfg_active: [u8; 0x40],
}
impl RegisterBlock {
    #[doc = "0x00..0x380 - no description available"]
    #[inline(always)]
    pub const fn chn(&self, n: usize) -> &CHN {
        &self.chn[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x00..0x380 - no description available"]
    #[inline(always)]
    pub fn chn_iter(&self) -> impl Iterator<Item = &CHN> {
        self.chn.iter()
    }
    #[doc = "0x3c0..0x3e0 - no description available"]
    #[inline(always)]
    pub const fn filter_1st_pla_in(&self, n: usize) -> &FILTER_1ST_PLA_IN {
        &self.filter_1st_pla_in[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x3c0..0x3e0 - no description available"]
    #[inline(always)]
    pub fn filter_1st_pla_in_iter(&self) -> impl Iterator<Item = &FILTER_1ST_PLA_IN> {
        self.filter_1st_pla_in.iter()
    }
    #[doc = "0x3c0 - no description available"]
    #[inline(always)]
    pub const fn filter_1st_pla_infrist_filter_pla_in_0(&self) -> &FILTER_1ST_PLA_IN {
        self.filter_1st_pla_in(0)
    }
    #[doc = "0x3c4 - no description available"]
    #[inline(always)]
    pub const fn filter_1st_pla_infrist_filter_pla_in_1(&self) -> &FILTER_1ST_PLA_IN {
        self.filter_1st_pla_in(1)
    }
    #[doc = "0x3c8 - no description available"]
    #[inline(always)]
    pub const fn filter_1st_pla_infrist_filter_pla_in_2(&self) -> &FILTER_1ST_PLA_IN {
        self.filter_1st_pla_in(2)
    }
    #[doc = "0x3cc - no description available"]
    #[inline(always)]
    pub const fn filter_1st_pla_infrist_filter_pla_in_3(&self) -> &FILTER_1ST_PLA_IN {
        self.filter_1st_pla_in(3)
    }
    #[doc = "0x3d0 - no description available"]
    #[inline(always)]
    pub const fn filter_1st_pla_infrist_filter_pla_in_4(&self) -> &FILTER_1ST_PLA_IN {
        self.filter_1st_pla_in(4)
    }
    #[doc = "0x3d4 - no description available"]
    #[inline(always)]
    pub const fn filter_1st_pla_infrist_filter_pla_in_5(&self) -> &FILTER_1ST_PLA_IN {
        self.filter_1st_pla_in(5)
    }
    #[doc = "0x3d8 - no description available"]
    #[inline(always)]
    pub const fn filter_1st_pla_infrist_filter_pla_in_6(&self) -> &FILTER_1ST_PLA_IN {
        self.filter_1st_pla_in(6)
    }
    #[doc = "0x3dc - no description available"]
    #[inline(always)]
    pub const fn filter_1st_pla_infrist_filter_pla_in_7(&self) -> &FILTER_1ST_PLA_IN {
        self.filter_1st_pla_in(7)
    }
    #[doc = "0x3e0..0x404 - no description available"]
    #[inline(always)]
    pub const fn filter_1st_pla_out(&self, n: usize) -> &FILTER_1ST_PLA_OUT {
        #[allow(clippy::no_effect)]
        [(); 9][n];
        unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(992)
                .add(4 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x3e0..0x404 - no description available"]
    #[inline(always)]
    pub fn filter_1st_pla_out_iter(&self) -> impl Iterator<Item = &FILTER_1ST_PLA_OUT> {
        (0..9).map(move |n| unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(992)
                .add(4 * n)
                .cast()
        })
    }
    #[doc = "0x3e0 - no description available"]
    #[inline(always)]
    pub const fn filter_1st_pla_outfrist_filter_pla_in_0(&self) -> &FILTER_1ST_PLA_OUT {
        self.filter_1st_pla_out(0)
    }
    #[doc = "0x3e4 - no description available"]
    #[inline(always)]
    pub const fn filter_1st_pla_outfrist_filter_pla_out_0(&self) -> &FILTER_1ST_PLA_OUT {
        self.filter_1st_pla_out(1)
    }
    #[doc = "0x3e8 - no description available"]
    #[inline(always)]
    pub const fn filter_1st_pla_outfrist_filter_pla_out_1(&self) -> &FILTER_1ST_PLA_OUT {
        self.filter_1st_pla_out(2)
    }
    #[doc = "0x3ec - no description available"]
    #[inline(always)]
    pub const fn filter_1st_pla_outfrist_filter_pla_out_2(&self) -> &FILTER_1ST_PLA_OUT {
        self.filter_1st_pla_out(3)
    }
    #[doc = "0x3f0 - no description available"]
    #[inline(always)]
    pub const fn filter_1st_pla_outfrist_filter_pla_out_3(&self) -> &FILTER_1ST_PLA_OUT {
        self.filter_1st_pla_out(4)
    }
    #[doc = "0x3f4 - no description available"]
    #[inline(always)]
    pub const fn filter_1st_pla_outfrist_filter_pla_out_4(&self) -> &FILTER_1ST_PLA_OUT {
        self.filter_1st_pla_out(5)
    }
    #[doc = "0x3f8 - no description available"]
    #[inline(always)]
    pub const fn filter_1st_pla_outfrist_filter_pla_out_5(&self) -> &FILTER_1ST_PLA_OUT {
        self.filter_1st_pla_out(6)
    }
    #[doc = "0x3fc - no description available"]
    #[inline(always)]
    pub const fn filter_1st_pla_outfrist_filter_pla_out_6(&self) -> &FILTER_1ST_PLA_OUT {
        self.filter_1st_pla_out(7)
    }
    #[doc = "0x400 - no description available"]
    #[inline(always)]
    pub const fn filter_1st_pla_outfrist_filter_pla_out_7(&self) -> &FILTER_1ST_PLA_OUT {
        self.filter_1st_pla_out(8)
    }
    #[doc = "0x400..0x420 - no description available"]
    #[inline(always)]
    pub const fn chn_cfg_active(&self, n: usize) -> &CHN_CFG_ACTIVE {
        #[allow(clippy::no_effect)]
        [(); 8][n];
        unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(1024)
                .add(4 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x400..0x420 - no description available"]
    #[inline(always)]
    pub fn chn_cfg_active_iter(&self) -> impl Iterator<Item = &CHN_CFG_ACTIVE> {
        (0..8).map(move |n| unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(1024)
                .add(4 * n)
                .cast()
        })
    }
    #[doc = "0x400 - no description available"]
    #[inline(always)]
    pub const fn chn_cfg_activecfg_active_chn0(&self) -> &CHN_CFG_ACTIVE {
        self.chn_cfg_active(0)
    }
    #[doc = "0x404 - no description available"]
    #[inline(always)]
    pub const fn chn_cfg_activecfg_active_chn1(&self) -> &CHN_CFG_ACTIVE {
        self.chn_cfg_active(1)
    }
    #[doc = "0x408 - no description available"]
    #[inline(always)]
    pub const fn chn_cfg_activecfg_active_chn2(&self) -> &CHN_CFG_ACTIVE {
        self.chn_cfg_active(2)
    }
    #[doc = "0x40c - no description available"]
    #[inline(always)]
    pub const fn chn_cfg_activecfg_active_chn3(&self) -> &CHN_CFG_ACTIVE {
        self.chn_cfg_active(3)
    }
    #[doc = "0x410 - no description available"]
    #[inline(always)]
    pub const fn chn_cfg_activecfg_active_chn4(&self) -> &CHN_CFG_ACTIVE {
        self.chn_cfg_active(4)
    }
    #[doc = "0x414 - no description available"]
    #[inline(always)]
    pub const fn chn_cfg_activecfg_active_chn5(&self) -> &CHN_CFG_ACTIVE {
        self.chn_cfg_active(5)
    }
    #[doc = "0x418 - no description available"]
    #[inline(always)]
    pub const fn chn_cfg_activecfg_active_chn6(&self) -> &CHN_CFG_ACTIVE {
        self.chn_cfg_active(6)
    }
    #[doc = "0x41c - no description available"]
    #[inline(always)]
    pub const fn chn_cfg_activecfg_active_chn7(&self) -> &CHN_CFG_ACTIVE {
        self.chn_cfg_active(7)
    }
}
#[doc = "no description available"]
pub use self::chn::CHN;
#[doc = r"Cluster"]
#[doc = "no description available"]
pub mod chn;
#[doc = "FILTER_1ST_PLA_IN (rw) register accessor: no description available\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`filter_1st_pla_in::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`filter_1st_pla_in::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@filter_1st_pla_in`]
module"]
pub type FILTER_1ST_PLA_IN = crate::Reg<filter_1st_pla_in::FILTER_1ST_PLA_IN_SPEC>;
#[doc = "no description available"]
pub mod filter_1st_pla_in;
#[doc = "FILTER_1ST_PLA_OUT (rw) register accessor: no description available\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`filter_1st_pla_out::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`filter_1st_pla_out::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@filter_1st_pla_out`]
module"]
pub type FILTER_1ST_PLA_OUT = crate::Reg<filter_1st_pla_out::FILTER_1ST_PLA_OUT_SPEC>;
#[doc = "no description available"]
pub mod filter_1st_pla_out;
#[doc = "CHN_CFG_ACTIVE (rw) register accessor: no description available\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`chn_cfg_active::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`chn_cfg_active::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chn_cfg_active`]
module"]
pub type CHN_CFG_ACTIVE = crate::Reg<chn_cfg_active::CHN_CFG_ACTIVE_SPEC>;
#[doc = "no description available"]
pub mod chn_cfg_active;
