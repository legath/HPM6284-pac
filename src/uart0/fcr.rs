#[doc = "Register `FCR` reader"]
pub type R = crate::R<FCR_SPEC>;
#[doc = "Register `FCR` writer"]
pub type W = crate::W<FCR_SPEC>;
#[doc = "Field `FIFOE` writer - FIFO enable Write 1 to enable both the transmitter and receiver FIFOs. The FIFOs are reset when the value of this bit toggles."]
pub type FIFOE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RFIFORST` writer - Receiver FIFO reset Write 1 to clear all bytes in the RXFIFO and resets its counter. The Receiver Shift Register is not cleared. This bit will automatically be cleared."]
pub type RFIFORST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TFIFORST` writer - Transmitter FIFO reset Write 1 to clear all bytes in the TXFIFO and resets its counter. The Transmitter Shift Register is not cleared. This bit will automatically be cleared."]
pub type TFIFORST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMAE` writer - DMA enable 0: Disable 1: Enable"]
pub type DMAE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TFIFOT` writer - Transmitter FIFO trigger level"]
pub type TFIFOT_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `RFIFOT` writer - Receiver FIFO trigger level"]
pub type RFIFOT_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl W {
    #[doc = "Bit 0 - FIFO enable Write 1 to enable both the transmitter and receiver FIFOs. The FIFOs are reset when the value of this bit toggles."]
    #[inline(always)]
    #[must_use]
    pub fn fifoe(&mut self) -> FIFOE_W<FCR_SPEC> {
        FIFOE_W::new(self, 0)
    }
    #[doc = "Bit 1 - Receiver FIFO reset Write 1 to clear all bytes in the RXFIFO and resets its counter. The Receiver Shift Register is not cleared. This bit will automatically be cleared."]
    #[inline(always)]
    #[must_use]
    pub fn rfiforst(&mut self) -> RFIFORST_W<FCR_SPEC> {
        RFIFORST_W::new(self, 1)
    }
    #[doc = "Bit 2 - Transmitter FIFO reset Write 1 to clear all bytes in the TXFIFO and resets its counter. The Transmitter Shift Register is not cleared. This bit will automatically be cleared."]
    #[inline(always)]
    #[must_use]
    pub fn tfiforst(&mut self) -> TFIFORST_W<FCR_SPEC> {
        TFIFORST_W::new(self, 2)
    }
    #[doc = "Bit 3 - DMA enable 0: Disable 1: Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dmae(&mut self) -> DMAE_W<FCR_SPEC> {
        DMAE_W::new(self, 3)
    }
    #[doc = "Bits 4:5 - Transmitter FIFO trigger level"]
    #[inline(always)]
    #[must_use]
    pub fn tfifot(&mut self) -> TFIFOT_W<FCR_SPEC> {
        TFIFOT_W::new(self, 4)
    }
    #[doc = "Bits 6:7 - Receiver FIFO trigger level"]
    #[inline(always)]
    #[must_use]
    pub fn rfifot(&mut self) -> RFIFOT_W<FCR_SPEC> {
        RFIFOT_W::new(self, 6)
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
#[doc = "FIFO Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FCR_SPEC;
impl crate::RegisterSpec for FCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fcr::R`](R) reader structure"]
impl crate::Readable for FCR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`fcr::W`](W) writer structure"]
impl crate::Writable for FCR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FCR to value 0"]
impl crate::Resettable for FCR_SPEC {
    const RESET_VALUE: u32 = 0;
}
