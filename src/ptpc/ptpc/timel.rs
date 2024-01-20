#[doc = "Register `timel` reader"]
pub type R = crate::R<TIMEL_SPEC>;
#[doc = "Register `timel` writer"]
pub type W = crate::W<TIMEL_SPEC>;
#[doc = "Field `TIMESTAMP_LOW` reader - No description avaiable"]
pub type TIMESTAMP_LOW_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - No description avaiable"]
    #[inline(always)]
    pub fn timestamp_low(&self) -> TIMESTAMP_LOW_R {
        TIMESTAMP_LOW_R::new(self.bits)
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
#[doc = "timestamp low\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timel::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timel::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TIMEL_SPEC;
impl crate::RegisterSpec for TIMEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`timel::R`](R) reader structure"]
impl crate::Readable for TIMEL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`timel::W`](W) writer structure"]
impl crate::Writable for TIMEL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets timel to value 0"]
impl crate::Resettable for TIMEL_SPEC {
    const RESET_VALUE: u32 = 0;
}
