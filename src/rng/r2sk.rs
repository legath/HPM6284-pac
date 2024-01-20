#[doc = "Register `R2SK[%s]` reader"]
pub type R = crate::R<R2SK_SPEC>;
#[doc = "Register `R2SK[%s]` writer"]
pub type W = crate::W<R2SK_SPEC>;
#[doc = "Field `FO2S0` reader - FIFO out to KMAN, will be SDP engine key."]
pub type FO2S0_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - FIFO out to KMAN, will be SDP engine key."]
    #[inline(always)]
    pub fn fo2s0(&self) -> FO2S0_R {
        FO2S0_R::new(self.bits)
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
#[doc = "no description available\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`r2sk::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`r2sk::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R2SK_SPEC;
impl crate::RegisterSpec for R2SK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`r2sk::R`](R) reader structure"]
impl crate::Readable for R2SK_SPEC {}
#[doc = "`write(|w| ..)` method takes [`r2sk::W`](W) writer structure"]
impl crate::Writable for R2SK_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets R2SK[%s]
to value 0"]
impl crate::Resettable for R2SK_SPEC {
    const RESET_VALUE: u32 = 0;
}
