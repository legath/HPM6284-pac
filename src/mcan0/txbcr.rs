#[doc = "Register `TXBCR` reader"]
pub type R = crate::R<TXBCR_SPEC>;
#[doc = "Register `TXBCR` writer"]
pub type W = crate::W<TXBCR_SPEC>;
#[doc = "Field `CR` reader - Cancellation Request Each Tx Buffer has its own Cancellation Request bit. Writing a ‘1’ will set the corresponding Cancellation Request bit; writing a ‘0’ has no impact. This enables the Host to set cancellation requests for multiple Tx Buffers with one write to TXBCR. TXBCR bits are set only for those Tx Buffers configured via TXBC. The bits remain set until the corresponding bit of TXBRP is reset. 0= No cancellation pending 1= Cancellation pending"]
pub type CR_R = crate::FieldReader<u32>;
#[doc = "Field `CR` writer - Cancellation Request Each Tx Buffer has its own Cancellation Request bit. Writing a ‘1’ will set the corresponding Cancellation Request bit; writing a ‘0’ has no impact. This enables the Host to set cancellation requests for multiple Tx Buffers with one write to TXBCR. TXBCR bits are set only for those Tx Buffers configured via TXBC. The bits remain set until the corresponding bit of TXBRP is reset. 0= No cancellation pending 1= Cancellation pending"]
pub type CR_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Cancellation Request Each Tx Buffer has its own Cancellation Request bit. Writing a ‘1’ will set the corresponding Cancellation Request bit; writing a ‘0’ has no impact. This enables the Host to set cancellation requests for multiple Tx Buffers with one write to TXBCR. TXBCR bits are set only for those Tx Buffers configured via TXBC. The bits remain set until the corresponding bit of TXBRP is reset. 0= No cancellation pending 1= Cancellation pending"]
    #[inline(always)]
    pub fn cr(&self) -> CR_R {
        CR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Cancellation Request Each Tx Buffer has its own Cancellation Request bit. Writing a ‘1’ will set the corresponding Cancellation Request bit; writing a ‘0’ has no impact. This enables the Host to set cancellation requests for multiple Tx Buffers with one write to TXBCR. TXBCR bits are set only for those Tx Buffers configured via TXBC. The bits remain set until the corresponding bit of TXBRP is reset. 0= No cancellation pending 1= Cancellation pending"]
    #[inline(always)]
    #[must_use]
    pub fn cr(&mut self) -> CR_W<TXBCR_SPEC> {
        CR_W::new(self, 0)
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
#[doc = "tx buffer cancellation request\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txbcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txbcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TXBCR_SPEC;
impl crate::RegisterSpec for TXBCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`txbcr::R`](R) reader structure"]
impl crate::Readable for TXBCR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`txbcr::W`](W) writer structure"]
impl crate::Writable for TXBCR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TXBCR to value 0"]
impl crate::Resettable for TXBCR_SPEC {
    const RESET_VALUE: u32 = 0;
}
