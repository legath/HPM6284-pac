#[doc = "Register `Status` reader"]
pub type R = crate::R<STATUS_SPEC>;
#[doc = "Register `Status` writer"]
pub type W = crate::W<STATUS_SPEC>;
#[doc = "Field `SPIACTIVE` reader - SPI register programming is in progress. In master mode, SPIActive becomes 1 after the SPI command register is written and becomes 0 after the transfer is finished. In slave mode, SPIActive becomes 1 after the SPI CS signal is asserted and becomes 0 after the SPI CS signal is deasserted. Note that due to clock synchronization, it may take at most two spi_clock cycles for SPIActive to change when the corresponding condition happens. Note this bit stays 0 when Direct IO Control or the memory-mapped interface is used."]
pub type SPIACTIVE_R = crate::BitReader;
#[doc = "Field `RXNUM_5_0` reader - Number of valid entries in the Receive FIFO"]
pub type RXNUM_5_0_R = crate::FieldReader;
#[doc = "Field `RXEMPTY` reader - Receive FIFO Empty flag"]
pub type RXEMPTY_R = crate::BitReader;
#[doc = "Field `RXFULL` reader - Receive FIFO Full flag"]
pub type RXFULL_R = crate::BitReader;
#[doc = "Field `TXNUM_5_0` reader - Number of valid entries in the Transmit FIFO"]
pub type TXNUM_5_0_R = crate::FieldReader;
#[doc = "Field `TXEMPTY` reader - Transmit FIFO Empty flag"]
pub type TXEMPTY_R = crate::BitReader;
#[doc = "Field `TXFULL` reader - Transmit FIFO Full flag"]
pub type TXFULL_R = crate::BitReader;
#[doc = "Field `RXNUM_7_6` reader - Number of valid entries in the Receive FIFO"]
pub type RXNUM_7_6_R = crate::FieldReader;
#[doc = "Field `TXNUM_7_6` reader - Number of valid entries in the Transmit FIFO"]
pub type TXNUM_7_6_R = crate::FieldReader;
impl R {
    #[doc = "Bit 0 - SPI register programming is in progress. In master mode, SPIActive becomes 1 after the SPI command register is written and becomes 0 after the transfer is finished. In slave mode, SPIActive becomes 1 after the SPI CS signal is asserted and becomes 0 after the SPI CS signal is deasserted. Note that due to clock synchronization, it may take at most two spi_clock cycles for SPIActive to change when the corresponding condition happens. Note this bit stays 0 when Direct IO Control or the memory-mapped interface is used."]
    #[inline(always)]
    pub fn spiactive(&self) -> SPIACTIVE_R {
        SPIACTIVE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 8:13 - Number of valid entries in the Receive FIFO"]
    #[inline(always)]
    pub fn rxnum_5_0(&self) -> RXNUM_5_0_R {
        RXNUM_5_0_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bit 14 - Receive FIFO Empty flag"]
    #[inline(always)]
    pub fn rxempty(&self) -> RXEMPTY_R {
        RXEMPTY_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Receive FIFO Full flag"]
    #[inline(always)]
    pub fn rxfull(&self) -> RXFULL_R {
        RXFULL_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:21 - Number of valid entries in the Transmit FIFO"]
    #[inline(always)]
    pub fn txnum_5_0(&self) -> TXNUM_5_0_R {
        TXNUM_5_0_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bit 22 - Transmit FIFO Empty flag"]
    #[inline(always)]
    pub fn txempty(&self) -> TXEMPTY_R {
        TXEMPTY_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Transmit FIFO Full flag"]
    #[inline(always)]
    pub fn txfull(&self) -> TXFULL_R {
        TXFULL_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:25 - Number of valid entries in the Receive FIFO"]
    #[inline(always)]
    pub fn rxnum_7_6(&self) -> RXNUM_7_6_R {
        RXNUM_7_6_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 28:29 - Number of valid entries in the Transmit FIFO"]
    #[inline(always)]
    pub fn txnum_7_6(&self) -> TXNUM_7_6_R {
        TXNUM_7_6_R::new(((self.bits >> 28) & 3) as u8)
    }
}
impl W {
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
#[doc = "Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`status::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`status::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STATUS_SPEC;
impl crate::RegisterSpec for STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`status::R`](R) reader structure"]
impl crate::Readable for STATUS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`status::W`](W) writer structure"]
impl crate::Writable for STATUS_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets Status to value 0"]
impl crate::Resettable for STATUS_SPEC {
    const RESET_VALUE: u32 = 0;
}
