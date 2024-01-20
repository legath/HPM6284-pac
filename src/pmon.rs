#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    monitor: [MONITOR; 4],
    _reserved1: [u8; 0x20],
    irq_flag: IRQ_FLAG,
    irq_enable: IRQ_ENABLE,
}
impl RegisterBlock {
    #[doc = "0x00..0x20 - no description available"]
    #[inline(always)]
    pub const fn monitor(&self, n: usize) -> &MONITOR {
        &self.monitor[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x00..0x20 - no description available"]
    #[inline(always)]
    pub fn monitor_iter(&self) -> impl Iterator<Item = &MONITOR> {
        self.monitor.iter()
    }
    #[doc = "0x00..0x08 - no description available"]
    #[inline(always)]
    pub const fn monitorglitch0(&self) -> &MONITOR {
        self.monitor(0)
    }
    #[doc = "0x08..0x10 - no description available"]
    #[inline(always)]
    pub const fn monitorglitch1(&self) -> &MONITOR {
        self.monitor(1)
    }
    #[doc = "0x10..0x18 - no description available"]
    #[inline(always)]
    pub const fn monitorclock0(&self) -> &MONITOR {
        self.monitor(2)
    }
    #[doc = "0x18..0x20 - no description available"]
    #[inline(always)]
    pub const fn monitorclock1(&self) -> &MONITOR {
        self.monitor(3)
    }
    #[doc = "0x40 - No description avaiable"]
    #[inline(always)]
    pub const fn irq_flag(&self) -> &IRQ_FLAG {
        &self.irq_flag
    }
    #[doc = "0x44 - No description avaiable"]
    #[inline(always)]
    pub const fn irq_enable(&self) -> &IRQ_ENABLE {
        &self.irq_enable
    }
}
#[doc = "no description available"]
pub use self::monitor::MONITOR;
#[doc = r"Cluster"]
#[doc = "no description available"]
pub mod monitor;
#[doc = "IRQ_FLAG (rw) register accessor: No description avaiable\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`irq_flag::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`irq_flag::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@irq_flag`]
module"]
pub type IRQ_FLAG = crate::Reg<irq_flag::IRQ_FLAG_SPEC>;
#[doc = "No description avaiable"]
pub mod irq_flag;
#[doc = "IRQ_ENABLE (rw) register accessor: No description avaiable\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`irq_enable::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`irq_enable::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@irq_enable`]
module"]
pub type IRQ_ENABLE = crate::Reg<irq_enable::IRQ_ENABLE_SPEC>;
#[doc = "No description avaiable"]
pub mod irq_enable;
