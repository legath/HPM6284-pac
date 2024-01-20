#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    muxcfg: [MUXCFG; 16],
}
impl RegisterBlock {
    #[doc = "0x00..0x40 - no description available"]
    #[inline(always)]
    pub const fn muxcfg(&self, n: usize) -> &MUXCFG {
        &self.muxcfg[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x00..0x40 - no description available"]
    #[inline(always)]
    pub fn muxcfg_iter(&self) -> impl Iterator<Item = &MUXCFG> {
        self.muxcfg.iter()
    }
    #[doc = "0x00 - no description available"]
    #[inline(always)]
    pub const fn muxcfghdma_mux0(&self) -> &MUXCFG {
        self.muxcfg(0)
    }
    #[doc = "0x04 - no description available"]
    #[inline(always)]
    pub const fn muxcfghdma_mux1(&self) -> &MUXCFG {
        self.muxcfg(1)
    }
    #[doc = "0x08 - no description available"]
    #[inline(always)]
    pub const fn muxcfghdma_mux2(&self) -> &MUXCFG {
        self.muxcfg(2)
    }
    #[doc = "0x0c - no description available"]
    #[inline(always)]
    pub const fn muxcfghdma_mux3(&self) -> &MUXCFG {
        self.muxcfg(3)
    }
    #[doc = "0x10 - no description available"]
    #[inline(always)]
    pub const fn muxcfghdma_mux4(&self) -> &MUXCFG {
        self.muxcfg(4)
    }
    #[doc = "0x14 - no description available"]
    #[inline(always)]
    pub const fn muxcfghdma_mux5(&self) -> &MUXCFG {
        self.muxcfg(5)
    }
    #[doc = "0x18 - no description available"]
    #[inline(always)]
    pub const fn muxcfghdma_mux6(&self) -> &MUXCFG {
        self.muxcfg(6)
    }
    #[doc = "0x1c - no description available"]
    #[inline(always)]
    pub const fn muxcfghdma_mux7(&self) -> &MUXCFG {
        self.muxcfg(7)
    }
    #[doc = "0x20 - no description available"]
    #[inline(always)]
    pub const fn muxcfgxdma_mux0(&self) -> &MUXCFG {
        self.muxcfg(8)
    }
    #[doc = "0x24 - no description available"]
    #[inline(always)]
    pub const fn muxcfgxdma_mux1(&self) -> &MUXCFG {
        self.muxcfg(9)
    }
    #[doc = "0x28 - no description available"]
    #[inline(always)]
    pub const fn muxcfgxdma_mux2(&self) -> &MUXCFG {
        self.muxcfg(10)
    }
    #[doc = "0x2c - no description available"]
    #[inline(always)]
    pub const fn muxcfgxdma_mux3(&self) -> &MUXCFG {
        self.muxcfg(11)
    }
    #[doc = "0x30 - no description available"]
    #[inline(always)]
    pub const fn muxcfgxdma_mux4(&self) -> &MUXCFG {
        self.muxcfg(12)
    }
    #[doc = "0x34 - no description available"]
    #[inline(always)]
    pub const fn muxcfgxdma_mux5(&self) -> &MUXCFG {
        self.muxcfg(13)
    }
    #[doc = "0x38 - no description available"]
    #[inline(always)]
    pub const fn muxcfgxdma_mux6(&self) -> &MUXCFG {
        self.muxcfg(14)
    }
    #[doc = "0x3c - no description available"]
    #[inline(always)]
    pub const fn muxcfgxdma_mux7(&self) -> &MUXCFG {
        self.muxcfg(15)
    }
}
#[doc = "MUXCFG (rw) register accessor: no description available\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`muxcfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`muxcfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@muxcfg`]
module"]
pub type MUXCFG = crate::Reg<muxcfg::MUXCFG_SPEC>;
#[doc = "no description available"]
pub mod muxcfg;
