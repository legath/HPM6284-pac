#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    xtal: XTAL,
    _reserved1: [u8; 0x7c],
    pll: (),
}
impl RegisterBlock {
    #[doc = "0x00 - OSC configuration"]
    #[inline(always)]
    pub const fn xtal(&self) -> &XTAL {
        &self.xtal
    }
    #[doc = "0x80..0x170 - no description available"]
    #[inline(always)]
    pub const fn pll(&self, n: usize) -> &PLL {
        #[allow(clippy::no_effect)]
        [(); 3][n];
        unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(128)
                .add(128 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x80..0x170 - no description available"]
    #[inline(always)]
    pub fn pll_iter(&self) -> impl Iterator<Item = &PLL> {
        (0..3).map(move |n| unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(128)
                .add(128 * n)
                .cast()
        })
    }
    #[doc = "0x80..0xd0 - no description available"]
    #[inline(always)]
    pub const fn pllpll0(&self) -> &PLL {
        self.pll(0)
    }
    #[doc = "0x100..0x150 - no description available"]
    #[inline(always)]
    pub const fn pllpll1(&self) -> &PLL {
        self.pll(1)
    }
    #[doc = "0x180..0x1d0 - no description available"]
    #[inline(always)]
    pub const fn pllpll2(&self) -> &PLL {
        self.pll(2)
    }
}
#[doc = "XTAL (rw) register accessor: OSC configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`xtal::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`xtal::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@xtal`]
module"]
pub type XTAL = crate::Reg<xtal::XTAL_SPEC>;
#[doc = "OSC configuration"]
pub mod xtal;
#[doc = "no description available"]
pub use self::pll::PLL;
#[doc = r"Cluster"]
#[doc = "no description available"]
pub mod pll;
