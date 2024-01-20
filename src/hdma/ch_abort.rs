#[doc = "Register `ChAbort` reader"]
pub type R = crate::R<CH_ABORT_SPEC>;
#[doc = "Register `ChAbort` writer"]
pub type W = crate::W<CH_ABORT_SPEC>;
#[doc = "Field `CHABORT` writer - Write 1 to bit n to abort channel n. The bits should only be set when the corresponding channels are enabled. Otherwise, the writes will be ignored for channels that are not enabled. (N: Number of channels)"]
pub type CHABORT_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl W {
    #[doc = "Bits 0:31 - Write 1 to bit n to abort channel n. The bits should only be set when the corresponding channels are enabled. Otherwise, the writes will be ignored for channels that are not enabled. (N: Number of channels)"]
    #[inline(always)]
    #[must_use]
    pub fn chabort(&mut self) -> CHABORT_W<CH_ABORT_SPEC> {
        CHABORT_W::new(self, 0)
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
#[doc = "Channel Abort Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch_abort::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch_abort::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CH_ABORT_SPEC;
impl crate::RegisterSpec for CH_ABORT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ch_abort::R`](R) reader structure"]
impl crate::Readable for CH_ABORT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ch_abort::W`](W) writer structure"]
impl crate::Writable for CH_ABORT_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ChAbort to value 0"]
impl crate::Resettable for CH_ABORT_SPEC {
    const RESET_VALUE: u32 = 0;
}
