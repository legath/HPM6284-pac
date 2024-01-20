#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    ptpc: (),
    _reserved1: [u8; 0x2000],
    time_sel: TIME_SEL,
    int_sts: INT_STS,
    int_en: INT_EN,
}
impl RegisterBlock {
    #[doc = "0x00..0x70 - no description available"]
    #[inline(always)]
    pub const fn ptpc(&self, n: usize) -> &PTPC {
        #[allow(clippy::no_effect)]
        [(); 2][n];
        unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(0)
                .add(4096 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x00..0x70 - no description available"]
    #[inline(always)]
    pub fn ptpc_iter(&self) -> impl Iterator<Item = &PTPC> {
        (0..2).map(move |n| unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(0)
                .add(4096 * n)
                .cast()
        })
    }
    #[doc = "0x2000 - No description avaiable"]
    #[inline(always)]
    pub const fn time_sel(&self) -> &TIME_SEL {
        &self.time_sel
    }
    #[doc = "0x2004 - No description avaiable"]
    #[inline(always)]
    pub const fn int_sts(&self) -> &INT_STS {
        &self.int_sts
    }
    #[doc = "0x2008 - No description avaiable"]
    #[inline(always)]
    pub const fn int_en(&self) -> &INT_EN {
        &self.int_en
    }
}
#[doc = "no description available"]
pub use self::ptpc::PTPC;
#[doc = r"Cluster"]
#[doc = "no description available"]
pub mod ptpc;
#[doc = "time_sel (rw) register accessor: No description avaiable\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`time_sel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`time_sel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@time_sel`]
module"]
pub type TIME_SEL = crate::Reg<time_sel::TIME_SEL_SPEC>;
#[doc = "No description avaiable"]
pub mod time_sel;
#[doc = "int_sts (rw) register accessor: No description avaiable\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_sts::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_sts::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_sts`]
module"]
pub type INT_STS = crate::Reg<int_sts::INT_STS_SPEC>;
#[doc = "No description avaiable"]
pub mod int_sts;
#[doc = "int_en (rw) register accessor: No description avaiable\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_en::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_en::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_en`]
module"]
pub type INT_EN = crate::Reg<int_en::INT_EN_SPEC>;
#[doc = "No description avaiable"]
pub mod int_en;
