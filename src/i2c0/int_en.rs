#[doc = "Register `IntEn` reader"]
pub type R = crate::R<INT_EN_SPEC>;
#[doc = "Register `IntEn` writer"]
pub type W = crate::W<INT_EN_SPEC>;
#[doc = "Field `FIFOEMPTY` reader - Set to enabled the FIFO Empty Interrupt Interrupts when the FIFO is empty."]
pub type FIFOEMPTY_R = crate::BitReader;
#[doc = "Field `FIFOEMPTY` writer - Set to enabled the FIFO Empty Interrupt Interrupts when the FIFO is empty."]
pub type FIFOEMPTY_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FIFOFULL` reader - Set to enable the FIFO Full Interrupt. Interrupts when the FIFO is full."]
pub type FIFOFULL_R = crate::BitReader;
#[doc = "Field `FIFOFULL` writer - Set to enable the FIFO Full Interrupt. Interrupts when the FIFO is full."]
pub type FIFOFULL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FIFOHALF` reader - Set to enable the FIFO Half Interrupt. Receiver: Interrupts when the FIFO is half-empty, i.e, there is >= 1/2 entries in the FIFO. Transmitter: Interrupts when the FIFO is half-empty, i.e. there is &lt;= 1/2 entries in the FIFO. This interrupt depends on the transaction direction; don’t enable this interrupt unless the transfer direction is determined, otherwise unintended interrupts may be triggered."]
pub type FIFOHALF_R = crate::BitReader;
#[doc = "Field `FIFOHALF` writer - Set to enable the FIFO Half Interrupt. Receiver: Interrupts when the FIFO is half-empty, i.e, there is >= 1/2 entries in the FIFO. Transmitter: Interrupts when the FIFO is half-empty, i.e. there is &lt;= 1/2 entries in the FIFO. This interrupt depends on the transaction direction; don’t enable this interrupt unless the transfer direction is determined, otherwise unintended interrupts may be triggered."]
pub type FIFOHALF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADDRHIT` reader - Set to enable the Address Hit Interrupt. Master: interrupts when the addressed slave returned an ACK. Slave: interrupts when the controller is addressed."]
pub type ADDRHIT_R = crate::BitReader;
#[doc = "Field `ADDRHIT` writer - Set to enable the Address Hit Interrupt. Master: interrupts when the addressed slave returned an ACK. Slave: interrupts when the controller is addressed."]
pub type ADDRHIT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ARBLOSE` reader - Set to enable the Arbitration Lose Interrupt. Master: interrupts when the controller loses the bus arbitration Slave: not available in this mode."]
pub type ARBLOSE_R = crate::BitReader;
#[doc = "Field `ARBLOSE` writer - Set to enable the Arbitration Lose Interrupt. Master: interrupts when the controller loses the bus arbitration Slave: not available in this mode."]
pub type ARBLOSE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STOP` reader - Set to enable the STOP Condition Interrupt Interrupts when a STOP condition is detected."]
pub type STOP_R = crate::BitReader;
#[doc = "Field `STOP` writer - Set to enable the STOP Condition Interrupt Interrupts when a STOP condition is detected."]
pub type STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `START` reader - Set to enable the START Condition Interrupt. Interrupts when a START condition/repeated START condition is detected."]
pub type START_R = crate::BitReader;
#[doc = "Field `START` writer - Set to enable the START Condition Interrupt. Interrupts when a START condition/repeated START condition is detected."]
pub type START_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BYTETRANS` reader - Set to enable the Byte Transmit Interrupt. Interrupts when a byte of data is transmitted."]
pub type BYTETRANS_R = crate::BitReader;
#[doc = "Field `BYTETRANS` writer - Set to enable the Byte Transmit Interrupt. Interrupts when a byte of data is transmitted."]
pub type BYTETRANS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BYTERECV` reader - Set to enable the Byte Receive Interrupt. Interrupts when a byte of data is received Auto-ACK will be disabled if this interrupt is enabled, that is, the software needs to ACK/NACK the received byte manually."]
pub type BYTERECV_R = crate::BitReader;
#[doc = "Field `BYTERECV` writer - Set to enable the Byte Receive Interrupt. Interrupts when a byte of data is received Auto-ACK will be disabled if this interrupt is enabled, that is, the software needs to ACK/NACK the received byte manually."]
pub type BYTERECV_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMPL` reader - Set to enable the Completion Interrupt. Master: interrupts when a transaction is issued from this master and completed without losing the bus arbitration. Slave: interrupts when a transaction addressing the controller is completed."]
pub type CMPL_R = crate::BitReader;
#[doc = "Field `CMPL` writer - Set to enable the Completion Interrupt. Master: interrupts when a transaction is issued from this master and completed without losing the bus arbitration. Slave: interrupts when a transaction addressing the controller is completed."]
pub type CMPL_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Set to enabled the FIFO Empty Interrupt Interrupts when the FIFO is empty."]
    #[inline(always)]
    pub fn fifoempty(&self) -> FIFOEMPTY_R {
        FIFOEMPTY_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Set to enable the FIFO Full Interrupt. Interrupts when the FIFO is full."]
    #[inline(always)]
    pub fn fifofull(&self) -> FIFOFULL_R {
        FIFOFULL_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Set to enable the FIFO Half Interrupt. Receiver: Interrupts when the FIFO is half-empty, i.e, there is >= 1/2 entries in the FIFO. Transmitter: Interrupts when the FIFO is half-empty, i.e. there is &lt;= 1/2 entries in the FIFO. This interrupt depends on the transaction direction; don’t enable this interrupt unless the transfer direction is determined, otherwise unintended interrupts may be triggered."]
    #[inline(always)]
    pub fn fifohalf(&self) -> FIFOHALF_R {
        FIFOHALF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Set to enable the Address Hit Interrupt. Master: interrupts when the addressed slave returned an ACK. Slave: interrupts when the controller is addressed."]
    #[inline(always)]
    pub fn addrhit(&self) -> ADDRHIT_R {
        ADDRHIT_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Set to enable the Arbitration Lose Interrupt. Master: interrupts when the controller loses the bus arbitration Slave: not available in this mode."]
    #[inline(always)]
    pub fn arblose(&self) -> ARBLOSE_R {
        ARBLOSE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Set to enable the STOP Condition Interrupt Interrupts when a STOP condition is detected."]
    #[inline(always)]
    pub fn stop(&self) -> STOP_R {
        STOP_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Set to enable the START Condition Interrupt. Interrupts when a START condition/repeated START condition is detected."]
    #[inline(always)]
    pub fn start(&self) -> START_R {
        START_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Set to enable the Byte Transmit Interrupt. Interrupts when a byte of data is transmitted."]
    #[inline(always)]
    pub fn bytetrans(&self) -> BYTETRANS_R {
        BYTETRANS_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Set to enable the Byte Receive Interrupt. Interrupts when a byte of data is received Auto-ACK will be disabled if this interrupt is enabled, that is, the software needs to ACK/NACK the received byte manually."]
    #[inline(always)]
    pub fn byterecv(&self) -> BYTERECV_R {
        BYTERECV_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Set to enable the Completion Interrupt. Master: interrupts when a transaction is issued from this master and completed without losing the bus arbitration. Slave: interrupts when a transaction addressing the controller is completed."]
    #[inline(always)]
    pub fn cmpl(&self) -> CMPL_R {
        CMPL_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Set to enabled the FIFO Empty Interrupt Interrupts when the FIFO is empty."]
    #[inline(always)]
    #[must_use]
    pub fn fifoempty(&mut self) -> FIFOEMPTY_W<INT_EN_SPEC> {
        FIFOEMPTY_W::new(self, 0)
    }
    #[doc = "Bit 1 - Set to enable the FIFO Full Interrupt. Interrupts when the FIFO is full."]
    #[inline(always)]
    #[must_use]
    pub fn fifofull(&mut self) -> FIFOFULL_W<INT_EN_SPEC> {
        FIFOFULL_W::new(self, 1)
    }
    #[doc = "Bit 2 - Set to enable the FIFO Half Interrupt. Receiver: Interrupts when the FIFO is half-empty, i.e, there is >= 1/2 entries in the FIFO. Transmitter: Interrupts when the FIFO is half-empty, i.e. there is &lt;= 1/2 entries in the FIFO. This interrupt depends on the transaction direction; don’t enable this interrupt unless the transfer direction is determined, otherwise unintended interrupts may be triggered."]
    #[inline(always)]
    #[must_use]
    pub fn fifohalf(&mut self) -> FIFOHALF_W<INT_EN_SPEC> {
        FIFOHALF_W::new(self, 2)
    }
    #[doc = "Bit 3 - Set to enable the Address Hit Interrupt. Master: interrupts when the addressed slave returned an ACK. Slave: interrupts when the controller is addressed."]
    #[inline(always)]
    #[must_use]
    pub fn addrhit(&mut self) -> ADDRHIT_W<INT_EN_SPEC> {
        ADDRHIT_W::new(self, 3)
    }
    #[doc = "Bit 4 - Set to enable the Arbitration Lose Interrupt. Master: interrupts when the controller loses the bus arbitration Slave: not available in this mode."]
    #[inline(always)]
    #[must_use]
    pub fn arblose(&mut self) -> ARBLOSE_W<INT_EN_SPEC> {
        ARBLOSE_W::new(self, 4)
    }
    #[doc = "Bit 5 - Set to enable the STOP Condition Interrupt Interrupts when a STOP condition is detected."]
    #[inline(always)]
    #[must_use]
    pub fn stop(&mut self) -> STOP_W<INT_EN_SPEC> {
        STOP_W::new(self, 5)
    }
    #[doc = "Bit 6 - Set to enable the START Condition Interrupt. Interrupts when a START condition/repeated START condition is detected."]
    #[inline(always)]
    #[must_use]
    pub fn start(&mut self) -> START_W<INT_EN_SPEC> {
        START_W::new(self, 6)
    }
    #[doc = "Bit 7 - Set to enable the Byte Transmit Interrupt. Interrupts when a byte of data is transmitted."]
    #[inline(always)]
    #[must_use]
    pub fn bytetrans(&mut self) -> BYTETRANS_W<INT_EN_SPEC> {
        BYTETRANS_W::new(self, 7)
    }
    #[doc = "Bit 8 - Set to enable the Byte Receive Interrupt. Interrupts when a byte of data is received Auto-ACK will be disabled if this interrupt is enabled, that is, the software needs to ACK/NACK the received byte manually."]
    #[inline(always)]
    #[must_use]
    pub fn byterecv(&mut self) -> BYTERECV_W<INT_EN_SPEC> {
        BYTERECV_W::new(self, 8)
    }
    #[doc = "Bit 9 - Set to enable the Completion Interrupt. Master: interrupts when a transaction is issued from this master and completed without losing the bus arbitration. Slave: interrupts when a transaction addressing the controller is completed."]
    #[inline(always)]
    #[must_use]
    pub fn cmpl(&mut self) -> CMPL_W<INT_EN_SPEC> {
        CMPL_W::new(self, 9)
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
#[doc = "Interrupt Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_en::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_en::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INT_EN_SPEC;
impl crate::RegisterSpec for INT_EN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_en::R`](R) reader structure"]
impl crate::Readable for INT_EN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`int_en::W`](W) writer structure"]
impl crate::Writable for INT_EN_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IntEn to value 0"]
impl crate::Resettable for INT_EN_SPEC {
    const RESET_VALUE: u32 = 0;
}
