#[doc = "Register `IntrEn` reader"]
pub type R = crate::R<INTR_EN_SPEC>;
#[doc = "Register `IntrEn` writer"]
pub type W = crate::W<INTR_EN_SPEC>;
#[doc = "Field `RXFIFOORINTEN` reader - Enable the SPI Receive FIFO Overrun interrupt. Control whether interrupts are triggered when the Receive FIFO overflows. (Slave mode only)"]
pub type RXFIFOORINTEN_R = crate::BitReader;
#[doc = "Field `RXFIFOORINTEN` writer - Enable the SPI Receive FIFO Overrun interrupt. Control whether interrupts are triggered when the Receive FIFO overflows. (Slave mode only)"]
pub type RXFIFOORINTEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXFIFOURINTEN` reader - Enable the SPI Transmit FIFO Underrun interrupt. Control whether interrupts are triggered when the Transmit FIFO run out of data. (Slave mode only)"]
pub type TXFIFOURINTEN_R = crate::BitReader;
#[doc = "Field `TXFIFOURINTEN` writer - Enable the SPI Transmit FIFO Underrun interrupt. Control whether interrupts are triggered when the Transmit FIFO run out of data. (Slave mode only)"]
pub type TXFIFOURINTEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXFIFOINTEN` reader - Enable the SPI Receive FIFO Threshold interrupt. Control whether interrupts are triggered when the valid entries are greater than or equal to the RX FIFO threshold."]
pub type RXFIFOINTEN_R = crate::BitReader;
#[doc = "Field `RXFIFOINTEN` writer - Enable the SPI Receive FIFO Threshold interrupt. Control whether interrupts are triggered when the valid entries are greater than or equal to the RX FIFO threshold."]
pub type RXFIFOINTEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXFIFOINTEN` reader - Enable the SPI Transmit FIFO Threshold interrupt. Control whether interrupts are triggered when the valid entries are less than or equal to the TX FIFO threshold."]
pub type TXFIFOINTEN_R = crate::BitReader;
#[doc = "Field `TXFIFOINTEN` writer - Enable the SPI Transmit FIFO Threshold interrupt. Control whether interrupts are triggered when the valid entries are less than or equal to the TX FIFO threshold."]
pub type TXFIFOINTEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENDINTEN` reader - Enable the End of SPI Transfer interrupt. Control whether interrupts are triggered when SPI transfers end. (In slave mode, end of read status transaction doesn鈥檛 trigger this interrupt.)"]
pub type ENDINTEN_R = crate::BitReader;
#[doc = "Field `ENDINTEN` writer - Enable the End of SPI Transfer interrupt. Control whether interrupts are triggered when SPI transfers end. (In slave mode, end of read status transaction doesn鈥檛 trigger this interrupt.)"]
pub type ENDINTEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLVCMDEN` reader - Enable the Slave Command Interrupt. Control whether interrupts are triggered whenever slave commands are received. (Slave mode only)"]
pub type SLVCMDEN_R = crate::BitReader;
#[doc = "Field `SLVCMDEN` writer - Enable the Slave Command Interrupt. Control whether interrupts are triggered whenever slave commands are received. (Slave mode only)"]
pub type SLVCMDEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Enable the SPI Receive FIFO Overrun interrupt. Control whether interrupts are triggered when the Receive FIFO overflows. (Slave mode only)"]
    #[inline(always)]
    pub fn rxfifoorinten(&self) -> RXFIFOORINTEN_R {
        RXFIFOORINTEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable the SPI Transmit FIFO Underrun interrupt. Control whether interrupts are triggered when the Transmit FIFO run out of data. (Slave mode only)"]
    #[inline(always)]
    pub fn txfifourinten(&self) -> TXFIFOURINTEN_R {
        TXFIFOURINTEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable the SPI Receive FIFO Threshold interrupt. Control whether interrupts are triggered when the valid entries are greater than or equal to the RX FIFO threshold."]
    #[inline(always)]
    pub fn rxfifointen(&self) -> RXFIFOINTEN_R {
        RXFIFOINTEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enable the SPI Transmit FIFO Threshold interrupt. Control whether interrupts are triggered when the valid entries are less than or equal to the TX FIFO threshold."]
    #[inline(always)]
    pub fn txfifointen(&self) -> TXFIFOINTEN_R {
        TXFIFOINTEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Enable the End of SPI Transfer interrupt. Control whether interrupts are triggered when SPI transfers end. (In slave mode, end of read status transaction doesn鈥檛 trigger this interrupt.)"]
    #[inline(always)]
    pub fn endinten(&self) -> ENDINTEN_R {
        ENDINTEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Enable the Slave Command Interrupt. Control whether interrupts are triggered whenever slave commands are received. (Slave mode only)"]
    #[inline(always)]
    pub fn slvcmden(&self) -> SLVCMDEN_R {
        SLVCMDEN_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable the SPI Receive FIFO Overrun interrupt. Control whether interrupts are triggered when the Receive FIFO overflows. (Slave mode only)"]
    #[inline(always)]
    #[must_use]
    pub fn rxfifoorinten(&mut self) -> RXFIFOORINTEN_W<INTR_EN_SPEC> {
        RXFIFOORINTEN_W::new(self, 0)
    }
    #[doc = "Bit 1 - Enable the SPI Transmit FIFO Underrun interrupt. Control whether interrupts are triggered when the Transmit FIFO run out of data. (Slave mode only)"]
    #[inline(always)]
    #[must_use]
    pub fn txfifourinten(&mut self) -> TXFIFOURINTEN_W<INTR_EN_SPEC> {
        TXFIFOURINTEN_W::new(self, 1)
    }
    #[doc = "Bit 2 - Enable the SPI Receive FIFO Threshold interrupt. Control whether interrupts are triggered when the valid entries are greater than or equal to the RX FIFO threshold."]
    #[inline(always)]
    #[must_use]
    pub fn rxfifointen(&mut self) -> RXFIFOINTEN_W<INTR_EN_SPEC> {
        RXFIFOINTEN_W::new(self, 2)
    }
    #[doc = "Bit 3 - Enable the SPI Transmit FIFO Threshold interrupt. Control whether interrupts are triggered when the valid entries are less than or equal to the TX FIFO threshold."]
    #[inline(always)]
    #[must_use]
    pub fn txfifointen(&mut self) -> TXFIFOINTEN_W<INTR_EN_SPEC> {
        TXFIFOINTEN_W::new(self, 3)
    }
    #[doc = "Bit 4 - Enable the End of SPI Transfer interrupt. Control whether interrupts are triggered when SPI transfers end. (In slave mode, end of read status transaction doesn鈥檛 trigger this interrupt.)"]
    #[inline(always)]
    #[must_use]
    pub fn endinten(&mut self) -> ENDINTEN_W<INTR_EN_SPEC> {
        ENDINTEN_W::new(self, 4)
    }
    #[doc = "Bit 5 - Enable the Slave Command Interrupt. Control whether interrupts are triggered whenever slave commands are received. (Slave mode only)"]
    #[inline(always)]
    #[must_use]
    pub fn slvcmden(&mut self) -> SLVCMDEN_W<INTR_EN_SPEC> {
        SLVCMDEN_W::new(self, 5)
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
#[doc = "Interrupt Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intr_en::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intr_en::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INTR_EN_SPEC;
impl crate::RegisterSpec for INTR_EN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intr_en::R`](R) reader structure"]
impl crate::Readable for INTR_EN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`intr_en::W`](W) writer structure"]
impl crate::Writable for INTR_EN_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IntrEn to value 0"]
impl crate::Resettable for INTR_EN_SPEC {
    const RESET_VALUE: u32 = 0;
}
