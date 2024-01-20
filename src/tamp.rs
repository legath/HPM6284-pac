#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    tamp: (),
    _reserved1: [u8; 0x80],
    tamp_flag: TAMP_FLAG,
    irq_en: IRQ_EN,
}
impl RegisterBlock {
    #[doc = "0x00..0x30 - no description available"]
    #[inline(always)]
    pub const fn tamp(&self, n: usize) -> &TAMP {
        #[allow(clippy::no_effect)]
        [(); 4][n];
        unsafe { &*(self as *const Self).cast::<u8>().add(0).add(16 * n).cast() }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x00..0x30 - no description available"]
    #[inline(always)]
    pub fn tamp_iter(&self) -> impl Iterator<Item = &TAMP> {
        (0..4)
            .map(move |n| unsafe { &*(self as *const Self).cast::<u8>().add(0).add(16 * n).cast() })
    }
    #[doc = "0x00..0x0c - no description available"]
    #[inline(always)]
    pub const fn tamptamp0(&self) -> &TAMP {
        self.tamp(0)
    }
    #[doc = "0x10..0x1c - no description available"]
    #[inline(always)]
    pub const fn tamptamp1(&self) -> &TAMP {
        self.tamp(1)
    }
    #[doc = "0x20..0x2c - no description available"]
    #[inline(always)]
    pub const fn tamptamp2(&self) -> &TAMP {
        self.tamp(2)
    }
    #[doc = "0x30..0x3c - no description available"]
    #[inline(always)]
    pub const fn tamptamp3(&self) -> &TAMP {
        self.tamp(3)
    }
    #[doc = "0x80 - Tamper flag"]
    #[inline(always)]
    pub const fn tamp_flag(&self) -> &TAMP_FLAG {
        &self.tamp_flag
    }
    #[doc = "0x84 - Tamper interrupt enable"]
    #[inline(always)]
    pub const fn irq_en(&self) -> &IRQ_EN {
        &self.irq_en
    }
}
#[doc = "no description available"]
pub use self::tamp::TAMP;
#[doc = r"Cluster"]
#[doc = "no description available"]
pub mod tamp;
#[doc = "TAMP_FLAG (rw) register accessor: Tamper flag\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tamp_flag::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tamp_flag::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tamp_flag`]
module"]
pub type TAMP_FLAG = crate::Reg<tamp_flag::TAMP_FLAG_SPEC>;
#[doc = "Tamper flag"]
pub mod tamp_flag;
#[doc = "IRQ_EN (rw) register accessor: Tamper interrupt enable\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`irq_en::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`irq_en::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@irq_en`]
module"]
pub type IRQ_EN = crate::Reg<irq_en::IRQ_EN_SPEC>;
#[doc = "Tamper interrupt enable"]
pub mod irq_en;
