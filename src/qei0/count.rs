#[doc = r"Register block"]
#[repr(C)]
pub struct COUNT {
    z: Z,
    ph: PH,
    spd: SPD,
    tmr: TMR,
}
impl COUNT {
    #[doc = "0x00 - Z counter"]
    #[inline(always)]
    pub const fn z(&self) -> &Z {
        &self.z
    }
    #[doc = "0x04 - Phase counter"]
    #[inline(always)]
    pub const fn ph(&self) -> &PH {
        &self.ph
    }
    #[doc = "0x08 - Speed counter"]
    #[inline(always)]
    pub const fn spd(&self) -> &SPD {
        &self.spd
    }
    #[doc = "0x0c - Timer counter"]
    #[inline(always)]
    pub const fn tmr(&self) -> &TMR {
        &self.tmr
    }
}
#[doc = "z (rw) register accessor: Z counter\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`z::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`z::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@z`]
module"]
pub type Z = crate::Reg<z::Z_SPEC>;
#[doc = "Z counter"]
pub mod z;
#[doc = "ph (rw) register accessor: Phase counter\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ph::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ph::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ph`]
module"]
pub type PH = crate::Reg<ph::PH_SPEC>;
#[doc = "Phase counter"]
pub mod ph;
#[doc = "spd (rw) register accessor: Speed counter\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spd::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spd::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spd`]
module"]
pub type SPD = crate::Reg<spd::SPD_SPEC>;
#[doc = "Speed counter"]
pub mod spd;
#[doc = "tmr (rw) register accessor: Timer counter\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tmr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tmr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tmr`]
module"]
pub type TMR = crate::Reg<tmr::TMR_SPEC>;
#[doc = "Timer counter"]
pub mod tmr;
