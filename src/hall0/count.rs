#[doc = r"Register block"]
#[repr(C)]
pub struct COUNT {
    w: W,
    v: V,
    u: U,
    tmr: TMR,
}
impl COUNT {
    #[doc = "0x00 - W counter"]
    #[inline(always)]
    pub const fn w(&self) -> &W {
        &self.w
    }
    #[doc = "0x04 - V counter"]
    #[inline(always)]
    pub const fn v(&self) -> &V {
        &self.v
    }
    #[doc = "0x08 - U counter"]
    #[inline(always)]
    pub const fn u(&self) -> &U {
        &self.u
    }
    #[doc = "0x0c - Timer counter"]
    #[inline(always)]
    pub const fn tmr(&self) -> &TMR {
        &self.tmr
    }
}
#[doc = "w (rw) register accessor: W counter\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`w::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`w::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@w`]
module"]
pub type W = crate::Reg<w::W_SPEC>;
#[doc = "W counter"]
pub mod w;
#[doc = "v (rw) register accessor: V counter\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`v::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`v::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@v`]
module"]
pub type V = crate::Reg<v::V_SPEC>;
#[doc = "V counter"]
pub mod v;
#[doc = "u (rw) register accessor: U counter\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`u::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`u::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@u`]
module"]
pub type U = crate::Reg<u::U_SPEC>;
#[doc = "U counter"]
pub mod u;
#[doc = "tmr (rw) register accessor: Timer counter\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tmr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tmr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tmr`]
module"]
pub type TMR = crate::Reg<tmr::TMR_SPEC>;
#[doc = "Timer counter"]
pub mod tmr;
