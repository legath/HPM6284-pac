#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    assign: [ASSIGN; 16],
}
impl RegisterBlock {
    #[doc = "0x00..0x800 - no description available"]
    #[inline(always)]
    pub const fn assign(&self, n: usize) -> &ASSIGN {
        &self.assign[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x00..0x800 - no description available"]
    #[inline(always)]
    pub fn assign_iter(&self) -> impl Iterator<Item = &ASSIGN> {
        self.assign.iter()
    }
    #[doc = "0x00..0x80 - no description available"]
    #[inline(always)]
    pub const fn assigngpioa(&self) -> &ASSIGN {
        self.assign(0)
    }
    #[doc = "0x80..0x100 - no description available"]
    #[inline(always)]
    pub const fn assigngpiob(&self) -> &ASSIGN {
        self.assign(1)
    }
    #[doc = "0x100..0x180 - no description available"]
    #[inline(always)]
    pub const fn assigngpioc(&self) -> &ASSIGN {
        self.assign(2)
    }
    #[doc = "0x180..0x200 - no description available"]
    #[inline(always)]
    pub const fn assignrsv(&self) -> &ASSIGN {
        self.assign(3)
    }
    #[doc = "0x200..0x280 - no description available"]
    #[inline(always)]
    pub const fn assignrsv(&self) -> &ASSIGN {
        self.assign(4)
    }
    #[doc = "0x280..0x300 - no description available"]
    #[inline(always)]
    pub const fn assignrsv(&self) -> &ASSIGN {
        self.assign(5)
    }
    #[doc = "0x300..0x380 - no description available"]
    #[inline(always)]
    pub const fn assignrsv(&self) -> &ASSIGN {
        self.assign(6)
    }
    #[doc = "0x380..0x400 - no description available"]
    #[inline(always)]
    pub const fn assignrsv(&self) -> &ASSIGN {
        self.assign(7)
    }
    #[doc = "0x400..0x480 - no description available"]
    #[inline(always)]
    pub const fn assignrsv(&self) -> &ASSIGN {
        self.assign(8)
    }
    #[doc = "0x480..0x500 - no description available"]
    #[inline(always)]
    pub const fn assignrsv(&self) -> &ASSIGN {
        self.assign(9)
    }
    #[doc = "0x500..0x580 - no description available"]
    #[inline(always)]
    pub const fn assignrsv(&self) -> &ASSIGN {
        self.assign(10)
    }
    #[doc = "0x580..0x600 - no description available"]
    #[inline(always)]
    pub const fn assignrsv(&self) -> &ASSIGN {
        self.assign(11)
    }
    #[doc = "0x600..0x680 - no description available"]
    #[inline(always)]
    pub const fn assignrsv(&self) -> &ASSIGN {
        self.assign(12)
    }
    #[doc = "0x680..0x700 - no description available"]
    #[inline(always)]
    pub const fn assigngpiox(&self) -> &ASSIGN {
        self.assign(13)
    }
    #[doc = "0x700..0x780 - no description available"]
    #[inline(always)]
    pub const fn assigngpioy(&self) -> &ASSIGN {
        self.assign(14)
    }
    #[doc = "0x780..0x800 - no description available"]
    #[inline(always)]
    pub const fn assigngpioz(&self) -> &ASSIGN {
        self.assign(15)
    }
}
#[doc = "no description available"]
pub use self::assign::ASSIGN;
#[doc = r"Cluster"]
#[doc = "no description available"]
pub mod assign;
