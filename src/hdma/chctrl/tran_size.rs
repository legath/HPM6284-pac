#[doc = "Register `TranSize` reader"]
pub type R = crate::R<TRAN_SIZE_SPEC>;
#[doc = "Register `TranSize` writer"]
pub type W = crate::W<TRAN_SIZE_SPEC>;
#[doc = "Field `TRANSIZE` reader - Total transfer size from source. The total number of transferred bytes is (TranSize * SrcWidth). This register is cleared when the DMA transfer is done. If a channel is enabled with zero total transfer size, the error event will be triggered and the transfer will be terminated."]
pub type TRANSIZE_R = crate::FieldReader<u32>;
#[doc = "Field `TRANSIZE` writer - Total transfer size from source. The total number of transferred bytes is (TranSize * SrcWidth). This register is cleared when the DMA transfer is done. If a channel is enabled with zero total transfer size, the error event will be triggered and the transfer will be terminated."]
pub type TRANSIZE_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Total transfer size from source. The total number of transferred bytes is (TranSize * SrcWidth). This register is cleared when the DMA transfer is done. If a channel is enabled with zero total transfer size, the error event will be triggered and the transfer will be terminated."]
    #[inline(always)]
    pub fn transize(&self) -> TRANSIZE_R {
        TRANSIZE_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Total transfer size from source. The total number of transferred bytes is (TranSize * SrcWidth). This register is cleared when the DMA transfer is done. If a channel is enabled with zero total transfer size, the error event will be triggered and the transfer will be terminated."]
    #[inline(always)]
    #[must_use]
    pub fn transize(&mut self) -> TRANSIZE_W<TRAN_SIZE_SPEC> {
        TRANSIZE_W::new(self, 0)
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
#[doc = "Channel n Transfer Size Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tran_size::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tran_size::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TRAN_SIZE_SPEC;
impl crate::RegisterSpec for TRAN_SIZE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tran_size::R`](R) reader structure"]
impl crate::Readable for TRAN_SIZE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tran_size::W`](W) writer structure"]
impl crate::Writable for TRAN_SIZE_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TranSize to value 0"]
impl crate::Resettable for TRAN_SIZE_SPEC {
    const RESET_VALUE: u32 = 0;
}
