#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    channel: (),
}
impl RegisterBlock {
    #[doc = "0x00..0x70 - no description available"]
    #[inline(always)]
    pub const fn channel(&self, n: usize) -> &CHANNEL {
        #[allow(clippy::no_effect)]
        [(); 4][n];
        unsafe { &*(self as *const Self).cast::<u8>().add(0).add(32 * n).cast() }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x00..0x70 - no description available"]
    #[inline(always)]
    pub fn channel_iter(&self) -> impl Iterator<Item = &CHANNEL> {
        (0..4)
            .map(move |n| unsafe { &*(self as *const Self).cast::<u8>().add(0).add(32 * n).cast() })
    }
    #[doc = "0x00..0x1c - no description available"]
    #[inline(always)]
    pub const fn channelchn0(&self) -> &CHANNEL {
        self.channel(0)
    }
    #[doc = "0x20..0x3c - no description available"]
    #[inline(always)]
    pub const fn channelchn1(&self) -> &CHANNEL {
        self.channel(1)
    }
    #[doc = "0x40..0x5c - no description available"]
    #[inline(always)]
    pub const fn channelchn2(&self) -> &CHANNEL {
        self.channel(2)
    }
    #[doc = "0x60..0x7c - no description available"]
    #[inline(always)]
    pub const fn channelchn3(&self) -> &CHANNEL {
        self.channel(3)
    }
}
#[doc = "no description available"]
pub use self::channel::CHANNEL;
#[doc = r"Cluster"]
#[doc = "no description available"]
pub mod channel;
