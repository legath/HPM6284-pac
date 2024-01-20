#[doc = r"Register block"]
#[repr(C)]
pub struct CHANNEL {
    cr: CR,
    cmp: [CMP; 2],
    rld: RLD,
    cntuptval: CNTUPTVAL,
    _reserved4: [u8; 0x0c],
    cappos: CAPPOS,
    capneg: CAPNEG,
    capprd: CAPPRD,
    capdty: CAPDTY,
    cnt: CNT,
}
impl CHANNEL {
    #[doc = "0x00 - Control Register"]
    #[inline(always)]
    pub const fn cr(&self) -> &CR {
        &self.cr
    }
    #[doc = "0x04..0x0c - no description available"]
    #[inline(always)]
    pub const fn cmp(&self, n: usize) -> &CMP {
        &self.cmp[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x04..0x0c - no description available"]
    #[inline(always)]
    pub fn cmp_iter(&self) -> impl Iterator<Item = &CMP> {
        self.cmp.iter()
    }
    #[doc = "0x04 - no description available"]
    #[inline(always)]
    pub const fn cmpcmp0(&self) -> &CMP {
        self.cmp(0)
    }
    #[doc = "0x08 - no description available"]
    #[inline(always)]
    pub const fn cmpcmp1(&self) -> &CMP {
        self.cmp(1)
    }
    #[doc = "0x0c - Reload register"]
    #[inline(always)]
    pub const fn rld(&self) -> &RLD {
        &self.rld
    }
    #[doc = "0x10 - Counter update value register"]
    #[inline(always)]
    pub const fn cntuptval(&self) -> &CNTUPTVAL {
        &self.cntuptval
    }
    #[doc = "0x20 - Capture rising edge register"]
    #[inline(always)]
    pub const fn cappos(&self) -> &CAPPOS {
        &self.cappos
    }
    #[doc = "0x24 - Capture falling edge register"]
    #[inline(always)]
    pub const fn capneg(&self) -> &CAPNEG {
        &self.capneg
    }
    #[doc = "0x28 - PWM period measure register"]
    #[inline(always)]
    pub const fn capprd(&self) -> &CAPPRD {
        &self.capprd
    }
    #[doc = "0x2c - PWM duty cycle measure register"]
    #[inline(always)]
    pub const fn capdty(&self) -> &CAPDTY {
        &self.capdty
    }
    #[doc = "0x30 - Counter"]
    #[inline(always)]
    pub const fn cnt(&self) -> &CNT {
        &self.cnt
    }
}
#[doc = "CR (rw) register accessor: Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr`]
module"]
pub type CR = crate::Reg<cr::CR_SPEC>;
#[doc = "Control Register"]
pub mod cr;
#[doc = "CMP (rw) register accessor: no description available\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmp::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmp::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmp`]
module"]
pub type CMP = crate::Reg<cmp::CMP_SPEC>;
#[doc = "no description available"]
pub mod cmp;
#[doc = "RLD (rw) register accessor: Reload register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rld::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rld::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rld`]
module"]
pub type RLD = crate::Reg<rld::RLD_SPEC>;
#[doc = "Reload register"]
pub mod rld;
#[doc = "CNTUPTVAL (rw) register accessor: Counter update value register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cntuptval::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cntuptval::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cntuptval`]
module"]
pub type CNTUPTVAL = crate::Reg<cntuptval::CNTUPTVAL_SPEC>;
#[doc = "Counter update value register"]
pub mod cntuptval;
#[doc = "CAPPOS (rw) register accessor: Capture rising edge register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cappos::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cappos::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cappos`]
module"]
pub type CAPPOS = crate::Reg<cappos::CAPPOS_SPEC>;
#[doc = "Capture rising edge register"]
pub mod cappos;
#[doc = "CAPNEG (rw) register accessor: Capture falling edge register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`capneg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`capneg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@capneg`]
module"]
pub type CAPNEG = crate::Reg<capneg::CAPNEG_SPEC>;
#[doc = "Capture falling edge register"]
pub mod capneg;
#[doc = "CAPPRD (rw) register accessor: PWM period measure register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`capprd::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`capprd::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@capprd`]
module"]
pub type CAPPRD = crate::Reg<capprd::CAPPRD_SPEC>;
#[doc = "PWM period measure register"]
pub mod capprd;
#[doc = "CAPDTY (rw) register accessor: PWM duty cycle measure register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`capdty::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`capdty::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@capdty`]
module"]
pub type CAPDTY = crate::Reg<capdty::CAPDTY_SPEC>;
#[doc = "PWM duty cycle measure register"]
pub mod capdty;
#[doc = "CNT (rw) register accessor: Counter\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cnt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cnt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cnt`]
module"]
pub type CNT = crate::Reg<cnt::CNT_SPEC>;
#[doc = "Counter"]
pub mod cnt;
