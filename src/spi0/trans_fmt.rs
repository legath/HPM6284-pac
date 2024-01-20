#[doc = "Register `TransFmt` reader"]
pub type R = crate::R<TRANS_FMT_SPEC>;
#[doc = "Register `TransFmt` writer"]
pub type W = crate::W<TRANS_FMT_SPEC>;
#[doc = "Field `CPHA` reader - SPI Clock Phase 0x0: Sampling data at odd SCLK edges 0x1: Sampling data at even SCLK edges"]
pub type CPHA_R = crate::BitReader;
#[doc = "Field `CPHA` writer - SPI Clock Phase 0x0: Sampling data at odd SCLK edges 0x1: Sampling data at even SCLK edges"]
pub type CPHA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CPOL` reader - SPI Clock Polarity 0x0: SCLK is LOW in the idle states 0x1: SCLK is HIGH in the idle states"]
pub type CPOL_R = crate::BitReader;
#[doc = "Field `CPOL` writer - SPI Clock Polarity 0x0: SCLK is LOW in the idle states 0x1: SCLK is HIGH in the idle states"]
pub type CPOL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLVMODE` reader - SPI Master/Slave mode selection 0x0: Master mode 0x1: Slave mode"]
pub type SLVMODE_R = crate::BitReader;
#[doc = "Field `SLVMODE` writer - SPI Master/Slave mode selection 0x0: Master mode 0x1: Slave mode"]
pub type SLVMODE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LSB` reader - Transfer data with the least significant bit first 0x0: Most significant bit first 0x1: Least significant bit first"]
pub type LSB_R = crate::BitReader;
#[doc = "Field `LSB` writer - Transfer data with the least significant bit first 0x0: Most significant bit first 0x1: Least significant bit first"]
pub type LSB_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MOSIBIDIR` reader - Bi-directional MOSI in regular (single) mode 0x0: MOSI is uni-directional signal in regular mode. 0x1: MOSI is bi-directional signal in regular mode. This bi-directional signal replaces the two"]
pub type MOSIBIDIR_R = crate::BitReader;
#[doc = "Field `MOSIBIDIR` writer - Bi-directional MOSI in regular (single) mode 0x0: MOSI is uni-directional signal in regular mode. 0x1: MOSI is bi-directional signal in regular mode. This bi-directional signal replaces the two"]
pub type MOSIBIDIR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DATAMERGE` reader - Enable Data Merge mode, which does automatic data split on write and data coalescing on read. This bit only takes effect when DataLen = 0x7. Under Data Merge mode, each write to the Data Register will transmit all fourbytes of the write data; each read from the Data Register will retrieve four bytes of received data as a single word data. When Data Merge mode is disabled, only the least (DataLen+1) significient bits of the Data Register are valid for read/write operations; no automatic data split/coalescing will be performed."]
pub type DATAMERGE_R = crate::BitReader;
#[doc = "Field `DATAMERGE` writer - Enable Data Merge mode, which does automatic data split on write and data coalescing on read. This bit only takes effect when DataLen = 0x7. Under Data Merge mode, each write to the Data Register will transmit all fourbytes of the write data; each read from the Data Register will retrieve four bytes of received data as a single word data. When Data Merge mode is disabled, only the least (DataLen+1) significient bits of the Data Register are valid for read/write operations; no automatic data split/coalescing will be performed."]
pub type DATAMERGE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DATALEN` reader - The length of each data unit in bits The actual bit number of a data unit is (DataLen + 1)"]
pub type DATALEN_R = crate::FieldReader;
#[doc = "Field `DATALEN` writer - The length of each data unit in bits The actual bit number of a data unit is (DataLen + 1)"]
pub type DATALEN_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `ADDRLEN` reader - Address length in bytes 0x0: 1 byte 0x1: 2 bytes 0x2: 3 bytes 0x3: 4 bytes"]
pub type ADDRLEN_R = crate::FieldReader;
#[doc = "Field `ADDRLEN` writer - Address length in bytes 0x0: 1 byte 0x1: 2 bytes 0x2: 3 bytes 0x3: 4 bytes"]
pub type ADDRLEN_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bit 0 - SPI Clock Phase 0x0: Sampling data at odd SCLK edges 0x1: Sampling data at even SCLK edges"]
    #[inline(always)]
    pub fn cpha(&self) -> CPHA_R {
        CPHA_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SPI Clock Polarity 0x0: SCLK is LOW in the idle states 0x1: SCLK is HIGH in the idle states"]
    #[inline(always)]
    pub fn cpol(&self) -> CPOL_R {
        CPOL_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SPI Master/Slave mode selection 0x0: Master mode 0x1: Slave mode"]
    #[inline(always)]
    pub fn slvmode(&self) -> SLVMODE_R {
        SLVMODE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Transfer data with the least significant bit first 0x0: Most significant bit first 0x1: Least significant bit first"]
    #[inline(always)]
    pub fn lsb(&self) -> LSB_R {
        LSB_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Bi-directional MOSI in regular (single) mode 0x0: MOSI is uni-directional signal in regular mode. 0x1: MOSI is bi-directional signal in regular mode. This bi-directional signal replaces the two"]
    #[inline(always)]
    pub fn mosibidir(&self) -> MOSIBIDIR_R {
        MOSIBIDIR_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 7 - Enable Data Merge mode, which does automatic data split on write and data coalescing on read. This bit only takes effect when DataLen = 0x7. Under Data Merge mode, each write to the Data Register will transmit all fourbytes of the write data; each read from the Data Register will retrieve four bytes of received data as a single word data. When Data Merge mode is disabled, only the least (DataLen+1) significient bits of the Data Register are valid for read/write operations; no automatic data split/coalescing will be performed."]
    #[inline(always)]
    pub fn datamerge(&self) -> DATAMERGE_R {
        DATAMERGE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:12 - The length of each data unit in bits The actual bit number of a data unit is (DataLen + 1)"]
    #[inline(always)]
    pub fn datalen(&self) -> DATALEN_R {
        DATALEN_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:17 - Address length in bytes 0x0: 1 byte 0x1: 2 bytes 0x2: 3 bytes 0x3: 4 bytes"]
    #[inline(always)]
    pub fn addrlen(&self) -> ADDRLEN_R {
        ADDRLEN_R::new(((self.bits >> 16) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - SPI Clock Phase 0x0: Sampling data at odd SCLK edges 0x1: Sampling data at even SCLK edges"]
    #[inline(always)]
    #[must_use]
    pub fn cpha(&mut self) -> CPHA_W<TRANS_FMT_SPEC> {
        CPHA_W::new(self, 0)
    }
    #[doc = "Bit 1 - SPI Clock Polarity 0x0: SCLK is LOW in the idle states 0x1: SCLK is HIGH in the idle states"]
    #[inline(always)]
    #[must_use]
    pub fn cpol(&mut self) -> CPOL_W<TRANS_FMT_SPEC> {
        CPOL_W::new(self, 1)
    }
    #[doc = "Bit 2 - SPI Master/Slave mode selection 0x0: Master mode 0x1: Slave mode"]
    #[inline(always)]
    #[must_use]
    pub fn slvmode(&mut self) -> SLVMODE_W<TRANS_FMT_SPEC> {
        SLVMODE_W::new(self, 2)
    }
    #[doc = "Bit 3 - Transfer data with the least significant bit first 0x0: Most significant bit first 0x1: Least significant bit first"]
    #[inline(always)]
    #[must_use]
    pub fn lsb(&mut self) -> LSB_W<TRANS_FMT_SPEC> {
        LSB_W::new(self, 3)
    }
    #[doc = "Bit 4 - Bi-directional MOSI in regular (single) mode 0x0: MOSI is uni-directional signal in regular mode. 0x1: MOSI is bi-directional signal in regular mode. This bi-directional signal replaces the two"]
    #[inline(always)]
    #[must_use]
    pub fn mosibidir(&mut self) -> MOSIBIDIR_W<TRANS_FMT_SPEC> {
        MOSIBIDIR_W::new(self, 4)
    }
    #[doc = "Bit 7 - Enable Data Merge mode, which does automatic data split on write and data coalescing on read. This bit only takes effect when DataLen = 0x7. Under Data Merge mode, each write to the Data Register will transmit all fourbytes of the write data; each read from the Data Register will retrieve four bytes of received data as a single word data. When Data Merge mode is disabled, only the least (DataLen+1) significient bits of the Data Register are valid for read/write operations; no automatic data split/coalescing will be performed."]
    #[inline(always)]
    #[must_use]
    pub fn datamerge(&mut self) -> DATAMERGE_W<TRANS_FMT_SPEC> {
        DATAMERGE_W::new(self, 7)
    }
    #[doc = "Bits 8:12 - The length of each data unit in bits The actual bit number of a data unit is (DataLen + 1)"]
    #[inline(always)]
    #[must_use]
    pub fn datalen(&mut self) -> DATALEN_W<TRANS_FMT_SPEC> {
        DATALEN_W::new(self, 8)
    }
    #[doc = "Bits 16:17 - Address length in bytes 0x0: 1 byte 0x1: 2 bytes 0x2: 3 bytes 0x3: 4 bytes"]
    #[inline(always)]
    #[must_use]
    pub fn addrlen(&mut self) -> ADDRLEN_W<TRANS_FMT_SPEC> {
        ADDRLEN_W::new(self, 16)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Transfer Format Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`trans_fmt::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`trans_fmt::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TRANS_FMT_SPEC;
impl crate::RegisterSpec for TRANS_FMT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`trans_fmt::R`](R) reader structure"]
impl crate::Readable for TRANS_FMT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`trans_fmt::W`](W) writer structure"]
impl crate::Writable for TRANS_FMT_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TransFmt to value 0x0002_0780"]
impl crate::Resettable for TRANS_FMT_SPEC {
    const RESET_VALUE: u32 = 0x0002_0780;
}
