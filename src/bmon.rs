#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    monitor: (),
}
impl RegisterBlock {
    #[doc = "0x00..0x10 - no description available"]
    #[inline(always)]
    pub const fn monitor(&self, n: usize) -> &MONITOR {
        #[allow(clippy::no_effect)]
        [(); 2][n];
        unsafe { &*(self as *const Self).cast::<u8>().add(0).add(16 * n).cast() }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x00..0x10 - no description available"]
    #[inline(always)]
    pub fn monitor_iter(&self) -> impl Iterator<Item = &MONITOR> {
        (0..2)
            .map(move |n| unsafe { &*(self as *const Self).cast::<u8>().add(0).add(16 * n).cast() })
    }
    #[doc = "0x00..0x08 - no description available"]
    #[inline(always)]
    pub const fn monitorglitch0(&self) -> &MONITOR {
        self.monitor(0)
    }
    #[doc = "0x10..0x18 - no description available"]
    #[inline(always)]
    pub const fn monitorclock0(&self) -> &MONITOR {
        self.monitor(1)
    }
}
#[doc = "no description available"]
pub use self::monitor::MONITOR;
#[doc = r"Cluster"]
#[doc = "no description available"]
pub mod monitor;
