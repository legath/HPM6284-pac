#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    channel: (),
    _reserved1: [u8; 0x0200],
    sr: SR,
    irqen: IRQEN,
    gcr: GCR,
}
impl RegisterBlock {
    #[doc = "0x00..0xd0 - no description available"]
    #[inline(always)]
    pub const fn channel(&self, n: usize) -> &CHANNEL {
        #[allow(clippy::no_effect)]
        [(); 4][n];
        unsafe { &*(self as *const Self).cast::<u8>().add(0).add(64 * n).cast() }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x00..0xd0 - no description available"]
    #[inline(always)]
    pub fn channel_iter(&self) -> impl Iterator<Item = &CHANNEL> {
        (0..4)
            .map(move |n| unsafe { &*(self as *const Self).cast::<u8>().add(0).add(64 * n).cast() })
    }
    #[doc = "0x00..0x34 - no description available"]
    #[inline(always)]
    pub const fn channelch0(&self) -> &CHANNEL {
        self.channel(0)
    }
    #[doc = "0x40..0x74 - no description available"]
    #[inline(always)]
    pub const fn channelch1(&self) -> &CHANNEL {
        self.channel(1)
    }
    #[doc = "0x80..0xb4 - no description available"]
    #[inline(always)]
    pub const fn channelch2(&self) -> &CHANNEL {
        self.channel(2)
    }
    #[doc = "0xc0..0xf4 - no description available"]
    #[inline(always)]
    pub const fn channelch3(&self) -> &CHANNEL {
        self.channel(3)
    }
    #[doc = "0x200 - Status register"]
    #[inline(always)]
    pub const fn sr(&self) -> &SR {
        &self.sr
    }
    #[doc = "0x204 - Interrupt request enable register"]
    #[inline(always)]
    pub const fn irqen(&self) -> &IRQEN {
        &self.irqen
    }
    #[doc = "0x208 - Global control register"]
    #[inline(always)]
    pub const fn gcr(&self) -> &GCR {
        &self.gcr
    }
}
#[doc = "no description available"]
pub use self::channel::CHANNEL;
#[doc = r"Cluster"]
#[doc = "no description available"]
pub mod channel;
#[doc = "SR (rw) register accessor: Status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sr`]
module"]
pub type SR = crate::Reg<sr::SR_SPEC>;
#[doc = "Status register"]
pub mod sr;
#[doc = "IRQEN (rw) register accessor: Interrupt request enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`irqen::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`irqen::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@irqen`]
module"]
pub type IRQEN = crate::Reg<irqen::IRQEN_SPEC>;
#[doc = "Interrupt request enable register"]
pub mod irqen;
#[doc = "GCR (rw) register accessor: Global control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gcr`]
module"]
pub type GCR = crate::Reg<gcr::GCR_SPEC>;
#[doc = "Global control register"]
pub mod gcr;
