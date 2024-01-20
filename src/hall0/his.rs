#[doc = r"Register block"]
#[repr(C)]
pub struct HIS {
    his0: HIS0,
    his1: HIS1,
}
impl HIS {
    #[doc = "0x00 - history register 0"]
    #[inline(always)]
    pub const fn his0(&self) -> &HIS0 {
        &self.his0
    }
    #[doc = "0x04 - history register 1"]
    #[inline(always)]
    pub const fn his1(&self) -> &HIS1 {
        &self.his1
    }
}
#[doc = "his0 (rw) register accessor: history register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`his0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`his0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@his0`]
module"]
pub type HIS0 = crate::Reg<his0::HIS0_SPEC>;
#[doc = "history register 0"]
pub mod his0;
#[doc = "his1 (rw) register accessor: history register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`his1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`his1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@his1`]
module"]
pub type HIS1 = crate::Reg<his1::HIS1_SPEC>;
#[doc = "history register 1"]
pub mod his1;
