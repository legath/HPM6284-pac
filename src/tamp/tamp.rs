#[doc = r"Register block"]
#[repr(C)]
pub struct TAMP {
    control: CONTROL,
    poly: POLY,
    lfsr: LFSR,
}
impl TAMP {
    #[doc = "0x00 - Tamper n control"]
    #[inline(always)]
    pub const fn control(&self) -> &CONTROL {
        &self.control
    }
    #[doc = "0x04 - Tamper n Polynomial of LFSR"]
    #[inline(always)]
    pub const fn poly(&self) -> &POLY {
        &self.poly
    }
    #[doc = "0x08 - Tamper n LFSR shift register"]
    #[inline(always)]
    pub const fn lfsr(&self) -> &LFSR {
        &self.lfsr
    }
}
#[doc = "CONTROL (rw) register accessor: Tamper n control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`control::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`control::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@control`]
module"]
pub type CONTROL = crate::Reg<control::CONTROL_SPEC>;
#[doc = "Tamper n control"]
pub mod control;
#[doc = "POLY (rw) register accessor: Tamper n Polynomial of LFSR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`poly::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`poly::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@poly`]
module"]
pub type POLY = crate::Reg<poly::POLY_SPEC>;
#[doc = "Tamper n Polynomial of LFSR"]
pub mod poly;
#[doc = "LFSR (rw) register accessor: Tamper n LFSR shift register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lfsr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lfsr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lfsr`]
module"]
pub type LFSR = crate::Reg<lfsr::LFSR_SPEC>;
#[doc = "Tamper n LFSR shift register"]
pub mod lfsr;
