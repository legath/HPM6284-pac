#[doc = r"Register block"]
#[repr(C)]
pub struct TARGETINT {
    inten: [INTEN; 4],
}
impl TARGETINT {
    #[doc = "0x00..0x10 - no description available"]
    #[inline(always)]
    pub const fn inten(&self, n: usize) -> &INTEN {
        &self.inten[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x00..0x10 - no description available"]
    #[inline(always)]
    pub fn inten_iter(&self) -> impl Iterator<Item = &INTEN> {
        self.inten.iter()
    }
    #[doc = "0x00 - no description available"]
    #[inline(always)]
    pub const fn inteninten0(&self) -> &INTEN {
        self.inten(0)
    }
    #[doc = "0x04 - no description available"]
    #[inline(always)]
    pub const fn inteninten1(&self) -> &INTEN {
        self.inten(1)
    }
    #[doc = "0x08 - no description available"]
    #[inline(always)]
    pub const fn inteninten2(&self) -> &INTEN {
        self.inten(2)
    }
    #[doc = "0x0c - no description available"]
    #[inline(always)]
    pub const fn inteninten3(&self) -> &INTEN {
        self.inten(3)
    }
}
#[doc = "INTEN (rw) register accessor: no description available\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`inten::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`inten::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@inten`]
module"]
pub type INTEN = crate::Reg<inten::INTEN_SPEC>;
#[doc = "no description available"]
pub mod inten;
