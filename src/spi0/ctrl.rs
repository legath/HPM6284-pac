#[doc = "Register `Ctrl` reader"]
pub type R = crate::R<CTRL_SPEC>;
#[doc = "Register `Ctrl` writer"]
pub type W = crate::W<CTRL_SPEC>;
#[doc = "Field `SPIRST` reader - SPI reset Write 1 to reset. It is automatically cleared to 0 after the reset operation completes."]
pub type SPIRST_R = crate::BitReader;
#[doc = "Field `SPIRST` writer - SPI reset Write 1 to reset. It is automatically cleared to 0 after the reset operation completes."]
pub type SPIRST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXFIFORST` reader - Receive FIFO reset Write 1 to reset. It is automatically cleared to 0 after the reset operation completes."]
pub type RXFIFORST_R = crate::BitReader;
#[doc = "Field `RXFIFORST` writer - Receive FIFO reset Write 1 to reset. It is automatically cleared to 0 after the reset operation completes."]
pub type RXFIFORST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXFIFORST` reader - Transmit FIFO reset Write 1 to reset. It is automatically cleared to 0 after the reset operation completes."]
pub type TXFIFORST_R = crate::BitReader;
#[doc = "Field `TXFIFORST` writer - Transmit FIFO reset Write 1 to reset. It is automatically cleared to 0 after the reset operation completes."]
pub type TXFIFORST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXDMAEN` reader - RX DMA enable"]
pub type RXDMAEN_R = crate::BitReader;
#[doc = "Field `RXDMAEN` writer - RX DMA enable"]
pub type RXDMAEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXDMAEN` reader - TX DMA enable"]
pub type TXDMAEN_R = crate::BitReader;
#[doc = "Field `TXDMAEN` writer - TX DMA enable"]
pub type TXDMAEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXTHRES` reader - Receive (RX) FIFO Threshold The RXFIFOInt interrupt or DMA request would be issued for consuming the RX FIFO when the RX data count is more than or equal to the RX FIFO threshold."]
pub type RXTHRES_R = crate::FieldReader;
#[doc = "Field `RXTHRES` writer - Receive (RX) FIFO Threshold The RXFIFOInt interrupt or DMA request would be issued for consuming the RX FIFO when the RX data count is more than or equal to the RX FIFO threshold."]
pub type RXTHRES_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `TXTHRES` reader - Transmit (TX) FIFO Threshold The TXFIFOInt interrupt or DMA request would be issued to replenish the TX FIFO when the TX data count is less than or equal to the TX FIFO threshold."]
pub type TXTHRES_R = crate::FieldReader;
#[doc = "Field `TXTHRES` writer - Transmit (TX) FIFO Threshold The TXFIFOInt interrupt or DMA request would be issued to replenish the TX FIFO when the TX data count is less than or equal to the TX FIFO threshold."]
pub type TXTHRES_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bit 0 - SPI reset Write 1 to reset. It is automatically cleared to 0 after the reset operation completes."]
    #[inline(always)]
    pub fn spirst(&self) -> SPIRST_R {
        SPIRST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Receive FIFO reset Write 1 to reset. It is automatically cleared to 0 after the reset operation completes."]
    #[inline(always)]
    pub fn rxfiforst(&self) -> RXFIFORST_R {
        RXFIFORST_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Transmit FIFO reset Write 1 to reset. It is automatically cleared to 0 after the reset operation completes."]
    #[inline(always)]
    pub fn txfiforst(&self) -> TXFIFORST_R {
        TXFIFORST_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - RX DMA enable"]
    #[inline(always)]
    pub fn rxdmaen(&self) -> RXDMAEN_R {
        RXDMAEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - TX DMA enable"]
    #[inline(always)]
    pub fn txdmaen(&self) -> TXDMAEN_R {
        TXDMAEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 8:15 - Receive (RX) FIFO Threshold The RXFIFOInt interrupt or DMA request would be issued for consuming the RX FIFO when the RX data count is more than or equal to the RX FIFO threshold."]
    #[inline(always)]
    pub fn rxthres(&self) -> RXTHRES_R {
        RXTHRES_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Transmit (TX) FIFO Threshold The TXFIFOInt interrupt or DMA request would be issued to replenish the TX FIFO when the TX data count is less than or equal to the TX FIFO threshold."]
    #[inline(always)]
    pub fn txthres(&self) -> TXTHRES_R {
        TXTHRES_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - SPI reset Write 1 to reset. It is automatically cleared to 0 after the reset operation completes."]
    #[inline(always)]
    #[must_use]
    pub fn spirst(&mut self) -> SPIRST_W<CTRL_SPEC> {
        SPIRST_W::new(self, 0)
    }
    #[doc = "Bit 1 - Receive FIFO reset Write 1 to reset. It is automatically cleared to 0 after the reset operation completes."]
    #[inline(always)]
    #[must_use]
    pub fn rxfiforst(&mut self) -> RXFIFORST_W<CTRL_SPEC> {
        RXFIFORST_W::new(self, 1)
    }
    #[doc = "Bit 2 - Transmit FIFO reset Write 1 to reset. It is automatically cleared to 0 after the reset operation completes."]
    #[inline(always)]
    #[must_use]
    pub fn txfiforst(&mut self) -> TXFIFORST_W<CTRL_SPEC> {
        TXFIFORST_W::new(self, 2)
    }
    #[doc = "Bit 3 - RX DMA enable"]
    #[inline(always)]
    #[must_use]
    pub fn rxdmaen(&mut self) -> RXDMAEN_W<CTRL_SPEC> {
        RXDMAEN_W::new(self, 3)
    }
    #[doc = "Bit 4 - TX DMA enable"]
    #[inline(always)]
    #[must_use]
    pub fn txdmaen(&mut self) -> TXDMAEN_W<CTRL_SPEC> {
        TXDMAEN_W::new(self, 4)
    }
    #[doc = "Bits 8:15 - Receive (RX) FIFO Threshold The RXFIFOInt interrupt or DMA request would be issued for consuming the RX FIFO when the RX data count is more than or equal to the RX FIFO threshold."]
    #[inline(always)]
    #[must_use]
    pub fn rxthres(&mut self) -> RXTHRES_W<CTRL_SPEC> {
        RXTHRES_W::new(self, 8)
    }
    #[doc = "Bits 16:23 - Transmit (TX) FIFO Threshold The TXFIFOInt interrupt or DMA request would be issued to replenish the TX FIFO when the TX data count is less than or equal to the TX FIFO threshold."]
    #[inline(always)]
    #[must_use]
    pub fn txthres(&mut self) -> TXTHRES_W<CTRL_SPEC> {
        TXTHRES_W::new(self, 16)
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
#[doc = "Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTRL_SPEC;
impl crate::RegisterSpec for CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrl::R`](R) reader structure"]
impl crate::Readable for CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ctrl::W`](W) writer structure"]
impl crate::Writable for CTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets Ctrl to value 0"]
impl crate::Resettable for CTRL_SPEC {
    const RESET_VALUE: u32 = 0;
}
