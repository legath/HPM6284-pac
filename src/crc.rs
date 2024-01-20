#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    chn: (),
}
impl RegisterBlock {
    #[doc = "0x00..0x100 - no description available"]
    #[inline(always)]
    pub const fn chn(&self, n: usize) -> &CHN {
        #[allow(clippy::no_effect)]
        [(); 8][n];
        unsafe { &*(self as *const Self).cast::<u8>().add(0).add(64 * n).cast() }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x00..0x100 - no description available"]
    #[inline(always)]
    pub fn chn_iter(&self) -> impl Iterator<Item = &CHN> {
        (0..8)
            .map(move |n| unsafe { &*(self as *const Self).cast::<u8>().add(0).add(64 * n).cast() })
    }
}
#[doc = "no description available"]
pub use self::chn::CHN;
#[doc = r"Cluster"]
#[doc = "no description available"]
pub mod chn;
