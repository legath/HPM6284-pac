#[doc = r"Register block"]
#[repr(C)]
pub struct ASSIGN {
    pin: [PIN; 32],
}
impl ASSIGN {
    #[doc = "0x00..0x80 - no description available"]
    #[inline(always)]
    pub const fn pin(&self, n: usize) -> &PIN {
        &self.pin[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x00..0x80 - no description available"]
    #[inline(always)]
    pub fn pin_iter(&self) -> impl Iterator<Item = &PIN> {
        self.pin.iter()
    }
    #[doc = "0x00 - no description available"]
    #[inline(always)]
    pub const fn pinpin00(&self) -> &PIN {
        self.pin(0)
    }
    #[doc = "0x04 - no description available"]
    #[inline(always)]
    pub const fn pinpin01(&self) -> &PIN {
        self.pin(1)
    }
    #[doc = "0x08 - no description available"]
    #[inline(always)]
    pub const fn pinpin02(&self) -> &PIN {
        self.pin(2)
    }
    #[doc = "0x0c - no description available"]
    #[inline(always)]
    pub const fn pinpin03(&self) -> &PIN {
        self.pin(3)
    }
    #[doc = "0x10 - no description available"]
    #[inline(always)]
    pub const fn pinpin04(&self) -> &PIN {
        self.pin(4)
    }
    #[doc = "0x14 - no description available"]
    #[inline(always)]
    pub const fn pinpin05(&self) -> &PIN {
        self.pin(5)
    }
    #[doc = "0x18 - no description available"]
    #[inline(always)]
    pub const fn pinpin06(&self) -> &PIN {
        self.pin(6)
    }
    #[doc = "0x1c - no description available"]
    #[inline(always)]
    pub const fn pinpin07(&self) -> &PIN {
        self.pin(7)
    }
    #[doc = "0x20 - no description available"]
    #[inline(always)]
    pub const fn pinpin08(&self) -> &PIN {
        self.pin(8)
    }
    #[doc = "0x24 - no description available"]
    #[inline(always)]
    pub const fn pinpin09(&self) -> &PIN {
        self.pin(9)
    }
    #[doc = "0x28 - no description available"]
    #[inline(always)]
    pub const fn pinpin10(&self) -> &PIN {
        self.pin(10)
    }
    #[doc = "0x2c - no description available"]
    #[inline(always)]
    pub const fn pinpin11(&self) -> &PIN {
        self.pin(11)
    }
    #[doc = "0x30 - no description available"]
    #[inline(always)]
    pub const fn pinpin12(&self) -> &PIN {
        self.pin(12)
    }
    #[doc = "0x34 - no description available"]
    #[inline(always)]
    pub const fn pinpin13(&self) -> &PIN {
        self.pin(13)
    }
    #[doc = "0x38 - no description available"]
    #[inline(always)]
    pub const fn pinpin14(&self) -> &PIN {
        self.pin(14)
    }
    #[doc = "0x3c - no description available"]
    #[inline(always)]
    pub const fn pinpin15(&self) -> &PIN {
        self.pin(15)
    }
    #[doc = "0x40 - no description available"]
    #[inline(always)]
    pub const fn pinpin16(&self) -> &PIN {
        self.pin(16)
    }
    #[doc = "0x44 - no description available"]
    #[inline(always)]
    pub const fn pinpin17(&self) -> &PIN {
        self.pin(17)
    }
    #[doc = "0x48 - no description available"]
    #[inline(always)]
    pub const fn pinpin18(&self) -> &PIN {
        self.pin(18)
    }
    #[doc = "0x4c - no description available"]
    #[inline(always)]
    pub const fn pinpin19(&self) -> &PIN {
        self.pin(19)
    }
    #[doc = "0x50 - no description available"]
    #[inline(always)]
    pub const fn pinpin20(&self) -> &PIN {
        self.pin(20)
    }
    #[doc = "0x54 - no description available"]
    #[inline(always)]
    pub const fn pinpin21(&self) -> &PIN {
        self.pin(21)
    }
    #[doc = "0x58 - no description available"]
    #[inline(always)]
    pub const fn pinpin22(&self) -> &PIN {
        self.pin(22)
    }
    #[doc = "0x5c - no description available"]
    #[inline(always)]
    pub const fn pinpin23(&self) -> &PIN {
        self.pin(23)
    }
    #[doc = "0x60 - no description available"]
    #[inline(always)]
    pub const fn pinpin24(&self) -> &PIN {
        self.pin(24)
    }
    #[doc = "0x64 - no description available"]
    #[inline(always)]
    pub const fn pinpin25(&self) -> &PIN {
        self.pin(25)
    }
    #[doc = "0x68 - no description available"]
    #[inline(always)]
    pub const fn pinpin26(&self) -> &PIN {
        self.pin(26)
    }
    #[doc = "0x6c - no description available"]
    #[inline(always)]
    pub const fn pinpin27(&self) -> &PIN {
        self.pin(27)
    }
    #[doc = "0x70 - no description available"]
    #[inline(always)]
    pub const fn pinpin28(&self) -> &PIN {
        self.pin(28)
    }
    #[doc = "0x74 - no description available"]
    #[inline(always)]
    pub const fn pinpin29(&self) -> &PIN {
        self.pin(29)
    }
    #[doc = "0x78 - no description available"]
    #[inline(always)]
    pub const fn pinpin30(&self) -> &PIN {
        self.pin(30)
    }
    #[doc = "0x7c - no description available"]
    #[inline(always)]
    pub const fn pinpin31(&self) -> &PIN {
        self.pin(31)
    }
}
#[doc = "PIN (rw) register accessor: no description available\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pin::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pin::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pin`]
module"]
pub type PIN = crate::Reg<pin::PIN_SPEC>;
#[doc = "no description available"]
pub mod pin;
