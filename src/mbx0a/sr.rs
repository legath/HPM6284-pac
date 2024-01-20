#[doc = "Register `SR` reader"]
pub type R = crate::R<SR_SPEC>;
#[doc = "Register `SR` writer"]
pub type W = crate::W<SR_SPEC>;
#[doc = "Field `RWMV` reader - RX word message valid, will trigger interrupt if the related interrupt enable bit set in the control (CR) registrer. 1, the other core had written word message in the RXREG. 0, no valid word message yet in the RXREG."]
pub type RWMV_R = crate::BitReader;
#[doc = "Field `TWME` reader - TX word message empty, will trigger interrupt if the related interrupt enable bit set in the control (CR) registrer. 1, means this core had write word message to TXREG. 0, means no valid word message in the TXREG yet."]
pub type TWME_R = crate::BitReader;
#[doc = "Field `RFMF` reader - RX FIFO Message Full, message from other core; will trigger interrupt if the related interrupt enable bit set in the control (CR) registrer. 1, the other core had written 4x32 message in the RXFIFO. 0, no 4x32 RX FIFO message from other core yet."]
pub type RFMF_R = crate::BitReader;
#[doc = "Field `RFMA` reader - RX FIFO Message Available, available data in the 4x32 TX FIFO message buffer to the other core, will trigger interrupt if the related interrupt enable bit set in the control (CR) registrer. 1, no any data in the 4x32 TXFIFO message buffer. 0, there are some data in the the 4x32 TXFIFO message buffer already."]
pub type RFMA_R = crate::BitReader;
#[doc = "Field `TFME` reader - TX FIFO Message Empty, no any data in the message FIFO buffer from other core, will not trigger any interrupt.message from other core. 1, no any message data in TXFIFO from other core. 0, there are some data in the 4x32 TX FIFO from other core yet."]
pub type TFME_R = crate::BitReader;
#[doc = "Field `TFME` writer - TX FIFO Message Empty, no any data in the message FIFO buffer from other core, will not trigger any interrupt.message from other core. 1, no any message data in TXFIFO from other core. 0, there are some data in the 4x32 TX FIFO from other core yet."]
pub type TFME_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TFMA` reader - TX FIFO Message slot available, the 4x32 TX FIFO message buffer to the other core full, will not trigger any interrupt. 1, TXFIFO message buffer has slot available 0, no slot available (fifo full)"]
pub type TFMA_R = crate::BitReader;
#[doc = "Field `TFMA` writer - TX FIFO Message slot available, the 4x32 TX FIFO message buffer to the other core full, will not trigger any interrupt. 1, TXFIFO message buffer has slot available 0, no slot available (fifo full)"]
pub type TFMA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EW2RO` writer - bus Error for Write to Read Only address; this bit is W1C bit. 1, write to read only address happened in the bus of this block. 0, nothis kind of bus error; write this bit to 1 will clear this bit when this kind of error happen."]
pub type EW2RO_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EAIVA` writer - bus Error for Accessing Invalid Address; this bit is W1C bit. 1, read and write to invalid address in the bus of this block, will set this bit. 0, nothis kind of bus error; write this bit to 1 will clear this bit when this kind of error happen."]
pub type EAIVA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EWTFF` writer - bus Error for write when tx fifo full, this bit is W1C bit. 1, write to a fulled tx fifo will cause this error bit set. 0, nothis kind of bus error; write this bit to 1 will clear this bit when this kind of error happen."]
pub type EWTFF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ERRFE` writer - bus Error for read when rx fifo empty, this bit is W1C bit. 1, read from a empty rx fifo will cause this error bit set. 0, nothis kind of bus error; write this bit to 1 will clear this bit when this kind of error happen."]
pub type ERRFE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EWTRF` writer - bus Error for write when tx word message are still valid, this bit is W1C bit. 1, write to word message when the word message are still valid will cause this error bit set. 0, nothis kind of bus error; write this bit to 1 will clear this bit when this kind of error happen."]
pub type EWTRF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ERRRE` writer - bus Error for read when rx word message are still invalid, this bit is W1C bit. 1, read from word message when the word message are still invalid will cause this error bit set. 0, nothis kind of bus error; write this bit to 1 will clear this bit when this kind of error happen."]
pub type ERRRE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TFEC` reader - TX FIFO empty message word count"]
pub type TFEC_R = crate::FieldReader;
#[doc = "Field `RFVC` reader - RX FIFO valid message count"]
pub type RFVC_R = crate::FieldReader;
impl R {
    #[doc = "Bit 0 - RX word message valid, will trigger interrupt if the related interrupt enable bit set in the control (CR) registrer. 1, the other core had written word message in the RXREG. 0, no valid word message yet in the RXREG."]
    #[inline(always)]
    pub fn rwmv(&self) -> RWMV_R {
        RWMV_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TX word message empty, will trigger interrupt if the related interrupt enable bit set in the control (CR) registrer. 1, means this core had write word message to TXREG. 0, means no valid word message in the TXREG yet."]
    #[inline(always)]
    pub fn twme(&self) -> TWME_R {
        TWME_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - RX FIFO Message Full, message from other core; will trigger interrupt if the related interrupt enable bit set in the control (CR) registrer. 1, the other core had written 4x32 message in the RXFIFO. 0, no 4x32 RX FIFO message from other core yet."]
    #[inline(always)]
    pub fn rfmf(&self) -> RFMF_R {
        RFMF_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - RX FIFO Message Available, available data in the 4x32 TX FIFO message buffer to the other core, will trigger interrupt if the related interrupt enable bit set in the control (CR) registrer. 1, no any data in the 4x32 TXFIFO message buffer. 0, there are some data in the the 4x32 TXFIFO message buffer already."]
    #[inline(always)]
    pub fn rfma(&self) -> RFMA_R {
        RFMA_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - TX FIFO Message Empty, no any data in the message FIFO buffer from other core, will not trigger any interrupt.message from other core. 1, no any message data in TXFIFO from other core. 0, there are some data in the 4x32 TX FIFO from other core yet."]
    #[inline(always)]
    pub fn tfme(&self) -> TFME_R {
        TFME_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - TX FIFO Message slot available, the 4x32 TX FIFO message buffer to the other core full, will not trigger any interrupt. 1, TXFIFO message buffer has slot available 0, no slot available (fifo full)"]
    #[inline(always)]
    pub fn tfma(&self) -> TFMA_R {
        TFMA_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 16:19 - TX FIFO empty message word count"]
    #[inline(always)]
    pub fn tfec(&self) -> TFEC_R {
        TFEC_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - RX FIFO valid message count"]
    #[inline(always)]
    pub fn rfvc(&self) -> RFVC_R {
        RFVC_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 6 - TX FIFO Message Empty, no any data in the message FIFO buffer from other core, will not trigger any interrupt.message from other core. 1, no any message data in TXFIFO from other core. 0, there are some data in the 4x32 TX FIFO from other core yet."]
    #[inline(always)]
    #[must_use]
    pub fn tfme(&mut self) -> TFME_W<SR_SPEC> {
        TFME_W::new(self, 6)
    }
    #[doc = "Bit 7 - TX FIFO Message slot available, the 4x32 TX FIFO message buffer to the other core full, will not trigger any interrupt. 1, TXFIFO message buffer has slot available 0, no slot available (fifo full)"]
    #[inline(always)]
    #[must_use]
    pub fn tfma(&mut self) -> TFMA_W<SR_SPEC> {
        TFMA_W::new(self, 7)
    }
    #[doc = "Bit 8 - bus Error for Write to Read Only address; this bit is W1C bit. 1, write to read only address happened in the bus of this block. 0, nothis kind of bus error; write this bit to 1 will clear this bit when this kind of error happen."]
    #[inline(always)]
    #[must_use]
    pub fn ew2ro(&mut self) -> EW2RO_W<SR_SPEC> {
        EW2RO_W::new(self, 8)
    }
    #[doc = "Bit 9 - bus Error for Accessing Invalid Address; this bit is W1C bit. 1, read and write to invalid address in the bus of this block, will set this bit. 0, nothis kind of bus error; write this bit to 1 will clear this bit when this kind of error happen."]
    #[inline(always)]
    #[must_use]
    pub fn eaiva(&mut self) -> EAIVA_W<SR_SPEC> {
        EAIVA_W::new(self, 9)
    }
    #[doc = "Bit 10 - bus Error for write when tx fifo full, this bit is W1C bit. 1, write to a fulled tx fifo will cause this error bit set. 0, nothis kind of bus error; write this bit to 1 will clear this bit when this kind of error happen."]
    #[inline(always)]
    #[must_use]
    pub fn ewtff(&mut self) -> EWTFF_W<SR_SPEC> {
        EWTFF_W::new(self, 10)
    }
    #[doc = "Bit 11 - bus Error for read when rx fifo empty, this bit is W1C bit. 1, read from a empty rx fifo will cause this error bit set. 0, nothis kind of bus error; write this bit to 1 will clear this bit when this kind of error happen."]
    #[inline(always)]
    #[must_use]
    pub fn errfe(&mut self) -> ERRFE_W<SR_SPEC> {
        ERRFE_W::new(self, 11)
    }
    #[doc = "Bit 12 - bus Error for write when tx word message are still valid, this bit is W1C bit. 1, write to word message when the word message are still valid will cause this error bit set. 0, nothis kind of bus error; write this bit to 1 will clear this bit when this kind of error happen."]
    #[inline(always)]
    #[must_use]
    pub fn ewtrf(&mut self) -> EWTRF_W<SR_SPEC> {
        EWTRF_W::new(self, 12)
    }
    #[doc = "Bit 13 - bus Error for read when rx word message are still invalid, this bit is W1C bit. 1, read from word message when the word message are still invalid will cause this error bit set. 0, nothis kind of bus error; write this bit to 1 will clear this bit when this kind of error happen."]
    #[inline(always)]
    #[must_use]
    pub fn errre(&mut self) -> ERRRE_W<SR_SPEC> {
        ERRRE_W::new(self, 13)
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
#[doc = "Status Registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SR_SPEC;
impl crate::RegisterSpec for SR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sr::R`](R) reader structure"]
impl crate::Readable for SR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sr::W`](W) writer structure"]
impl crate::Writable for SR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SR to value 0xe2"]
impl crate::Resettable for SR_SPEC {
    const RESET_VALUE: u32 = 0xe2;
}
