#[doc = "Register `IntrSt` reader"]
pub type R = crate::R<INTR_ST_SPEC>;
#[doc = "Register `IntrSt` writer"]
pub type W = crate::W<INTR_ST_SPEC>;
#[doc = "Field `RXFIFOORINT` writer - RX FIFO Overrun interrupt. This bit is set when RX FIFO Overrun interrupts occur. (Slave mode only)"]
pub type RXFIFOORINT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXFIFOURINT` writer - TX FIFO Underrun interrupt. This bit is set when TX FIFO Underrun interrupts occur. (Slave mode only)"]
pub type TXFIFOURINT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXFIFOINT` writer - RX FIFO Threshold interrupt. This bit is set when RX FIFO Threshold interrupts occur."]
pub type RXFIFOINT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXFIFOINT` writer - TX FIFO Threshold interrupt. This bit is set when TX FIFO Threshold interrupts occur."]
pub type TXFIFOINT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENDINT` writer - End of SPI Transfer interrupt. This bit is set when End of SPI Transfer interrupts occur."]
pub type ENDINT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLVCMDINT` writer - Slave Command Interrupt. This bit is set when Slave Command interrupts occur. (Slave mode only)"]
pub type SLVCMDINT_W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - RX FIFO Overrun interrupt. This bit is set when RX FIFO Overrun interrupts occur. (Slave mode only)"]
    #[inline(always)]
    #[must_use]
    pub fn rxfifoorint(&mut self) -> RXFIFOORINT_W<INTR_ST_SPEC> {
        RXFIFOORINT_W::new(self, 0)
    }
    #[doc = "Bit 1 - TX FIFO Underrun interrupt. This bit is set when TX FIFO Underrun interrupts occur. (Slave mode only)"]
    #[inline(always)]
    #[must_use]
    pub fn txfifourint(&mut self) -> TXFIFOURINT_W<INTR_ST_SPEC> {
        TXFIFOURINT_W::new(self, 1)
    }
    #[doc = "Bit 2 - RX FIFO Threshold interrupt. This bit is set when RX FIFO Threshold interrupts occur."]
    #[inline(always)]
    #[must_use]
    pub fn rxfifoint(&mut self) -> RXFIFOINT_W<INTR_ST_SPEC> {
        RXFIFOINT_W::new(self, 2)
    }
    #[doc = "Bit 3 - TX FIFO Threshold interrupt. This bit is set when TX FIFO Threshold interrupts occur."]
    #[inline(always)]
    #[must_use]
    pub fn txfifoint(&mut self) -> TXFIFOINT_W<INTR_ST_SPEC> {
        TXFIFOINT_W::new(self, 3)
    }
    #[doc = "Bit 4 - End of SPI Transfer interrupt. This bit is set when End of SPI Transfer interrupts occur."]
    #[inline(always)]
    #[must_use]
    pub fn endint(&mut self) -> ENDINT_W<INTR_ST_SPEC> {
        ENDINT_W::new(self, 4)
    }
    #[doc = "Bit 5 - Slave Command Interrupt. This bit is set when Slave Command interrupts occur. (Slave mode only)"]
    #[inline(always)]
    #[must_use]
    pub fn slvcmdint(&mut self) -> SLVCMDINT_W<INTR_ST_SPEC> {
        SLVCMDINT_W::new(self, 5)
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
#[doc = "Interrupt Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intr_st::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intr_st::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INTR_ST_SPEC;
impl crate::RegisterSpec for INTR_ST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intr_st::R`](R) reader structure"]
impl crate::Readable for INTR_ST_SPEC {}
#[doc = "`write(|w| ..)` method takes [`intr_st::W`](W) writer structure"]
impl crate::Writable for INTR_ST_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IntrSt to value 0"]
impl crate::Resettable for INTR_ST_SPEC {
    const RESET_VALUE: u32 = 0;
}
